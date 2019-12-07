#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rustos::{exit_qemu, loop_hlt, print, println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop_hlt()
}

fn should_fail() {
    print!("should_fail... ");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop_hlt()
}
