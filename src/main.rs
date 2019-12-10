#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rustos::hlt_loop;
use rustos::vga::{PaletteCode, VGA_HEIGHT, VGA_WIDTH, WRITER};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    rustos::init();

    #[cfg(test)]
    test_main();

    {
        let mut vga = WRITER.lock();
        vga.fill_rect(0, 0, VGA_WIDTH - 1, VGA_HEIGHT - 29, PaletteCode::DarkCyan);
        vga.fill_rect(
            0,
            VGA_HEIGHT - 28,
            VGA_WIDTH - 1,
            VGA_HEIGHT - 28,
            PaletteCode::Gray,
        );
        vga.fill_rect(
            0,
            VGA_HEIGHT - 27,
            VGA_WIDTH - 1,
            VGA_HEIGHT - 27,
            PaletteCode::White,
        );
        vga.fill_rect(
            0,
            VGA_HEIGHT - 26,
            VGA_WIDTH - 1,
            VGA_HEIGHT - 1,
            PaletteCode::Gray,
        );

        vga.fill_rect(3, VGA_HEIGHT - 24, 59, VGA_HEIGHT - 24, PaletteCode::White);
        vga.fill_rect(2, VGA_HEIGHT - 24, 2, VGA_HEIGHT - 4, PaletteCode::White);
        vga.fill_rect(3, VGA_HEIGHT - 4, 59, VGA_HEIGHT - 4, PaletteCode::DarkGray);
        vga.fill_rect(
            59,
            VGA_HEIGHT - 23,
            59,
            VGA_HEIGHT - 5,
            PaletteCode::DarkGray,
        );
        vga.fill_rect(2, VGA_HEIGHT - 3, 59, VGA_HEIGHT - 3, PaletteCode::Black);
        vga.fill_rect(60, VGA_HEIGHT - 24, 60, VGA_HEIGHT - 3, PaletteCode::Black);

        vga.fill_rect(
            VGA_WIDTH - 47,
            VGA_HEIGHT - 24,
            VGA_WIDTH - 4,
            VGA_HEIGHT - 24,
            PaletteCode::DarkGray,
        );
        vga.fill_rect(
            VGA_WIDTH - 47,
            VGA_HEIGHT - 23,
            VGA_WIDTH - 47,
            VGA_HEIGHT - 4,
            PaletteCode::DarkGray,
        );
        vga.fill_rect(
            VGA_WIDTH - 47,
            VGA_HEIGHT - 3,
            VGA_WIDTH - 4,
            VGA_HEIGHT - 3,
            PaletteCode::White,
        );
        vga.fill_rect(
            VGA_WIDTH - 3,
            VGA_HEIGHT - 24,
            VGA_WIDTH - 3,
            VGA_HEIGHT - 3,
            PaletteCode::White,
        );
    }

    rustos::println!("Hello world!");

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
