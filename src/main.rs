#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rustos::loop_hlt;
use rustos::vga::{PaletteCode, VGA_HEIGHT, VGA_WIDTH, WRITER};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    rustos::init();
    rustos::println!("Hello world!");

    {
        use PaletteCode::*;
        let colors = [
            Black,
            Red,
            Green,
            Yellow,
            Blue,
            Magenta,
            Cyan,
            White,
            Gray,
            DarkRed,
            DarkGreen,
            DrakYellow,
            DarkBlue,
            DarkMagenta,
            DarkCyan,
            DarkGray,
        ];

        let mut vga_writer = WRITER.lock();
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                vga_writer.write_pixel(x, y, colors[x & 0xf]);
            }
        }
    }

    loop_hlt()
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use rustos::println;

    println!("{}", info);
    loop_hlt()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}
