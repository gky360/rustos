use lazy_static::lazy_static;

use crate::println;
use crate::x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
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
