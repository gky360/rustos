use core::marker::PhantomData;

use crate::x86_64::VirtAddr;

// #[allow(missing_debug_implementations)]
#[derive(Clone)]
#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    pub divide_error: Entry<HandlerFunc>,
    pub debug: Entry<HandlerFunc>,
    pub non_maskable_interrupt: Entry<HandlerFunc>,
    pub breakpoint: Entry<HandlerFunc>,
    pub overflow: Entry<HandlerFunc>,
    pub bound_range_exceeded: Entry<HandlerFunc>,
    pub invalid_opcode: Entry<HandlerFunc>,
    pub device_not_available: Entry<HandlerFunc>,
    pub double_fault: Entry<DivergingHandlerFuncWithErrCode>,
    coprocessor_segment_overrun: Entry<HandlerFunc>,
    pub invalid_tss: Entry<HandlerFuncWithErrCode>,
    pub segment_not_present: Entry<HandlerFuncWithErrCode>,
    pub stack_segment_fault: Entry<HandlerFuncWithErrCode>,
    pub general_protection_fault: Entry<HandlerFuncWithErrCode>,
    pub page_fault: Entry<PageFaultHandlerFunc>,
    reserved_1: Entry<HandlerFunc>,
    pub x87_floating_point: Entry<HandlerFunc>,
    pub alignment_check: Entry<HandlerFuncWithErrCode>,
    pub machine_check: Entry<DivergingHandlerFunc>,
    pub simd_floating_point: Entry<HandlerFunc>,
    pub virtualization: Entry<HandlerFunc>,
    reserved_2: [Entry<HandlerFunc>; 9],
    pub security_exception: Entry<HandlerFuncWithErrCode>,
    reserved_3: Entry<HandlerFunc>,

    interrupts: [Entry<HandlerFunc>; 256 - 32],
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Entry<F> {
    pointer_low: u16,
    gdt_selector: u16,
    options: EntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
    phantom: PhantomData<F>,
}

pub type HandlerFunc = extern "x86-interrupt" fn(&mut InterruptStackFrame);
pub type HandlerFuncWithErrCode =
    extern "x86-interrupt" fn(&mut InterruptStackFrame, error_code: u64);
pub type PageFaultHandlerFunc =
    extern "x86-interrupt" fn(&mut InterruptStackFrame, error_code: u64); // TODO: impl and use PageFaultErrorCode
pub type DivergingHandlerFunc = extern "x86-interrupt" fn(&mut InterruptStackFrame) -> !;
pub type DivergingHandlerFuncWithErrCode =
    extern "x86-interrupt" fn(&mut InterruptStackFrame, error_code: u64) -> !;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntryOptions(u16);

#[repr(C)]
pub struct InterruptStackFrame {
    value: InterruptStackFrameValue,
}

#[derive(Clone)]
#[repr(C)]
pub struct InterruptStackFrameValue {
    pub instruction_pointer: VirtAddr,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: VirtAddr,
    pub stack_segment: u64,
}
