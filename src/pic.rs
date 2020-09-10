use crate::inline_asm::{enable_interrupts, inb, io_wait, outb};

const PIC1_CMD: u16 = 0x20;
const PIC2_CMD: u16 = 0xA0;
const PIC1_DATA: u16 = PIC1_CMD + 1;
const PIC2_DATA: u16 = PIC2_CMD + 1;
// Commands
const EOI: u8 = 0x20;
const INIT: u8 = 0x11;
const ICW4_8086: u8 = 0x01; // 8086/88 (MCS-80/85) mode

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

pub const PIC_LINE_TIMER: u8 = 32;
pub const PIC_LINE_KEYBOARD: u8 = 33;

pub fn init_pic() {
    remap(PIC1_OFFSET, PIC2_OFFSET);
    enable_interrupts();
}

pub fn send_eoi(irq: u8) {
    if irq >= 8 {
        outb(PIC2_CMD, EOI);
    }
    outb(PIC1_CMD, EOI);
}

pub fn remap(pic1_offset: u8, pic2_offset: u8) {
    // Save pic masks
    let (pic1_mask, pic2_mask) = (inb(PIC1_DATA), inb(PIC2_DATA));

    outb(PIC1_CMD, INIT);
    io_wait();
    outb(PIC2_CMD, INIT);
    io_wait();
    outb(PIC1_DATA, pic1_offset);
    io_wait();
    outb(PIC2_DATA, pic2_offset);
    io_wait();
    outb(PIC1_DATA, 4);
    io_wait();
    outb(PIC2_DATA, 2);
    io_wait();
    outb(PIC1_DATA, ICW4_8086);
    io_wait();
    outb(PIC2_DATA, ICW4_8086);
    io_wait();

    // Restore pic masks
    outb(PIC1_DATA, pic1_mask);
    outb(PIC2_DATA, pic2_mask);
}