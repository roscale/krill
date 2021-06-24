//! https://wiki.osdev.org/IDT

use core::fmt::{Display, Formatter};
use core::fmt;
use core::mem::size_of;

use pc_keyboard::{DecodedKey, KeyCode, KeyState};

use crate::inline_asm::{get_cs, lidt, without_interrupts, hlt_loop, disable_interrupts};
use crate::pic::{PIC_LINE_KEYBOARD, PIC_LINE_TIMER, send_eoi};
use crate::ps2::read_keyboard_scancode;

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StackFrame {
    pub ip: u32,
    pub cs: u32,
    pub flags: u32,
    pub sp: u32,
    pub ds: u32,
}

impl Display for StackFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "IP: {}", self.ip)?;
        writeln!(f, "CS: {}", self.cs)?;
        writeln!(f, "FLAGS: {}", self.flags)?;
        writeln!(f, "SP: {}", self.sp)?;
        writeln!(f, "DS: {}", self.ds)?;
        Ok(())
    }
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct InterruptDescriptorTable {
    pub divide_error: Descriptor,
    pub debug: Descriptor,
    pub non_maskable_interrupt: Descriptor,
    pub breakpoint: Descriptor,
    pub overflow: Descriptor,
    pub bound_range_exceeded: Descriptor,
    pub invalid_opcode: Descriptor,
    pub device_not_available: Descriptor,
    pub double_fault: Descriptor,
    // no longer used
    pub coprocessor_segment_overrun: Descriptor,
    pub invalid_tss: Descriptor,
    pub segment_not_present: Descriptor,
    pub stack_segment_fault: Descriptor,
    pub general_protection_fault: Descriptor,
    pub page_fault: Descriptor,
    reserved_1: Descriptor,
    pub x87_floating_point: Descriptor,
    pub alignment_check: Descriptor,
    pub machine_check: Descriptor,
    pub simd_floating_point: Descriptor,
    pub virtualization: Descriptor,
    reserved_2: [Descriptor; 9],
    pub security_exception: Descriptor,
    reserved_3: Descriptor,
    // Hardware interrupts
    pic_timer: Descriptor,
    pic_keyboard: Descriptor,
    interrupts: [Descriptor; 256 - 30],
}

impl InterruptDescriptorTable {
    fn new() -> Self {
        let code_segment = get_cs();
        let mut idt = InterruptDescriptorTable {
            divide_error: Descriptor::new(divide_error_handler as u32, code_segment, 0b1000_1110),
            debug: Descriptor::new(debug_handler as u32, code_segment, 0b1000_1110),
            non_maskable_interrupt: Descriptor::new(non_maskable_interrupt_handler as u32, code_segment, 0b1000_1110),
            breakpoint: Descriptor::new(breakpoint_handler as u32, code_segment, 0b1000_1110),
            overflow: Descriptor::new(overflow_handler as u32, code_segment, 0b1000_1110),
            bound_range_exceeded: Descriptor::new(bound_range_exceeded_handler as u32, code_segment, 0b1000_1110),
            invalid_opcode: Descriptor::new(invalid_opcode_handler as u32, code_segment, 0b1000_1110),
            device_not_available: Descriptor::new(device_not_available_handler as u32, code_segment, 0b1000_1110),
            double_fault: Descriptor::new(double_fault_handler as u32, code_segment, 0b1000_1110),
            coprocessor_segment_overrun: Descriptor::new(coprocessor_segment_overrun_handler as u32, code_segment, 0b1000_1110),
            invalid_tss: Descriptor::new(invalid_tss_handler as u32, code_segment, 0b1000_1110),
            segment_not_present: Descriptor::new(segment_not_present_handler as u32, code_segment, 0b1000_1110),
            stack_segment_fault: Descriptor::new(stack_segment_fault_handler as u32, code_segment, 0b1000_1110),
            general_protection_fault: Descriptor::new(general_protection_fault_handler as u32, code_segment, 0b1000_1110),
            page_fault: Descriptor::new(page_fault_handler as u32, code_segment, 0b1000_1110),
            reserved_1: Descriptor::new(unused_handler as u32, 0, 0),
            x87_floating_point: Descriptor::new(x87_floating_point_handler as u32, code_segment, 0b1000_1110),
            alignment_check: Descriptor::new(alignment_check_handler as u32, code_segment, 0b1000_1110),
            machine_check: Descriptor::new(machine_check_handler as u32, code_segment, 0b1000_1110),
            simd_floating_point: Descriptor::new(simd_floating_point_handler as u32, code_segment, 0b1000_1110),
            virtualization: Descriptor::new(virtualization_handler as u32, code_segment, 0b1000_1110),
            reserved_2: [Descriptor::new(unused_handler as u32, 0, 0); 9],
            security_exception: Descriptor::new(security_exception_handler as u32, code_segment, 0b1000_1110),
            reserved_3: Descriptor::new(unused_handler as u32, 0, 0),
            pic_timer: Descriptor::new(pic_timer_handler as u32, code_segment, 0b1000_1110),
            pic_keyboard: Descriptor::new(pic_keyboard_handler as u32, code_segment, 0b1000_1110),
            interrupts: [Descriptor::new(system_call as u32, code_segment, 0); 256 - 30],
        };

        // Interrupt routine for system calls.
        idt.interrupts[94] = Descriptor::new(system_call as u32, code_segment, 0b1110_1110); // WHY 94?

        idt
    }

