use core::marker::PhantomData;

pub struct Port {
    port: u16,
    phantom: PhantomData<u32>,
}

impl Port {
    pub const fn new(port: u16) -> Port {
        Port {
            port,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub unsafe fn write(&mut self, value: u32) {
        asm!("outl %eax, %dx" :: "{dx}"(self.port), "{eax}"(value) :: "volatile");
    }
}
