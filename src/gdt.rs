//! https://wiki.osdev.org/GDT
//! https://wiki.osdev.org/GDT_Tutorial

use core::mem::size_of;

use crate::inline_asm::{lgdt, ltr, reload_cs};
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
    pub tss_segment: TSSDescriptor,
}

impl GlobalDescriptorTable {
    pub fn new() -> GlobalDescriptorTable {
        GlobalDescriptorTable {
            null_descriptor: Descriptor::new(0, 0, 0, 0),
            code_segment: Descriptor::new(0, 0xFFFFF, 0x9A, 0xA),
            data_segment: Descriptor::new(0, 0xFFFFF, 0x92, 0xA),
            tss_segment: TSSDescriptor::new(
                &*TSS as *const _ as u64,
                (size_of::<TaskStateSegment>() - 1) as u32,
                0x89,
                0,
            ),
        }
    }

    pub fn load(&'static self) {
        #[repr(C, packed)]
        #[derive(Debug, Copy, Clone)]
        struct GDTPointer {
            pub size: u16,
            pub address: u64,
        }
        lgdt(&GDTPointer {
            size: (size_of::<GlobalDescriptorTable>() - 1) as u16,
            address: &self.null_descriptor as *const _ as u64,
        });
        // Reload the CS
        let code_segment_selector = 1 << 3;
        reload_cs(code_segment_selector);
        // Load the TSS
        let tss_segment_selector = 3 << 3;
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

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct TSSDescriptor {
    pub limit_lo: u16,
    pub base_lo_32: u16,
    pub base_mi_32: u8,
    pub access: u8,
    pub flags_and_limit_hi: u8,
    pub base_hi_32: u8,
    pub base_hi_64: u64,
}

impl TSSDescriptor {
    pub(crate) fn new(base: u64, limit: u32, access: u8, flags: u8) -> TSSDescriptor {
        if limit > 0xFFFFF {
            panic!("Invalid size {}. Must be a 20 bit number", limit);
        }
        TSSDescriptor {
            limit_lo: limit as u16,
            base_lo_32: base as u16,
            base_mi_32: (base >> 16) as u8,
            access,
            flags_and_limit_hi: (flags << 4) | (limit >> 16) as u8,
            base_hi_32: (base >> 24) as u8,
            base_hi_64: base >> 32,
        }
    }
}