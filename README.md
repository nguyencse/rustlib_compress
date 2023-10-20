# rustlib_compress
Image compress using Rust

# export bundle for iOS
`cargo install cargo-lipo`
then
`cargo lipo --release`
to generate a bundle named `librustylib.a` for iOS and use this bundle to integrate with your apps. </br>
The above command is just for building the iOS platform. So, use another command to adapt the specific platforms. (to be continued to update). </br>
It can be used for all platforms, like mobile (Android, iOS, Flutter, React Native), web apps, desktop apps, and even terminal CLI.

Let's contribute, folks.
