use crate::x86_64::structures::gdt::SegmentSelector;

pub fn cs() -> SegmentSelector {
    let segment: u16;
    unsafe { llvm_asm!("mov $0, cs" : "=r"(segment) ::: "intel") };
    SegmentSelector(segment)
}
