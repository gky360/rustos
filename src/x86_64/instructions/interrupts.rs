pub fn are_enabled() -> bool {
    use crate::x86_64::registers::rflags::RFlags;

    RFlags::read().contains(RFlags::INTERRUPT_FLAG)
}

#[inline]
pub fn enable() {
    unsafe {
        asm!("sti" :::: "volatile");
    }
}

#[inline]
pub fn disable() {
    unsafe {
        asm!("cli" :::: "volatile");
    }
}

pub fn without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let saved_intpt_flag = are_enabled();
    if saved_intpt_flag {
        disable();
    }

    let ret = f();

    if saved_intpt_flag {
        enable();
    }
    ret
}

#[inline]
pub fn int3() {
    unsafe { asm!("int3" :::: "volatile", "intel") };
}