    pub fn load(&'static self) {
        #[repr(C, packed)]
        #[derive(Debug, Copy, Clone)]
        struct IDTPointer {
            pub size: u16,
            pub address: u32,
        }
        lidt(&IDTPointer {
            size: (size_of::<InterruptDescriptorTable>() - 1) as u16,
            address: self as *const _ as u32,
        });
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Descriptor {
    pub handler_address_low: u16,
    pub segment_selector: u16,
    pub zero: u8,
    pub type_and_attributes: u8,
    pub handler_address_high: u16,
}

impl Descriptor {
    fn new(handler_address: u32, segment_selector: u16, type_and_attributes: u8) -> Descriptor {
        Self {
            handler_address_low: handler_address as u16,
            segment_selector,
            zero: 0,
            type_and_attributes,
            handler_address_high: (handler_address >> 16) as u16,
        }
    }
}

extern "x86-interrupt" fn divide_error_handler(frame: StackFrame) {
    panic!("divide_error\nStack frame:\n{}", frame);
}

extern "x86-interrupt" fn debug_handler(_frame: StackFrame) {
    panic!("debug_handler");
}

extern "x86-interrupt" fn non_maskable_interrupt_handler(_frame: StackFrame) {
    panic!("non_maskable_interrupt_handler");
}

extern "x86-interrupt" fn breakpoint_handler(_frame: StackFrame) {
    println!("Breakpoint handler");
}

extern "x86-interrupt" fn overflow_handler(_frame: StackFrame) {
    panic!("overflow_handler");
}

extern "x86-interrupt" fn bound_range_exceeded_handler(_frame: StackFrame) {
    panic!("bound_range_exceeded_handler");
}

extern "x86-interrupt" fn invalid_opcode_handler(_frame: StackFrame) {
    panic!("invalid_opcode_handler");
}

extern "x86-interrupt" fn device_not_available_handler(_frame: StackFrame) {
    panic!("device_not_available_handler");
}

extern "x86-interrupt" fn double_fault_handler(_frame: StackFrame, error_code: u32) {
    panic!("double_fault_handler with error code {}", error_code);
}

extern "x86-interrupt" fn coprocessor_segment_overrun_handler(_frame: StackFrame) {
    panic!("coprocessor_segment_overrun_handler");
}

extern "x86-interrupt" fn invalid_tss_handler(_frame: StackFrame, error_code: u32) {
    panic!("invalid_tss_handler with error code {}", error_code);
}

extern "x86-interrupt" fn segment_not_present_handler(_frame: StackFrame, error_code: u32) {
    panic!("segment_not_present_handler with error code {}", error_code);
}

extern "x86-interrupt" fn stack_segment_fault_handler(_frame: StackFrame, error_code: u32) {
    panic!("stack_segment_fault_handler with error code {}", error_code);
}

extern "x86-interrupt" fn general_protection_fault_handler(_frame: StackFrame, error_code: u32) {
    panic!("general_protection_fault_handler with error code {}", error_code);
}

extern "x86-interrupt" fn page_fault_handler(frame: StackFrame, error_code: u32) {
    println!("page_fault_handler with error code {}", error_code);
    extern "C" {
        fn get_cr2() -> u32;
    }
    unsafe { println!("Faulty address: 0x{:x}", get_cr2()); }
    panic!("{}", frame);
}

extern "x86-interrupt" fn x87_floating_point_handler(_frame: StackFrame) {
    panic!("x87_floating_point")
}

extern "x86-interrupt" fn alignment_check_handler(_frame: StackFrame, error_code: u32) {
    panic!("alignment_check {}", error_code)
}

extern "x86-interrupt" fn machine_check_handler(_frame: StackFrame) {
    panic!("machine_check")
}

extern "x86-interrupt" fn simd_floating_point_handler(_frame: StackFrame) {
    panic!("simd_floating_point")
}

extern "x86-interrupt" fn virtualization_handler(_frame: StackFrame) {
    panic!("virtualization")
}

extern "x86-interrupt" fn security_exception_handler(_frame: StackFrame, error_code: u32) {
    panic!("security_exception {}", error_code)
}

extern "x86-interrupt" fn unused_handler(_frame: StackFrame) {
    panic!("unused interrupt");
}

// Hardware interrupt handlers
extern "x86-interrupt" fn pic_timer_handler(_frame: StackFrame) {
    print!(".");
    send_eoi(PIC_LINE_TIMER);
}

extern "x86-interrupt" fn pic_keyboard_handler(_frame: StackFrame) {
    let scancode = read_keyboard_scancode();
    use crate::ps2::KEYBOARD;

    without_interrupts(|| {
        let mut keyboard = KEYBOARD.lock();
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(decoded_key) = keyboard.process_keyevent(key_event.clone()) {
                if key_event.state == KeyState::Down {
                    match key_event.code {
                        KeyCode::Enter => vga_println!(),
                        KeyCode::Backspace => {},
                        KeyCode::Tab => {},
                        _ => {
                            if let DecodedKey::Unicode(char) = decoded_key {
                                vga_print!("{}", char);
                            }
                        }
                    }
                }
            }
        }
    });
    send_eoi(PIC_LINE_KEYBOARD);
}

extern "x86-interrupt" fn system_call(frame: StackFrame) {
    dbg!(frame);
}