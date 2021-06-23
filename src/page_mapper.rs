use core::mem::transmute;

use spin::Mutex;

use crate::frame_allocator::*;
use crate::paging;
use crate::util::{BitOperations, Units};

const PD_ADDRESS: u32 = 0xFFFFF000;
lazy_static! {
    pub static ref page_directory: Mutex<&'static mut paging::PageDirectory>
        = Mutex::new(unsafe { transmute(PD_ADDRESS) });
}

pub fn create_initial_page_directory() -> &'static mut paging::PageDirectory {
    let page_directory_address = FRAME_ALLOCATOR.lock().allocate_frame().unwrap();
    let mut pd: &mut paging::PageDirectory = unsafe {
        transmute(page_directory_address)
    };

    let last_entry = pd.entries.last_mut().unwrap();
    last_entry.set_present(true);
    last_entry.set_page_table_address(page_directory_address as u32);

    let mut kernel_page_table = create_page_table();

    let mut frame = 0;
    for page_entry in &mut kernel_page_table.entries {
        page_entry.set_present(true);
        page_entry.set_frame_address(frame);
        frame += 4.KiB();
    }

    pd.entries[0].set_present(true);
    pd.entries[0].set_page_table_address(kernel_page_table as *const _ as u32);

    dbg!(kernel_page_table as *const _ as u32);

    pd
}

pub fn create_page_table() -> &'static mut paging::PageTable {
    let page_table_address = FRAME_ALLOCATOR.lock().allocate_frame().unwrap();
    let mut page_table: &mut paging::PageTable = unsafe {
        transmute(page_table_address as u32)
    };

    let last_entry = page_table.entries.last_mut().unwrap();
    last_entry.set_present(true);
    last_entry.set_frame_address(page_table_address as u32);

    page_table
}

pub fn map_page(physical_address: u32, virtual_address: u32) {
    assert_eq!(physical_address % 4.KiB(), 0);
    assert_eq!(virtual_address % 4.KiB(), 0);

    let pd_index = virtual_address.get_bits(22..=31) as usize;
    let pt_index = virtual_address.get_bits(12..=21) as usize;

    let mut pd = page_directory.lock();
    let mut pd_entry = &mut pd.entries[pd_index];
    if !pd_entry.is_present() {
        pd_entry.set_present(true);
        pd_entry.set_page_table_address(unsafe { transmute(create_page_table()) });
    }

    let mut pt: &mut paging::PageTable = unsafe { transmute(0xFFC0_0000 + 4.KiB() * pd_index) };

    let entry = &mut pt.entries[pt_index];
    entry.set_present(true);
    entry.set_frame_address(physical_address);
}