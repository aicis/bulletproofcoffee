# Bulletproof Coffee

Java bindings for the rust [Bulletproofs](https://github.com/dalek-cryptography/bulletproofs) library.

## Building
Requires the rust toolchain and gradle.

- Build the versions of the native library you want using `cargo build --release`
- Build the jar using `gradle jar`

Note that the Java library assumes they lie in `src/resources/native/<platform>/` when running,
so be sure that if you want to run on x86 macOS the platform is `x86_64-apple-darwin`.
