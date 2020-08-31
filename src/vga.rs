//! This file provides VGA text mode functionality.

use crate::libstd::memset;

const BUFFER_ADDRESS: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const BUFFER_SIZE: usize = BUFFER_WIDTH * BUFFER_HEIGHT;

/// Fills the VGA buffer with null bytes.
#[inline]
pub fn vga_clear() {
    unsafe {
        memset(BUFFER_ADDRESS, 0, BUFFER_SIZE);
    }
}

/// Prints a string on the first line.
pub fn vga_print(s: &str) {
    vga_clear(); // Clear leftover characters from previous prints.
    for (i, char) in s.bytes().enumerate() {
        unsafe {
            *BUFFER_ADDRESS.offset(i as isize * 2) = char;
            *BUFFER_ADDRESS.offset(i as isize * 2 + 1) = 0xF;
        }
    }
}
