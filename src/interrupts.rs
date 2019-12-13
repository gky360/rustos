use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use spin;

use crate::x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{print, println};

pub fn init() {
    init_idt();
    init_pics();
    crate::x86_64::instructions::interrupts::enable();
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt
    };
}

fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

fn init_pics() {
    unsafe { PICS.lock().initialize() };
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

#[cfg(test)]
mod tests {
    use crate::{eprint, eprintln};

    #[test_case]
    fn test_breakpoint_exception() {
        eprint!("test_breakpoint_exception... ");
        // invoke a breakpoint exception
        crate::x86_64::instructions::interrupts::int3();
        eprintln!("[ok]");
    }
}
