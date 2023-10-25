# rustlib_compress
Image compress using Rust</br></br>
This is a fork of https://github.com/siiptuo/pio to adapt multiple platforms, not even CLI or desktop.</br>
Big thanks.

## Features

- Optimize images automatically for the app, web.
- Supports PNG, JPEG and WebP
- Ensure images are displayed consistently across browsers by handling ICC profiles and Exif orientation
- Powered by great projects like [mozjpeg](https://github.com/mozilla/mozjpeg) and [pngquant](https://pngquant.org/)
- Easily installable statically-linked binary (for Android, iOS, Flutter, React Native, web apps, desktop apps, and even terminal CLI).

# export bundle for iOS
`cargo install cargo-lipo`
then
`cargo lipo --release`
to generate a bundle named `librustylib.a` for iOS and use this bundle to integrate with your apps. </br>
The above command is just for building the iOS platform. So, use another command to adapt the specific platforms. (to be continued to update). </br>

Let's contribute, folks.

## Installation

### Linux

Download the latest Linux binary from [GitHub releases](https://github.com/nguyencse/rustlib_compress/releases).
These versions are built on Ubuntu 22.04.3 LTS.

After downloading the binary, run `chmod +x path-to-comp` to make it executable.</br>
Consider storing the binary somewhere on your `PATH` like `/usr/local/bin/comp`.

## Usage

Basic usage:

```sh
comp input.jpeg --output output.jpeg
```

The target quality can be set using `--quality` option:

```sh
comp input.jpeg --quality 95 --output output.jpeg
```

The target quality is a value between 0 and 100 and roughly corresponds to JPEG quality values.

For the full list of available options, run `comp --help`.
