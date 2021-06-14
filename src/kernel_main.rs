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

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    Serial(COM1).init(38400);
    Serial(COM2).init(38400);
    Serial(COM3).init(38400);
    Serial(COM4).init(38400);

    gdt::GDT.load();
    idt::IDT.load();

    let kernel_end: *const u8 = unsafe {
        extern "C" {
            // Dummy data type. The address of this variable is the beginning of free frames.
            static mut kernel_end: c_void;
        }
        transmute(&mut kernel_end)
    };

    let mut frame_allocator = FrameAllocator::new();

    // Mark kernel frames as occupied.
    for frame in (0..kernel_end as u32).step_by(4.KiB()) {
        frame_allocator.mark_occupied(frame);
    }

    let mut page_directory: &mut paging::PageDirectory = unsafe {
        transmute(frame_allocator.allocate_frame().unwrap())
    };

    let mut page_table: &mut PageTable = unsafe {
        transmute(frame_allocator.allocate_frame().unwrap())
    };

    dbg!(kernel_end);
    dbg!(page_directory as *mut _);
    dbg!(page_table as *mut _);

    let mut frame = 0;
    for page_entry in &mut page_table.page_table_entries {
        page_entry.set_present(true);
        page_entry.set_frame_address(frame);
        frame += 4.KiB();
    }

    page_directory.page_directory_entries[0].set_present(true);
    page_directory.page_directory_entries[0].set_page_table_address(page_table as *const _ as u32);

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
