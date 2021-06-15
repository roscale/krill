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
use crate::inline_asm::hlt_loop;
use crate::libstd::memset;
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
mod page_allocator;

#[no_mangle]
pub static HIGHER_HALF_ADDRESS: u32 = 0xC0000000;

extern "C" {
    static kernel_end: c_void;
    fn set_page_directory(ptr: *const paging::PageDirectory);
    fn enable_paging();
    fn jump_to_higher_half(ptr: extern "C" fn() -> !);
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let kernel_end_addr: *const u8 = unsafe {
        transmute(&kernel_end)
    };

    let mut frame_allocator = FrameAllocator::new();

    // Mark kernel frames as occupied.
    for frame in (0..kernel_end_addr as u32).step_by(4.KiB()) {
        frame_allocator.mark_occupied(frame);
    }

    let page_directory_address = frame_allocator.allocate_frame().unwrap();
    let mut page_directory: &mut paging::PageDirectory = unsafe {
        transmute(page_directory_address)
    };

    // Set up recursive entry
    let page_directory_address: u32 = unsafe { transmute(page_directory_address) };
    let last_entry = page_directory.entries.last_mut().unwrap();
    last_entry.set_present(true);
    last_entry.set_page_table_address(page_directory_address);

    let page_table_0_address = frame_allocator.allocate_frame().unwrap();
    let mut page_table_0: &mut PageTable = unsafe {
        transmute(page_table_0_address)
    };

    // Set up recursive entry
    let page_table_address: u32 = unsafe { transmute(page_table_0_address) };
    let last_entry = page_table_0.entries.last_mut().unwrap();
    last_entry.set_present(true);
    last_entry.set_frame_address(page_table_address);

    // Identity map the first 1 MiB and the kernel
    let mut index = 0;
    for frame in (0..kernel_end_addr as u32).step_by(4.KiB()) {
        let mut entry = &mut page_table_0.entries[index];
        entry.set_present(true);
        entry.set_frame_address(frame);
        index += 1;
    }

    page_directory.entries[0].set_present(true);
    page_directory.entries[0].set_page_table_address(unsafe { transmute(page_table_0) });


    let page_table_768_address = frame_allocator.allocate_frame().unwrap();
    let mut page_table_768: &mut PageTable = unsafe {
        transmute(page_table_768_address)
    };

    // Set up recursive entry
    let page_table_address: u32 = unsafe { transmute(page_table_768_address) };
    let last_entry = page_table_768.entries.last_mut().unwrap();
    last_entry.set_present(true);
    last_entry.set_frame_address(page_table_address);

    // Higher-half kernel mapping: 3..=4 GiB
    let mut index = 1.MiB() / 4.KiB();
    for frame in (1.MiB()..kernel_end_addr as u32).step_by(4.KiB()) {
        let mut entry = &mut page_table_768.entries[index];
        entry.set_present(true);
        entry.set_frame_address(frame);
        index += 1;
    }

    page_directory.entries[768].set_present(true);
    page_directory.entries[768].set_page_table_address(unsafe { transmute(page_table_768) });

    unsafe {
        set_page_directory(&*page_directory);
        enable_paging();
        jump_to_higher_half(higher_half);
    }
    hlt_loop();
}

pub extern "C" fn higher_half() -> ! {
    Serial(COM1).init(38400);
    Serial(COM2).init(38400);
    Serial(COM3).init(38400);
    Serial(COM4).init(38400);

    gdt::GDT.load();
    idt::IDT.load();

    {
        let mut vga_text_state = vga::VGA_TEXT_STATE.lock();
        vga_text_state.clear_screen();
        vga_text_state.enable_cursor();
    }
    println!("HIGHER HALF");
    vga_println!("HIGHER HALF");

    vga_print!("Keyboard support: ");

    init_pic();

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
