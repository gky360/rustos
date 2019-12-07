use core::marker::PhantomData;

pub use crate::x86_64::structures::port::PortWrite;

impl PortWrite for u8 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u8) {
        asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
    }
}

impl PortWrite for u32 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u32) {
        asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile");
    }
}

/// An I/O port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port<T: PortWrite> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: PortWrite> Port<T> {
    /// Creates an I/O port with the given port number.
    pub const fn new(port: u16) -> Port<T> {
        Port {
            port: port,
            phantom: PhantomData,
        }
    }

    /// Writes to the port.
    ///
    /// This function is unsafe because the I/O port could have side effects that violate memory
    /// safety.
    #[inline]
    pub unsafe fn write(&mut self, value: T) {
        T::write_to_port(self.port, value)
    }
}
