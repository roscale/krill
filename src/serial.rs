use core::fmt;
use core::fmt::Write;

use crate::io;

pub const COM1: u16 = 0x3F8;
pub const COM2: u16 = 0x2F8;
pub const COM3: u16 = 0x3E8;
pub const COM4: u16 = 0x2E8;

pub struct Serial(pub u16);

impl Serial {
    pub fn init(&self) {
        io::outb(self.0 + 1, 0x00);
        io::outb(self.0 + 3, 0x80);
        io::outb(self.0 + 0, 0x03);
        io::outb(self.0 + 1, 0x00);
        io::outb(self.0 + 3, 0x03);
        io::outb(self.0 + 4, 0x0B);
    }
}

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            write_char(self.0, byte);
        }
        Ok(())
    }
}

#[inline]
fn write_char(com: u16, c: u8) {
    while is_transmit_empty(com) == 0 {}
    io::outb(com, c);
}

#[inline]
fn is_transmit_empty(com: u16) -> u8 {
    io::inb(com + 5) & 0x20
}

#[macro_export]
macro_rules! println {
    () => (
        $crate::print!("\n")
    );
    ($($arg:tt)*) => (
        Serial(COM1).write_fmt(format_args_nl!($($arg)*)).unwrap()
    );
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (Serial(COM1).write_fmt(format_args!($($arg)*)).unwrap())
}