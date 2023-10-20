# rustlib_compress
Image compress using Rust

## Features

- Optimize images automatically for the web
- Supports PNG, JPEG and WebP
- Ensure images are displayed consistently across browsers by handling ICC profiles and Exif orientation
- Powered by great projects like [mozjpeg](https://github.com/mozilla/mozjpeg) and [pngquant](https://pngquant.org/)
- Easily installable statically-linked binary (for Linux and macOS)

# export bundle for iOS
`cargo install cargo-lipo`
then
`cargo lipo --release`
to generate a bundle named `librustylib.a` for iOS and use this bundle to integrate with your apps. </br>
The above command is just for building the iOS platform. So, use another command to adapt the specific platforms. (to be continued to update). </br>
It can be used for all platforms, like mobile (Android, iOS, Flutter, React Native), web apps, desktop apps, and even terminal CLI.

Let's contribute, folks.
