#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(fmt_as_str)]
#![feature(format_args_nl)]
#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

use crate::serial::{COM1, COM2, COM3, COM4, Serial};
use crate::vga::vga_print;

mod libstd;
mod io;
mod vga;
mod serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    Serial(COM1).init();
    Serial(COM2).init();
    Serial(COM3).init();
    Serial(COM4).init();

    println!("Hello, {} {}", "World!", 42);
    for char in "Ca fonctionne".bytes() {
        print!("{} ", char as char);
    }

    vga_print("Hello, World!");

    loop {}
}

/// Function called on panic
#[panic_handler]
#[allow(unused_must_use)]
fn panic(info: &PanicInfo) -> ! {
    println!("-------------------------------------------------");
    println!("KERNEL PANIC");

    if let Some(location) = info.location() {
        println!("in file {}, at line {}, column {}",
                 location.file(),
                 location.line(),
                 location.column());
    } else {
        println!("at unknown location");
    }

    if let Some(a) = info.message() {
        if let Some(str) = a.as_str() {
            println!("\n{}", str);
        } else {
            println!("\nTODO can't decode String message, needs heap allocation");
        }
    }

    if let Some(s) = info.payload().downcast_ref::<&str>() {
        println!("\nwith payload:");
        println!("{}", s);
    }
    println!("-------------------------------------------------");

    loop {}
}
