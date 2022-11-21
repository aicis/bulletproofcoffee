CROSS=cross --build --release --target
TARGET=./target/release/
DEST=../src/main/resources/native


native:
	cargo build --release

windows_x86_64:
	cross build --release --target x86_64-pc-windows-gnu
	mv target/x86_64-pc-windows-gnu/release/java_ffi.dll ${DEST}/java_ffi_x86_64.dll

linux_aarch64:
	cross build --release --target aarch64-unknown-linux-gnu
	mv target/aarch64-unknown-linux-gnu/release/libjava_ffi.so ${DEST}/libjava_ffi_aarch64.so

linux_x86_64:
	cross build --release --target x86_64-unknown-linux-gnu
	mv target/x86_64-unknown-linux-gnu/release/libjava_ffi.so ${DEST}/libjava_ffi_x86_64.so

macos_x86_64:
	cross build --release --target x86_64-apple-darwin
	mv target/x86_64-apple-darwin/release/libjava_ffi.dylib ${DEST}/libjava_ffi_x86_64.dylib

macos_aarch64:
	cross build --release --target aarch64-apple-darwin
	mv target/aarch64-apple-darwin/release/libjava_ffi.dylib ${DEST}/libjava_ffi_aarch64.dylib

.PHONY: all clean

all: windows_x86_64 linux_aarch64 linux_x86_64 macos_x86_64 macos_aarch64

clean:
	cross clean
