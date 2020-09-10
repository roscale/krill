use pc_keyboard::{HandleControl, Keyboard, layouts, ScancodeSet1};
use spin::Mutex;

use crate::inline_asm::inb;

const _PS2_CMD: u16 = 0x64;
const PS2_DATA: u16 = 0x60;

lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> =
        Mutex::new(Keyboard::new(layouts::Azerty, ScancodeSet1, HandleControl::Ignore));
}

pub fn read_keyboard_scancode() -> u8 {
    inb(PS2_DATA)
}
