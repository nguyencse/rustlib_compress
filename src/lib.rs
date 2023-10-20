pub mod common;
pub mod jpeg;
pub mod output;
pub mod png;
pub mod profile;
pub mod ssim;
pub mod webp;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::os::raw::{c_char};
use std::ffi::{CStr};

use common::{ChromaSubsampling, ChromaSubsamplingOption, CompressResult, Format, Image};
use output::Output;

type LossyCompressor = Box<dyn Fn(&Image, u8, ChromaSubsampling) -> CompressResult>;
type LosslessCompressor = Box<dyn Fn(&Image) -> CompressResult>;

#[rustfmt::skip]
const QUALITY_SSIM: [f64; 101] = [
    0.64405, 0.64405, 0.493921, 0.3717685, 0.2875005, 0.226447, 0.18505, 0.155942,
    0.13402550000000002, 0.1161245, 0.10214999999999999, 0.09164900000000001, 0.0830645,
    0.0747825, 0.0686465, 0.0636275, 0.058777499999999996, 0.054973999999999995, 0.0509935,
    0.048128000000000004, 0.0452685, 0.0428175, 0.0404645, 0.0387125, 0.036169999999999994,
    0.034700999999999996, 0.03334, 0.0319895, 0.029954, 0.029339499999999998, 0.028261,
    0.0271415, 0.025916, 0.0248545, 0.0244545, 0.023451, 0.022603, 0.022269, 0.021344, 0.020581,
    0.0202495, 0.019450000000000002, 0.019161499999999998, 0.0189065, 0.018063, 0.017832,
    0.0169555, 0.016857999999999998, 0.016676, 0.0159105, 0.0157275, 0.015555,
    0.014891499999999998, 0.014727, 0.0145845, 0.013921, 0.0137565, 0.0135065, 0.012928,
    0.012669, 0.0125305, 0.011922499999999999, 0.011724, 0.011544, 0.0112675, 0.0107825,
    0.010481, 0.010245, 0.009772, 0.0095075, 0.009262, 0.008721, 0.0084715, 0.008324999999999999,
    0.007556500000000001, 0.0074540000000000006, 0.007243, 0.0067735, 0.0066254999999999994,
    0.006356499999999999, 0.005924499999999999, 0.005674500000000001, 0.005422, 0.0050215,
    0.0047565, 0.0044755, 0.0041294999999999995, 0.0038510000000000003, 0.00361, 0.003372,
    0.0029255, 0.0027010000000000003, 0.0024415, 0.002091, 0.0017955, 0.001591, 0.001218,
    0.0009805, 0.000749, 0.000548, 0.0004,
];

fn find_image(
    image: &Image,
    attr: &ssim::Calculator,
    lossy_compress: &LossyCompressor,
    target: f64,
    min_quality: u8,
    max_quality: u8,
    original_size: u64,
    chroma_subsampling: ChromaSubsampling,
) -> Result<(f64, Vec<u8>), String> {
    let mut min = min_quality;
    let mut max = max_quality;
    let mut best_buffer = Vec::new();
    let mut best_dssim = f64::INFINITY;

    // Compress image with different qualities and find which is closest to the SSIM target. Binary
    // search is used to speed up the search. Since there are 101 possible quality values, only
    // ceil(log2(101)) = 7 comparisons are needed at maximum.
    loop {
        // Overflow is not possible because `min` and `max` are in range 0-100.
        let quality = (min + max) / 2;

        let (compressed, buffer) = lossy_compress(image, quality, chroma_subsampling)?;

        for x in 0..=100 / 4 {
            if x == quality / 4 {
                eprint!("O")
            } else if x == 0 || x == 100 / 4 {
                eprint!("|");
            } else if x == min / 4 {
                eprint!("[");
            } else if x == max / 4 {
                eprint!("]");
            } else if x > min / 4 && x < max / 4 {
                eprint!("-");
            } else {
                eprint!(" ");
            }
        }

        let dssim = attr
            .compare(&compressed)
            .ok_or_else(|| "Failed to calculate SSIM image".to_string())?;

        eprintln!(
            " {:>3} quality  {:.6} SSIM  {:>3} % of original",
            quality,
            dssim,
            100 * buffer.len() as u64 / original_size,
        );

        // Last steps of the binary search are pretty close to each other, so the final step may
        // not actually have SSIM closest to the target. Instead of using the last step, keep track
        // of the best attempt so far.
        if (dssim - target).abs() < (best_dssim - target).abs() {
            best_buffer = buffer;
            best_dssim = dssim;
        }

        // Binary search step.
        if dssim > target {
            min = quality + 1;
        } else {
            // Prevent underflow because comparison is unreliable at low qualities.
            if quality == 0 {
                break;
            }
            max = quality - 1;
        }

        if min > max {
            break;
        }
    }

    Ok((best_dssim, best_buffer))
}

