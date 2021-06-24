use crate::util;
use crate::util::BitOperations;

#[derive(Debug)]
#[repr(transparent)]
pub struct PageDirectory {
    pub entries: [PageDirectoryEntry; 1024],
}

impl PageDirectory {
    pub fn new() -> Self {
        Self {
            entries: [PageDirectoryEntry::new(); 1024],
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct PageDirectoryEntry {
    value: u32,
}

impl PageDirectoryEntry {
    pub fn new() -> Self {
        Self {
            value: 0,
        }
    }

    pub fn is_present(&self) -> bool {
        self.value.get_bit(0)
    }

    pub fn set_present(&mut self, value: bool) {
        self.value.set_bit(0, value);
    }

    /// The kernel can always write regardless of this flag.
    pub fn is_user_writable(&self) -> bool {
        self.value.get_bit(1)
    }

    pub fn set_user_writable(&mut self, value: bool) {
        self.value.set_bit(1, value);
    }

    pub fn is_user_accessible(&self) -> bool {
        self.value.get_bit(2)
    }

    pub fn set_user_accessible(&mut self, value: bool) {
        self.value.set_bit(2, value);
    }

    pub fn is_write_through(&self) -> bool {
        self.value.get_bit(3)
    }

    pub fn set_write_through(&mut self, value: bool) {
        self.value.set_bit(3, value);
    }

    pub fn is_cache_disabled(&self) -> bool {
        self.value.get_bit(4)
    }

    pub fn set_cache_disabled(&mut self, value: bool) {
        self.value.set_bit(4, value);
    }

    pub fn has_been_accessed(&self) -> bool {
        self.value.get_bit(5)
    }

    pub fn clear_accessed(&mut self) {
        self.value.set_bit(5, false);
    }

    pub fn is_4mb_page_size(&self) -> bool {
        self.value.get_bit(7)
    }

    pub fn set_4mb_page_size(&mut self, value: bool) {
        self.value.set_bit(7, value);
    }

    pub fn get_page_table_address(&self) -> u32 {
        self.value.get_bits_unshifted(12..=31)
    }

    pub fn set_page_table_address(&mut self, address: u32) {
        if address.get_bits(0..=11) != 0 {
            panic!("Page table address {:#x} is not aligned to 4 KiB.", address);
        }
        self.value.set_bits(12..=31, address.get_bits(12..=31));
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct PageTable {
    pub entries: [PageTableEntry; 1024],
}

impl PageTable {
    pub fn new() -> Self {
        PageTable {
            entries: [PageTableEntry::new(); 1024],
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct PageTableEntry {
    value: u32,
}

impl PageTableEntry {
    pub fn new() -> Self {
        Self {
            value: 0,
        }
    }

    pub fn is_present(&self) -> bool {
        self.value.get_bit(0)
    }

    pub fn set_present(&mut self, value: bool) {
        self.value.set_bit(0, value);
    }

    pub fn is_read_write(&self) -> bool {
        self.value.get_bit(1)
    }

    pub fn set_read_write(&mut self, value: bool) {
        self.value.set_bit(1, value);
    }

    pub fn is_user_accessible(&self) -> bool {
        self.value.get_bit(2)
    }

    pub fn set_user_accessible(&mut self, value: bool) {
        self.value.set_bit(2, value);
    }

    pub fn is_write_through(&self) -> bool {
        self.value.get_bit(3)
    }

    pub fn set_write_through(&mut self, value: bool) {
        self.value.set_bit(3, value);
    }

    pub fn is_cache_disabled(&self) -> bool {
        self.value.get_bit(4)
    }

    pub fn set_cache_disabled(&mut self, value: bool) {
        self.value.set_bit(4, value);
    }

    pub fn has_been_accessed(&self) -> bool {
        self.value.get_bit(5)
    }

    pub fn clear_accessed(&mut self) {
        self.value.set_bit(5, false);
    }

    pub fn is_dirty(&self) -> bool {
        self.value.get_bit(6)
    }

    pub fn clear_dirty(&mut self) {
        self.value.set_bit(6, false);
    }

    pub fn is_global(&self) -> bool {
        self.value.get_bit(8)
    }

    pub fn set_global(&mut self, value: bool) {
        self.value.set_bit(8, value);
    }

    pub fn get_frame_address(&self) -> u32 {
        self.value.get_bits_unshifted(12..=31)
    }

    pub fn set_frame_address(&mut self, address: u32) {
        if address.get_bits(0..=11) != 0 {
            panic!("Frame address {:#x} not shifted", address);
        }
        self.value.set_bits(12..=31, address.get_bits(12..=31));
    }
}
