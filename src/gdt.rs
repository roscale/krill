//! https://wiki.osdev.org/GDT
//! https://wiki.osdev.org/GDT_Tutorial

use core::mem::{size_of, transmute};

use crate::inline_asm::{lgdt, ltr};
use crate::tss::{TaskStateSegment, TSS};

lazy_static! {
    pub static ref GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct GlobalDescriptorTable {
    pub null_descriptor: Descriptor,
    pub code_segment: Descriptor,
    pub data_segment: Descriptor,
    pub user_code_segment: Descriptor,
    pub user_data_segment: Descriptor,
    pub tss_segment: Descriptor,
}

impl GlobalDescriptorTable {
    pub fn new() -> GlobalDescriptorTable {
        GlobalDescriptorTable {
            null_descriptor: Descriptor::new(0, 0, 0, 0),
            code_segment: Descriptor::new(0, 0xFFFFF, 0x9A, 0xC),
            data_segment: Descriptor::new(0, 0xFFFFF, 0x92, 0xC),
            user_code_segment: Descriptor::new(0, 0xFFFFF, 0xFA, 0xC),
            user_data_segment: Descriptor::new(0, 0xFFFFF, 0xF2, 0xC),
            tss_segment: Descriptor::new(
                unsafe { transmute(&*TSS) },
                (size_of::<TaskStateSegment>()) as u32,
                0x89,
                0x40,
            ),
        }
    }

    pub fn load(&'static self) {
        #[repr(C, packed)]
        #[derive(Debug, Copy, Clone)]
        struct GDTPointer {
            pub size: u16,
            pub address: u32,
        }
        lgdt(&GDTPointer {
            size: (size_of::<GlobalDescriptorTable>() - 1) as u16,
            address: self as *const _ as u32,
        });

        extern "C" {
            fn reload_segments();
        }
        unsafe { reload_segments(); }

        // Load the TSS
        let tss_segment_selector = (5 << 3) | 3; // or 3 to set RPL to ring 3
        ltr(tss_segment_selector);
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Descriptor {
    pub limit_lo: u16,
    pub base_lo: u16,
    pub base_mi: u8,
    pub access: u8,
    pub flags_and_limit_hi: u8,
    pub base_hi: u8,
}

impl Descriptor {
    fn new(base: u32, limit: u32, access: u8, flags: u8) -> Descriptor {
        if limit > 0xFFFFF {
            panic!("Invalid size {}. Must be a 20 bit number", limit);
        }
        Descriptor {
            limit_lo: limit as u16,
            base_lo: base as u16,
            base_mi: (base >> 16) as u8,
            access,
            flags_and_limit_hi: (flags << 4) | (limit >> 16) as u8,
            base_hi: (base >> 24) as u8,
        }
    }
}
