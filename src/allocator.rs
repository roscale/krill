use linked_list_allocator::LockedHeap;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn init_heap(heap_bottom: usize, heap_size: usize) {
    unsafe {
        ALLOCATOR.lock().init(heap_bottom, heap_size);
    }
}