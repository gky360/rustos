#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rustos::{eprint, eprintln, exit_qemu, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    eprintln!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    eprint!("should_fail... ");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    eprintln!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
