pub use crate::x86_64::structures::DescriptorTablePointer;

#[inline]
pub unsafe fn lidt(idt: &DescriptorTablePointer) {
    llvm_asm!("lidt [$0]" :: "r"(idt) : "memory" : "intel");
}
