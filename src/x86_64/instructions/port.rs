use core::marker::PhantomData;

pub use crate::x86_64::structures::port::PortWrite;

impl PortWrite for u8 {
    #[inline]
    #[allow(clippy::missing_safety_doc)]
    unsafe fn write_to_port(port: u16, value: u8) {
        llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
    }
}

impl PortWrite for u32 {
    #[inline]
    #[allow(clippy::missing_safety_doc)]
    unsafe fn write_to_port(port: u16, value: u32) {
        llvm_asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile");
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port<T: PortWrite> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: PortWrite> Port<T> {
    pub const fn new(port: u16) -> Port<T> {
        Port {
            port,
            phantom: PhantomData,
        }
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn write(&mut self, value: T) {
        T::write_to_port(self.port, value)
    }
}
