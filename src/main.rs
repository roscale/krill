#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::vga::vga_print;

mod libstd;
mod vga;

/// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    vga_print("Panic");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_print("Hello, World!");
    loop {}
}