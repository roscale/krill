#[inline]
pub(crate) fn outb(address: u16, value: u8) {
    unsafe {
        llvm_asm!("outb $1, $0" :: "N{dx}"(address), "{al}"(value) :: "volatile");
    }
}

#[inline]
pub(crate) fn inb(address: u16) -> u8 {
    unsafe {
        let value: u8;
        llvm_asm!("inb $1, $0" : "={al}"(value) : "N{dx}"(address) :: "volatile");
        value
    }
}
