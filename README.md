# Krill

A 32-bit operating system written in Rust.

## Requirements
* A Linux environment.
* An internet connection.
* The `rustup` Rust toolchain installer. If you are on Arch, install the 
`rustup` package.
* The nightly toolchain of Rust. Run `rustup toolchain install nightly` to 
install the toolchain and `rustup default nightly` to set it as the default.
* The QEMU emulator. If you are on Arch, install the `qemu` package.

## Compilation
Run `make run` to compile and run the OS with QEMU. If you just want to compile 
the project, run `make`. An image will be generated at `./krill.bin`.

## License
See `LICENSE`.