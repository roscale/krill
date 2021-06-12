#[inline]
pub(crate) fn outb(address: u16, value: u8) {
    unsafe {
        llvm_asm!("outb $1, $0" :: "N{dx}"(address), "{al}"(value) :: "volatile");
    }
}

#[inline]
pub(crate) fn inb(address: u16) -> u8 {
    let value: u8;
    unsafe {
        llvm_asm!("inb $1, $0" : "={al}"(value) : "N{dx}"(address) :: "volatile");
    }
    value
}

#[inline]
pub(crate) fn get_cs() -> u16 {
    let mut segment;
    unsafe {
        llvm_asm!("mov %cs, $0" : "=r" (segment));
    }
    segment
}

#[inline]
pub(crate) fn lgdt<T>(ptr: &T) {
    unsafe {
        llvm_asm!("lgdt ($0)" :: "r" (ptr) : "memory");
    }
}

#[inline]
pub(crate) fn lidt<T>(ptr: &T) {
    unsafe {
        llvm_asm!("lidt ($0)" :: "r" (ptr) : "memory");
    }
}

// #[inline]
// pub(crate) fn reload_cs(segment_selector: u64) {
//     unsafe {
//         llvm_asm!("pushq $0; \
//                 leaq  1f(%rip), %rax; \
//                 pushq %rax; \
//                 lretq; \
//                 1:" :: "ri" (segment_selector) : "rax" "memory");
//     }
// }

#[inline]
pub(crate) fn ltr(segment_selector: u16) {
    unsafe {
        llvm_asm!("ltr $0" :: "r" (segment_selector));
    }
}

#[inline]
pub(crate) fn disable_interrupts() {
    unsafe {
        llvm_asm!("cli" :::: "volatile");
    }
}

#[inline]
pub(crate) fn enable_interrupts() {
    unsafe {
        llvm_asm!("sti" :::: "volatile");
    }
}

#[inline]
pub(crate) fn io_wait() {
    unsafe {
        llvm_asm!("jmp 1f; \
            1:jmp 2f; \
            2:" :::: "volatile");
    }
}

// pub(crate) fn are_interrupts_enabled() -> bool {
//     let r: u64;
//     unsafe {
//         llvm_asm!("pushfq; popq $0" : "=r"(r) :: "memory")
//     }
//     r & (1 << 9) != 0
// }

// #[inline]
// pub(crate) fn without_interrupts<F>(f: F) where F: Fn() {
//     let were_interrupts_enabled = are_interrupts_enabled();
//     if were_interrupts_enabled {
//         disable_interrupts();
//     }
//     f();
//     if were_interrupts_enabled {
//         enable_interrupts();
//     }
// }

#[inline]
pub(crate) fn hlt_loop() -> ! {
    loop {
        unsafe {
            llvm_asm!("hlt" :::: "volatile");
        }
    }
}