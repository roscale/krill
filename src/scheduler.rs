use alloc::boxed::Box;
use crate::paging::PageDirectory;
use alloc::vec::Vec;
use spin::Mutex;
use core::fmt::Debug;
use crate::idt::InterruptStackFrame;
use crate::utility_functions::set_page_directory;

lazy_static! {
    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
}

pub struct Scheduler {
    pub ready_tasks: Vec<Box<Task>>,
    pub task_index: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            ready_tasks: Vec::new(),
            task_index: 0,
        }
    }
}

pub fn schedule_next_task(frame: &mut InterruptStackFrame, gpr: &mut GeneralPurposeRegisters) {
    let mut scheduler = SCHEDULER.lock();

    let task_index = scheduler.task_index;
    let mut current_task = &mut scheduler.ready_tasks[task_index];
    current_task.registers = Registers {
        cs: frame.cs,
        ss: frame.ss,
        eflags: frame.flags,
        eip: frame.ip,
        esp: frame.sp,
        ebp: gpr.ebp,
        eax: gpr.eax,
        ebx: gpr.ebx,
        ecx: gpr.ecx,
        edx: gpr.edx,
        esi: gpr.esi,
        edi: gpr.edi,
    };

    scheduler.task_index = (scheduler.task_index + 1) % scheduler.ready_tasks.len();

    let next_task = &scheduler.ready_tasks[scheduler.task_index];

    *frame = next_task.registers.get_interrupt_stack_frame();
    *gpr = next_task.registers.get_general_purpose_registers();

    unsafe { set_page_directory(&*next_task.address_space); }
}

pub struct Task {
    pub registers: Registers,
    pub address_space: Box<PageDirectory>,
}

impl Task {
    pub fn new(registers: Registers, address_space: Box<PageDirectory>) -> Self {
        Self {
            registers,
            address_space,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct Registers {
    pub cs: u32,
    pub ss: u32,
    pub eflags: u32,
    pub eip: u32,
    pub esp: u32,
    pub ebp: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub esi: u32,
    pub edi: u32,
}

impl Registers {
    pub fn get_interrupt_stack_frame(&self) -> InterruptStackFrame {
        InterruptStackFrame {
            ip: self.eip,
            cs: self.cs,
            flags: self.eflags,
            sp: self.esp,
            ss: self.ss,
        }
    }

    pub fn get_general_purpose_registers(&self) -> GeneralPurposeRegisters {
        GeneralPurposeRegisters {
            edi: self.edi,
            esi: self.esi,
            ebp: self.ebp,
            esp: self.esp,
            ebx: self.ebx,
            edx: self.edx,
            ecx: self.ecx,
            eax: self.eax,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct GeneralPurposeRegisters {
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
}
