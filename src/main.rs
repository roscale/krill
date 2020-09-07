#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(fmt_as_str)]
#![feature(format_args_nl)]
#![feature(abi_x86_interrupt)]
#![feature(stmt_expr_attributes)]
#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;

use core::fmt::Write;
use core::panic::PanicInfo;

use crate::serial::{COM1, COM2, COM3, COM4, Serial};
use crate::vga::vga_print;

mod libstd;
mod inline_asm;
mod vga;
#[macro_use]
mod serial;
mod idt;
mod gdt;
mod tss;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    Serial(COM1).init(38400);
    Serial(COM2).init(38400);
    Serial(COM3).init(38400);
    Serial(COM4).init(38400);

    gdt::GDT.load();
    idt::IDT.load();

    vga_print("Hello, World!");

    loop {}
}

/// Function called on panic
#[panic_handler]
#[allow(unused_must_use)]
fn panic(info: &PanicInfo) -> ! {
    println!();
    println!("-------------------------------------------------");
    println!("KERNEL PANIC");

    if let Some(location) = info.location() {
        println!("at {}:{}:{}",
                 location.file(),
                 location.line(),
                 location.column());
    } else {
        println!("at unknown location");
    }

    if let Some(message) = info.message() {
        println!();
        println!("{}", message);
    }

    if let Some(s) = info.payload().downcast_ref::<&str>() {
        println!();
        println!("with payload:");
        println!("{}", s);
    }
    println!("-------------------------------------------------");

    loop {}
}
