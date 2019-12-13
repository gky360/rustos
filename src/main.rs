#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rustos::hlt_loop;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    rustos::init();

    #[cfg(test)]
    test_main();

    rustos::println!("Hello world");
    rustos::println!("    from rustos!");

    hlt_loop()
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use rustos::eprintln;

    eprintln!("{}", info);
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}
