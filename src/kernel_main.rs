#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(abi_x86_interrupt)]
#![feature(stmt_expr_attributes)]
#![no_std]

#[macro_use]
extern crate lazy_static;
extern crate pc_keyboard;
extern crate spin;

use core::panic::PanicInfo;

use crate::inline_asm::hlt_loop;
// use crate::pic::init_pic;
use crate::serial::{COM1, COM2, COM3, COM4, Serial};

mod libstd;
mod inline_asm;
#[macro_use]
mod vga;
#[macro_use]
mod serial;
mod gdt;
// mod idt;
mod tss;
// mod pic;
// mod ps2;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    Serial(COM1).init(38400);
    Serial(COM2).init(38400);
    Serial(COM3).init(38400);
    Serial(COM4).init(38400);

    let a = 3;
    let b = 3;
    let c = 3;
    let d = 3;

    gdt::GDT.load();
    // idt::IDT.load();

    {
        let mut vga_text_state = vga::VGA_TEXT_STATE.lock();
        vga_text_state.clear_screen();
        vga_text_state.enable_cursor();
    }
    vga_print!("Keyboard support: ");

    // init_pic();
    hlt_loop();
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
