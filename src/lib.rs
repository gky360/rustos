#![no_std]
#![cfg_attr(test, no_main)]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop_hlt()
}

use core::marker::PhantomData;

struct Port {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop_hlt()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn loop_hlt() -> ! {
    loop {
        hlt();
    }
}

fn hlt() {
    unsafe {
        asm!("hlt" :::: "volatile");
    }
}
