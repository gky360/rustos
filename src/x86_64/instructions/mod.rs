pub mod interrupts;
pub mod port;

/// Halts the CPU until the next interrupt arrives.
#[inline]
pub fn hlt() {
    unsafe {
        asm!("hlt" :::: "volatile");
    }
}
