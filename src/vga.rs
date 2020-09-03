//! This file provides VGA text mode functionality.

use crate::libstd::memset;

const BUFFER_ADDRESS: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const BUFFER_SIZE: usize = BUFFER_WIDTH * BUFFER_HEIGHT;

#[allow(dead_code)]
enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    LightMagenta = 0xD,
    LightBrown = 0xE,
    White = 0xF,
}

impl Color {
    fn color(self) -> u8 {
        self as u8
    }
}

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
            *BUFFER_ADDRESS.offset(i as isize * 2 + 1) = Color::LightGray.color();
        }
    }
}
