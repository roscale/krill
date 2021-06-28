use core::mem::transmute;

use spin::Mutex;

use crate::*;
use crate::util::{BitOperations, Units};
use alloc::boxed::Box;
use crate::paging::PageDirectory;

const PD_ADDRESS: u32 = 0xFFFFF000;
lazy_static! {
    pub static ref GLOBAL_PAGE_DIRECTORY: Mutex<&'static mut PageDirectory>
        = Mutex::new(unsafe { transmute(PD_ADDRESS) });
}

pub fn create_initial_page_directory() -> Box<PageDirectory> {
    let mut pd = Box::new(PageDirectory::new());

    for e in &mut pd.entries {
        e.set_user_accessible(true);
        e.set_user_writable(true);
    }

    pd.entries[1023].set_present(true);
    pd.entries[1023].set_page_table_address(unsafe { transmute(&*pd) });

    let mut i = 0;
    for f in (0..4.MiB()).step_by(4.MiB()) {
        let mut kernel_page_table = create_page_table();

        for e in &mut kernel_page_table.entries {
            e.set_present(true);
            e.set_user_accessible(true);
            e.set_read_write(true);
        }

        let mut frame = f;
        for page_entry in &mut kernel_page_table.entries {
            page_entry.set_frame_address(frame);
            frame += 4.KiB();
        }

        pd.entries[i].set_present(true);
        pd.entries[i].set_page_table_address(unsafe { transmute(&*kernel_page_table) });

        i += 1;
        Box::leak(kernel_page_table);
    }

    pd
}

pub fn create_page_table() -> Box<PageTable> {
    let mut page_table = Box::new(PageTable::new());

    page_table.entries[1023].set_present(true);
    page_table.entries[1023].set_frame_address(unsafe { transmute(&*page_table) });

    page_table
}

pub fn map_page(physical_address: u32, virtual_address: u32) {
    assert_eq!(physical_address % 4.KiB(), 0);
    assert_eq!(virtual_address % 4.KiB(), 0);

    let pd_index = virtual_address.get_bits(22..=31) as usize;
    let pt_index = virtual_address.get_bits(12..=21) as usize;

    let mut pd = GLOBAL_PAGE_DIRECTORY.lock();
    let pd_entry = &mut pd.entries[pd_index];
    if !pd_entry.is_present() {
        pd_entry.set_present(true);
        pd_entry.set_page_table_address(unsafe { transmute(create_page_table()) });
    }

    let pt: &mut PageTable = unsafe { transmute(0xFFC0_0000 + 4.KiB() * pd_index) };

    let entry = &mut pt.entries[pt_index];
    entry.set_present(true);
    entry.set_frame_address(physical_address);
}