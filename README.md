# Krill

An x86_64 operating system written in Rust.

## Requirements
* A Linux environment.
* An internet connection.
* The `rustup` Rust toolchain installer. If you are on Arch, install the 
`rustup` package.
* The nightly toolchain of Rust. Run `rustup toolchain install nightly` to 
install the toolchain and `rustup default nightly` to set it as the default.
* The `bootimage` cargo tool. Run `cargo install bootimage` to install it.
* The `llvm-tools-preview` component. Run `rustup component add llvm-tools-preview` 
to add the component.
* The QEMU emulator. If you are on Arch, install the `qemu` package.

## Compilation
Run `cargo run` to compile and run the OS with QEMU. If you just want to compile 
the project, run `cargo build`. A bootable image will be generated at 
`target/x86_64-krill/[debug|release]/bootimage-krill.bin`. If you want to build it in 
release mode (with optimizations), use the `--release` flag.

## License
See `LICENSE`.