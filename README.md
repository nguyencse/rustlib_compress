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

Let's contribute, folks.

## Installation

### iOS
Build from source:</br>
`cargo install cargo-lipo`
then
`cargo lipo --release`
to generate a bundle named `libcomp.a` for iOS and use this bundle to integrate with your apps. </br>
or can use latest bundle file from [GitHub releases](https://github.com/nguyencse/rustlib_compress/releases) 

### Linux & macOS

Download the latest Linux & macOS binary from [GitHub releases](https://github.com/nguyencse/rustlib_compress/releases).
These versions are built on Ubuntu 22.04.3 LTS and macOS Sonoma 14.0

After downloading the binary, run `chmod +x path-to-comp` to make it executable.</br>
Consider storing the binary somewhere on your `PATH` like `/usr/local/bin/comp`.</br>

Copy downloaded binary to local bin:
`sudo cp path-to-comp /usr/local/bin/comp`</br>
Make binary excutable:
`sudo chmod +x /usr/local/bin/comp`

## Usage

Basic usage CLI:

```sh
comp input.jpeg --output output.jpeg
```

The target quality can be set using `--quality` option:

```sh
comp input.jpeg --quality 95 --output output.jpeg
```

The target quality is a value between 0 and 100 and roughly corresponds to JPEG quality values.

For the full list of available options, run `comp --help`.
