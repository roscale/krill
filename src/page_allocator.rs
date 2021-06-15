use crate::frame_allocator::FrameAllocator;
use crate::util::BitOperations;
use core::mem::transmute;
use crate::paging;

const PD_ADDRESS: u32 = 0xFFFFF000;

fn map_page(physical_address: u32, virtual_address: u32, frame_allocator: &mut FrameAllocator) {
    let pd_index = virtual_address.get_bits(22..=31);
    let pt_index = virtual_address.get_bits(12..=21);
    let offset = virtual_address.get_bits(0..=11);

    let page_directory: &mut paging::PageDirectory = unsafe { transmute(PD_ADDRESS) };
}