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

use core::ffi::c_void;
use core::mem::transmute;
use core::panic::PanicInfo;

use crate::frame_allocator::FrameAllocator;
use crate::inline_asm::{disable_interrupts, hlt_loop};
use crate::libstd::memset;
use crate::page_mapper::{map_page, create_initial_page_directory, create_page_table};
use crate::paging::PageTable;
use crate::pic::init_pic;
use crate::serial::{COM1, COM2, COM3, COM4, Serial};
use crate::util::Units;

mod libstd;
mod inline_asm;
#[macro_use]
mod vga;
#[macro_use]
mod serial;
mod gdt;
mod idt;
mod tss;
mod pic;
mod ps2;
mod paging;
mod util;
mod frame_allocator;
mod page_mapper;

// Change this when enabling higher half kernel.
const KERNEL_VIRTUAL_OFFSET: u32 = 0;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    Serial(COM1).init(38400);
    Serial(COM2).init(38400);
    Serial(COM3).init(38400);
    Serial(COM4).init(38400);

    gdt::GDT.load();
    idt::IDT.load();

    let mut page_directory = create_initial_page_directory();

    unsafe {
        extern "C" {
            fn set_page_directory(ptr: *const paging::PageDirectory);
            fn enable_paging();
        }
        set_page_directory(&*page_directory);
        enable_paging();
    }

    {
        let mut vga_text_state = vga::VGA_TEXT_STATE.lock();
        vga_text_state.clear_screen();
        vga_text_state.enable_cursor();
    }

    vga_print!("Keyboard support: ");

    init_pic();

    hlt_loop();
}

/// Function called on panic
#[panic_handler]
#[allow(unused_must_use)]
fn panic(info: &PanicInfo) -> ! {
    disable_interrupts();
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
    loop {
        hlt_loop();
    }
}
