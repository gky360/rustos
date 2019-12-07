/// Disable interrupts.
///
/// This is a wrapper around the `cli` instruction.
#[inline]
pub fn disable() {
    unsafe {
        asm!("cli" :::: "volatile");
    }
}