fn compress_image(
    image: Image,
    lossy_compress: LossyCompressor,
    lossless_compress: Option<LosslessCompressor>,
    target: f64,
    min_quality: u8,
    max_quality: u8,
    original_size: u64,
    chroma_subsampling: ChromaSubsamplingOption,
) -> Result<Vec<u8>, String> {
    let attr = ssim::Calculator::new(&image)
        .ok_or_else(|| "Failed to calculate SSIM image".to_string())?;

    let mut best_buffer = Vec::new();
    let mut best_dssim = f64::INFINITY;

    let samplings = match chroma_subsampling {
        ChromaSubsamplingOption::Auto => vec![
            ChromaSubsampling::_444,
            ChromaSubsampling::_422,
            ChromaSubsampling::_420,
        ],
        ChromaSubsamplingOption::Manual(sampling) => vec![sampling],
        ChromaSubsamplingOption::None => vec![ChromaSubsampling::_444],
    };

    for sampling in samplings {
        eprintln!("chroma subsampling: {:?}", sampling);
        let (dssim, buffer) = find_image(
            &image,
            &attr,
            &lossy_compress,
            target,
            min_quality,
            max_quality,
            original_size,
            sampling,
        )?;
        if (dssim - target).abs() < (best_dssim - target).abs() {
            best_buffer = buffer;
            best_dssim = dssim;
        }
    }

    // Try lossless compression if the format supports it. For example, lossless WebP can sometimes
    // be smaller than lossy WebP for non-photographic images.
    if let Some(compress) = lossless_compress {
        eprint!("|                        |");
        let (_, b) = compress(&image)?;
        eprintln!(
            "    lossless  0.000000 SSIM  {:>3} % of original",
            100 * b.len() as u64 / original_size
        );
        if b.len() < best_buffer.len() {
            return Ok(b);
        }
    }

    Ok(best_buffer)
}

#[no_mangle]
pub extern fn rust_fn_compress(input_path: *const c_char, output_path: *const c_char) -> bool {

    let target = QUALITY_SSIM[85];
    let min: u8 = 85;
    let max: u8 = 85;

    let c_str_in = unsafe { CStr::from_ptr(input_path) };
    let string_in = match c_str_in.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };

    let c_str_out = unsafe { CStr::from_ptr(output_path) };
    let string_out = match c_str_out.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };

    let input_file_path = PathBuf::from(string_in.to_string());
    let output_file_path = PathBuf::from(string_out.to_string());

    println!("rust_fn_compress: input -> {}, output -> {}", string_in, string_out);

    let (input_format, input_buffer) = {

        let mut reader: Box<dyn std::io::Read> = Box::new(File::open(input_file_path.clone())
            .map_err(|err| format!("failed to open input file: {}", err)).unwrap());

        // Read enough data to determine input file format by magic number.
        let mut buf = vec![0; 16];
        reader
            .read_exact(&mut buf)
            .map_err(|err| format!("failed to read magic number: {}", err)).unwrap();
        let fmt = Format::from_magic(&buf)
            .ok_or_else(|| "unknown input format, expected jpeg, png or webp".to_string()).unwrap();
        // Read rest of the input.
        reader
            .read_to_end(&mut buf)
            .map_err(|err| format!("failed to read input: {}", err)).unwrap();

        (fmt, buf)
    };

    let (output_format, output_writer) = {
        let format = Format::from_path(output_file_path.clone()).ok_or_else(|| {
            "failed to determine output format: either use a known file extension (jpeg, png or webp) or specify the format using `--output-format`".to_string()
        }).unwrap();

        let output = Output::write_file(output_file_path)
            .map_err(|err| format!("failed to open output file: {}", err)).unwrap();
        (format, output)
    };

    let chroma_subsampling = if output_format.supports_chroma_subsampling() {
        match "auto" {
            "420" => ChromaSubsamplingOption::Manual(ChromaSubsampling::_420),
            "422" => ChromaSubsamplingOption::Manual(ChromaSubsampling::_422),
            "444" => ChromaSubsamplingOption::Manual(ChromaSubsampling::_444),
            "auto" => ChromaSubsamplingOption::Auto,
            _ => unreachable!(),
        }
    } else {
        ChromaSubsamplingOption::None
    };

    let original_size = input_buffer.len();

    let input_image = match input_format {
        Format::JPEG => jpeg::read(&input_buffer),
        Format::PNG => png::read(&input_buffer),
        Format::WEBP => webp::read(&input_buffer),
    }
    .map_err(|err| format!("failed to read input: {}", err)).unwrap();

    let (lossy_compress, lossless_compress): (LossyCompressor, Option<LosslessCompressor>) =
        match output_format {
            Format::JPEG => (Box::new(jpeg::compress), None),
            Format::PNG => (Box::new(|img, q, _cs| png::compress(img, q)), None),
            Format::WEBP => (
                Box::new(|img, q, _cs| webp::compress(img, q, false)),
                Some(Box::new(|img| webp::compress(img, 100, true))),
            ),
        };

    match compress_image(
        input_image,
        lossy_compress,
        lossless_compress,
        target,
        min,
        max,
        original_size as u64,
        chroma_subsampling,
    ) {
        Ok(output_buffer) => {
            if output_buffer.len() <= original_size as usize {
                output_writer
                    .write(&output_buffer)
                    .map_err(|err| format!("failed to write output: {}", err)).unwrap();
                true
            } else {
                eprintln!("warning: Output would be larger than input, copying input to output...");
                output_writer
                    .write(&output_buffer)
                    .map_err(|err| format!("failed to write output: {}", err)).unwrap();
                true
            }
        }
        Err(_err) => {
            false
        },
    }
}
