use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref PALETTE: Palette = Palette([
        Color(0x00, 0x00, 0x00), // 0 : Black
        Color(0xff, 0x00, 0x00), // 1 : Red
        Color(0x00, 0xff, 0x00), // 2 : Green
        Color(0xff, 0xff, 0x00), // 3 : Yellow
        Color(0x00, 0x00, 0xff), // 4 : Blue
        Color(0xff, 0x00, 0xff), // 5 : Magenta
        Color(0x00, 0xff, 0xff), // 6 : Cyan
        Color(0xff, 0xff, 0xff), // 7 : White
        Color(0xc6, 0xc6, 0xc6), // 8 : Gray
        Color(0x84, 0x00, 0x00), // 9 : DrakRed
        Color(0x00, 0x84, 0x00), // 10: DarkGreen
        Color(0x84, 0x84, 0x00), // 11: DarkYellow
        Color(0x00, 0x00, 0x84), // 12: DarkBlue
        Color(0x84, 0x00, 0x84), // 13: DarkMagenta
        Color(0x00, 0x84, 0x84), // 14: DrakCyan
        Color(0x84, 0x84, 0x84), // 15: DarkGray
    ]);

    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        buffer: unsafe {&mut *(0xa0000 as *mut Buffer)}
    });
}

pub fn init() {
    PALETTE.init();
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PaletteCode {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
    Gray = 8,
    DarkRed = 9,
    DarkGreen = 10,
    DrakYellow = 11,
    DarkBlue = 12,
    DarkMagenta = 13,
    DarkCyan = 14,
    DarkGray = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Color(u8, u8, u8);

use crate::x86_64::instructions::interrupts;
use crate::x86_64::instructions::port::Port;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette([Color; 16]);

impl Palette {
    /// set color palette
    /// ref: chap-04 p84
    fn init(&self) {
        interrupts::without_interrupts(|| {
            let mut code_port = Port::new(0x03c8);
            let mut color_port = Port::new(0x03c9);
            unsafe { code_port.write(0 as u8) };
            for color in &self.0 {
                unsafe {
                    color_port.write(color.0 >> 2);
                    color_port.write(color.1 >> 2);
                    color_port.write(color.2 >> 2);
                }
            }
        });
    }
}

pub const VGA_HEIGHT: usize = 200;
pub const VGA_WIDTH: usize = 320;

use volatile::Volatile;

#[repr(transparent)]
struct Buffer {
    pixels: [[Volatile<PaletteCode>; VGA_WIDTH]; VGA_HEIGHT],
}

pub struct Writer {
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_pixel(&mut self, x: usize, y: usize, code: PaletteCode) {
        self.buffer.pixels[y][x].write(code);
    }

    pub fn fill(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, code: PaletteCode) {
        for y in y0..=y1 {
            for x in x0..=x1 {
                self.write_pixel(x, y, code);
            }
        }
    }

    pub fn fill_all(&mut self, code: PaletteCode) {
        self.fill(0, 0, VGA_WIDTH - 1, VGA_HEIGHT - 1, code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{print, println};

    #[test_case]
    fn test_write_pixel() {
        print!("test_write_pixel... ");
        WRITER.lock().write_pixel(3, 5, PaletteCode::White);
        println!("[ok]");
    }

    #[test_case]
    fn test_fill_all() {
        print!("test_fill_all... ");
        WRITER.lock().fill_all(PaletteCode::White);
        println!("[ok]");
    }

    #[test_case]
    fn test_fill_output() {
        print!("test_fill_output... ");

        interrupts::without_interrupts(|| {
            let mut writer = WRITER.lock();
            writer.fill_all(PaletteCode::White);
            writer.fill(5, 10, 15, 20, PaletteCode::DarkGray);
            for x in 0..VGA_WIDTH {
                for y in 0..VGA_HEIGHT {
                    let expected_code = if 5 <= x && 10 <= y && x <= 15 && y <= 20 {
                        PaletteCode::DarkGray
                    } else {
                        PaletteCode::White
                    };
                    let code = writer.buffer.pixels[y][x].read();
                    assert_eq!(code, expected_code);
                }
            }
        });

        println!("[ok]");
    }
}
