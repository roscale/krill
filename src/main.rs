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
extern crate pc_keyboard;
extern crate spin;

use core::panic::PanicInfo;

use bootloader::BootInfo;
use bootloader::bootinfo::MemoryRegionType;

use crate::inline_asm::hlt_loop;
use crate::paging::PageTable;
use crate::pic::init_pic;
use crate::serial::{COM1, COM2, COM3, COM4, Serial};

mod libstd;
mod inline_asm;
#[macro_use]
mod vga;
#[macro_use]
mod serial;
mod idt;
mod gdt;
mod tss;
mod pic;
mod ps2;
mod paging;

#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! {
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
    vga_print!("Keyboard support: ");

    init_pic();

    fn print_tables(address: u64, virtual_offset: u64, level: u8) {
        let page_table = unsafe { ((address + virtual_offset) as *mut PageTable).as_mut() }.unwrap();
        println!("[[Page table level {} at physical address 0x{:x}]]\n", level, address);
        println!("{:?}", page_table);

        if level <= 3 {
            return;
        }

        for entry in &page_table.entries {
            if entry.is_present() && !entry.is_huge_page() {
                print_tables(entry.physical_address(), virtual_offset, level - 1);
            }
        }
    }

    // unsafe {
    //     println!("{:?}", l4_page_table);
    //     println!("{}", *(0x18000001000 as *mut u64));
    // }

    print_tables(0x1000, _boot_info.physical_memory_offset, 4);

    // println!("offset 0x{:x}", _boot_info.physical_memory_offset);

    // let a = 4;
    // println!("address of a = {:p}", &a);

    // println!("offset 0x{:x}", _boot_info.physical_memory_offset);

    // for region in _boot_info.memory_map.iter() {
    //     println!("{:#?}", region);
    //     align_down()
    // }


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
