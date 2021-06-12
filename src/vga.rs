//! This file provides VGA text mode functionality.

use core::fmt::Write;

use spin::Mutex;

use crate::inline_asm::{inb, outb};
use crate::libstd::memcpy;

lazy_static! {
    pub static ref VGA_TEXT_STATE: Mutex<VGATextState> = Mutex::new(VGATextState::new());
}

const BUFFER_ADDRESS: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_HEIGHT: u16 = 25;
const BUFFER_WIDTH: u16 = 80;
const BUFFER_SIZE: u16 = BUFFER_WIDTH * BUFFER_HEIGHT;

pub struct VGATextState {
    cursor_x: u16,
    cursor_y: u16,
}

impl VGATextState {
    pub const fn new() -> VGATextState {
        VGATextState {
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    fn advance_cursor(&mut self) {
        self.cursor_x += 1;
        if self.cursor_x >= BUFFER_WIDTH {
            self.newline();
        }
    }

    fn newline(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 1;
        if self.cursor_y >= BUFFER_HEIGHT {
            self.cursor_y = BUFFER_HEIGHT - 1; // Stay on the last line
            self.scroll_down_one_line();
        }
    }

    pub fn scroll_down_one_line(&self) {
        for y in 1..BUFFER_HEIGHT {
            unsafe {
                let dest = BUFFER_ADDRESS.offset(((y - 1) * 2 * BUFFER_WIDTH) as isize);
                let src = BUFFER_ADDRESS.offset((y * 2 * BUFFER_WIDTH) as isize);
                memcpy(dest, src, (2 * BUFFER_WIDTH) as usize);
            }
        }
        // Clear the last line
        let last_line = (BUFFER_HEIGHT - 1) * BUFFER_WIDTH;
        for x in 0..BUFFER_WIDTH {
            unsafe {
                *BUFFER_ADDRESS.offset(((last_line + x) * 2) as isize) = 0;
                *BUFFER_ADDRESS.offset(((last_line + x) * 2) as isize + 1) = Color::LightGray.color();
            }
        }
    }

    /// Clear the screen and reset the cursor position.
    pub fn clear_screen(&mut self) {
        for i in 0..BUFFER_SIZE {
            unsafe {
                *BUFFER_ADDRESS.offset(i as isize * 2) = 0;
                *BUFFER_ADDRESS.offset(i as isize * 2 + 1) = Color::LightGray.color();
            }
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.update_cursor_position();
    }

    fn update_cursor_position(&self) {
        let i = self.get_buffer_index_at_cursor();
        outb(0x3D4, 0x0F);
        outb(0x3D5, i as u8);
        outb(0x3D4, 0x0E);
        outb(0x3D5, (i >> 8) as u8);
    }

    fn get_buffer_index_at_cursor(&self) -> u16 {
        BUFFER_WIDTH * self.cursor_y + self.cursor_x
    }

    /// Show block cursor.
    pub fn enable_cursor(&self) {
        outb(0x3D4, 0x0A);
        outb(0x3D5, (inb(0x3D5) & 0xC0) | 0);
        outb(0x3D4, 0x0B);
        outb(0x3D5, (inb(0x3D5) & 0xE0) | 15);
    }
}

impl Write for VGATextState {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for char in s.bytes() {
            unsafe {
                if char == b'\n' {
                    self.newline();
                    continue;
                }
                let i = self.get_buffer_index_at_cursor();
                *BUFFER_ADDRESS.offset(i as isize * 2) = char;
                *BUFFER_ADDRESS.offset(i as isize * 2 + 1) = Color::LightGray.color();
                self.advance_cursor();
            }
        }
        self.update_cursor_position();
        Ok(())
    }
}

#[macro_export]
macro_rules! vga_println {
    () => (vga_print!("\n"));
    ($($arg:tt)*) => (
        {
            use core::fmt::Write;
            // use crate::inline_asm::without_interrupts;
            use crate::vga::VGA_TEXT_STATE;
            // without_interrupts(|| {
                VGA_TEXT_STATE.lock().write_fmt(format_args_nl!($($arg)*)).unwrap();
            // })
        }
    );
}

#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => (
        {
            use core::fmt::Write;
            // use crate::inline_asm::without_interrupts;
            use crate::vga::VGA_TEXT_STATE;
            // without_interrupts(|| {
                VGA_TEXT_STATE.lock().write_fmt(format_args!($($arg)*)).unwrap();
            // })
        }
    );
}

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