//! https://wiki.osdev.org/Task_State_Segment

lazy_static! {
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stacks[0] = {
            const STACK_SIZE: usize = 5 * 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            unsafe { &STACK as *const _ as u64 + STACK_SIZE as u64}
        };
        tss
    };
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct TaskStateSegment {
    pub _reserved1: u32,
    pub stack_pointers: [u64; 3],
    pub _reserved2: u32,
    pub _reserved3: u32,
    pub interrupt_stacks: [u64; 7],
    pub _reserved4: u32,
    pub _reserved5: u32,
    pub _reserved6: u16,
    pub iomap_base: u16,
}

impl TaskStateSegment {
    pub fn new() -> TaskStateSegment {
        TaskStateSegment {
            _reserved1: 0,
            stack_pointers: [0; 3],
            _reserved2: 0,
            _reserved3: 0,
            interrupt_stacks: [0; 7],
            _reserved4: 0,
            _reserved5: 0,
            _reserved6: 0,
            iomap_base: 0,
        }
    }
}