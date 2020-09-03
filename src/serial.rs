//! https://wiki.osdev.org/Serial_Ports

use core::fmt;
use core::fmt::Write;

use crate::io;

pub const COM1: u16 = 0x3F8;
pub const COM2: u16 = 0x2F8;
pub const COM3: u16 = 0x3E8;
pub const COM4: u16 = 0x2E8;

#[derive(Debug)]
pub struct Serial(pub u16);

impl Serial {
    pub fn init(&self, baud_rate: u32) {
        if 115200 % baud_rate != 0 {
            panic!("Invalid baud rate {}. Value must divide 115200.", baud_rate);
        }

        io::outb(self.0 + 1, 0x00); // Disable interrupts
        io::outb(self.0 + 3, 0x80); // Enable DLAB
        {
            // Set baud rate
            let baud_rate_divisor = (115200 / baud_rate) as u16;
            let baud_rate_lo = (baud_rate_divisor & 0x00FF) as u8;
            let baud_rate_hi = (baud_rate_divisor >> 8) as u8;
            io::outb(self.0 + 0, baud_rate_lo);
            io::outb(self.0 + 1, baud_rate_hi);
        }
        io::outb(self.0 + 3, 0x03); // 8 bits, no parity, one stop bit
        io::outb(self.0 + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        io::outb(self.0 + 4, 0x0B); // IRQs enabled, RTS/DSR set
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
    () => (print!("\n"));
    ($($arg:tt)*) => (
        Serial(COM1).write_fmt(format_args_nl!($($arg)*)).unwrap()
    );
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (
        Serial(COM1).write_fmt(format_args!($($arg)*)).unwrap()
    );
}

#[macro_export]
macro_rules! dbg {
    () => {
        println!("[{}:{}]", file!(), line!());
    };
    ($val:expr) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                println!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    // Trailing comma with single argument is ignored
    ($val:expr,) => { dbg!($val) };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}