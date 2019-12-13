pub mod interrupts;
pub mod port;
pub mod segmentation;
pub mod tables;

/// Halts the CPU until the next interrupt arrives.
#[inline]
pub fn hlt() {
    unsafe {
        asm!("hlt" :::: "volatile");
    }
}
