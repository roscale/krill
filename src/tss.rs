//! https://wiki.osdev.org/Task_State_Segment

use core::mem::size_of;
lazy_static! {
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.esp0 = {
            const STACK_SIZE: usize = 5 * 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            unsafe { &STACK as *const _ as u32 + STACK_SIZE as u32}
        };
        tss
    };
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct TaskStateSegment {
    pub _unused1: u32,
    pub esp0: u32,
    pub ss0: u32,
    pub _unused2: [u32; 22],
    pub iopb: u32,
}

impl TaskStateSegment {
    pub fn new() -> TaskStateSegment {
        TaskStateSegment {
            _unused1: 0,
            esp0: 0, // set above
            ss0: 0x10,
            _unused2: [0; 22],
            iopb: size_of::<TaskStateSegment>() as u32,
        }
    }
}