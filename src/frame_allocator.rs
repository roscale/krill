use crate::libstd::memset;
use crate::util::{BitOperations, Units};

// 1024 * 1024 pages
// 8 pages per byte
const NUMBER_OF_BITS: usize = 1024 * 1024 / 8;

pub struct FrameAllocator {
    bitmap: [u8; NUMBER_OF_BITS], // a bit is 1 if the frame is free, 0 otherwise
}

impl FrameAllocator {
    pub fn new() -> Self {
        Self {
            bitmap: [0xFF; NUMBER_OF_BITS]
        }
    }

    pub fn allocate_frame(&mut self) -> Option<*mut u8> {
        let mut offset = 0;
        for byte in &mut self.bitmap {
            if *byte != 0 {
                for i in 0..8 {
                    if byte.get_bit(i) == true {
                        byte.set_bit(i, false);
                        let address = (offset + i as u32 * 4.KiB()) as *mut u8;
                        unsafe { memset(address, 0, 4.KiB()) }; // Clear the memory
                        return Some(address);
                    }
                }
            }
            offset += 8 * 4.KiB();
        }
        None
    }

    pub fn free_frame(&mut self, frame_address: u32) {
        if self.get_frame_bit(frame_address) == true {
            panic!("Frame {} already freed.", frame_address);
        }
        self.set_frame_bit(frame_address, true);
    }

    pub fn mark_occupied(&mut self, frame_address: u32) {
        if self.get_frame_bit(frame_address) == false {
            panic!("Frame {} already occupied.", frame_address);
        }
        self.set_frame_bit(frame_address, false);
    }

    fn get_frame_bit(&mut self, frame_address: u32) -> bool {
        if frame_address % 4.KiB() != 0 {
            panic!("Frame address {} not aligned to 4 KiB.", frame_address);
        }
        let frame_number = frame_address / 4.KiB();
        self.bitmap[frame_number as usize / 8].get_bit((frame_number % 8) as u8)
    }

    fn set_frame_bit(&mut self, frame_address: u32, value: bool) {
        if frame_address % 4.KiB() != 0 {
            panic!("Frame address {} not aligned to 4 KiB.", frame_address);
        }
        let frame_number = frame_address / 4.KiB();
        self.bitmap[frame_number as usize / 8].set_bit((frame_number % 8) as u8, value);
    }
}
