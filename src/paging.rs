use core::fmt::{self, Debug, Formatter, Display};

#[repr(align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

impl Debug for PageTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.is_present() {
                writeln!(f, "[Page entry #{}]", i)?;
                writeln!(f, "{:#?}", entry)?;
            }
        }
        Ok(())
    }
}

#[repr(transparent)]
pub struct PageTableEntry {
    value: u64,
}

impl PageTableEntry {
    pub fn is_present(&self) -> bool {
        (self.value & 1) != 0
    }

    pub fn is_writable(&self) -> bool {
        (self.value & (1 << 1)) != 0
    }

    pub fn is_user_accessible(&self) -> bool {
        (self.value & (1 << 2)) != 0
    }

    pub fn write_through_caching(&self) -> bool {
        (self.value & (1 << 3) != 0)
    }

    pub fn disable_cache(&self) -> bool {
        (self.value & (1 << 4) != 0)
    }

    pub fn is_accessed(&self) -> bool {
        (self.value & (1 << 5) != 0)
    }

    pub fn is_dirty(&self) -> bool {
        (self.value & (1 << 6) != 0)
    }

    pub fn is_huge_page(&self) -> bool {
        (self.value & (1 << 7) != 0)
    }

    pub fn is_global(&self) -> bool {
        (self.value & (1 << 8) != 0)
    }

    /// Returns a 52 bit physical address.
    pub fn physical_address(&self) -> u64 {
        (self.value & 0xFFFFFFFFFF000)
    }

    pub fn is_no_execute(&self) -> bool {
        (self.value & (1 << 63) != 0)
    }
}

impl Debug for PageTableEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Present: {}", self.is_present())?;
        writeln!(f, "Writable: {}", self.is_writable())?;
        writeln!(f, "User accessible: {}", self.is_user_accessible())?;
        writeln!(f, "Write through caching: {}", self.write_through_caching())?;
        writeln!(f, "Disable cache: {}", self.disable_cache())?;
        writeln!(f, "Accessed: {}", self.is_accessed())?;
        writeln!(f, "Dirty: {}", self.is_dirty())?;
        writeln!(f, "Huge page: {}", self.is_huge_page())?;
        writeln!(f, "Global: {}", self.is_global())?;
        writeln!(f, "Physical address: 0x{:x}", self.physical_address())?;
        writeln!(f, "No execute: {}", self.is_no_execute())
    }
}
