use core::fmt;
use font8x8::UnicodeFonts;
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
        buffer: unsafe {&mut *(0xa0000 as *mut Buffer)},
        row_pos:0,
        col_pos: 0
    });
}

pub fn init() {
    PALETTE.init();
    init_screen();
    init_mouse_cursor();
}

const TASK_BAR_HEIGHT: usize = 24;

fn init_screen() {
    let mut writer = WRITER.lock();
    writer.fill_rect(
        0,
        0,
        VGA_WIDTH - 1,
        VGA_HEIGHT - TASK_BAR_HEIGHT - 1,
        PaletteCode::DarkCyan,
    );
    writer.fill_rect(
        0,
        VGA_HEIGHT - TASK_BAR_HEIGHT,
        VGA_WIDTH - 1,
        VGA_HEIGHT - TASK_BAR_HEIGHT,
        PaletteCode::Gray,
    );
    writer.fill_rect(
        0,
        VGA_HEIGHT - TASK_BAR_HEIGHT + 1,
        VGA_WIDTH - 1,
        VGA_HEIGHT - TASK_BAR_HEIGHT + 1,
        PaletteCode::White,
    );
    writer.fill_rect(
        0,
        VGA_HEIGHT - TASK_BAR_HEIGHT + 2,
        VGA_WIDTH - 1,
        VGA_HEIGHT - 1,
        PaletteCode::Gray,
    );

    writer.fill_rect(
        3,
        VGA_HEIGHT - TASK_BAR_HEIGHT + 4,
        59,
        VGA_HEIGHT - TASK_BAR_HEIGHT + 4,
        PaletteCode::White,
    );
    writer.fill_rect(
        2,
        VGA_HEIGHT - TASK_BAR_HEIGHT + 4,
        2,
        VGA_HEIGHT - 4,
        PaletteCode::White,
    );
    writer.fill_rect(3, VGA_HEIGHT - 4, 59, VGA_HEIGHT - 4, PaletteCode::DarkGray);
    writer.fill_rect(
        59,
        VGA_HEIGHT - TASK_BAR_HEIGHT + 5,
        59,
        VGA_HEIGHT - 5,
        PaletteCode::DarkGray,
    );
    writer.fill_rect(2, VGA_HEIGHT - 3, 59, VGA_HEIGHT - 3, PaletteCode::Black);
    writer.fill_rect(
        60,
        VGA_HEIGHT - TASK_BAR_HEIGHT + 4,
        60,
        VGA_HEIGHT - 3,
        PaletteCode::Black,
    );

    writer.fill_rect(
        VGA_WIDTH - 47,
        VGA_HEIGHT - TASK_BAR_HEIGHT + 4,
        VGA_WIDTH - 4,
        VGA_HEIGHT - TASK_BAR_HEIGHT + 4,
        PaletteCode::DarkGray,
    );
    writer.fill_rect(
        VGA_WIDTH - 47,
        VGA_HEIGHT - TASK_BAR_HEIGHT + 5,
        VGA_WIDTH - 47,
        VGA_HEIGHT - 4,
        PaletteCode::DarkGray,
    );
    writer.fill_rect(
        VGA_WIDTH - 47,
        VGA_HEIGHT - 3,
        VGA_WIDTH - 4,
        VGA_HEIGHT - 3,
        PaletteCode::White,
    );
    writer.fill_rect(
        VGA_WIDTH - 3,
        VGA_HEIGHT - TASK_BAR_HEIGHT + 4,
        VGA_WIDTH - 3,
        VGA_HEIGHT - 3,
        PaletteCode::White,
    );
}

const MOUSE_CURSOR_HEIGHT: usize = 16;
const MOUSE_CURSOR_WIDTH: usize = 16;
const MOUSE_X: usize = (VGA_WIDTH - MOUSE_CURSOR_WIDTH) / 2;
const MOUSE_Y: usize = (VGA_HEIGHT - MOUSE_CURSOR_HEIGHT - TASK_BAR_HEIGHT) / 2;
const MOUSE_CURSOR_IMAGE: [[u8; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT] = [
    *b"**************..",
    *b"*OOOOOOOOOOO*...",
    *b"*OOOOOOOOOO*....",
    *b"*OOOOOOOOO*.....",
    *b"*OOOOOOOO*......",
    *b"*OOOOOOO*.......",
    *b"*OOOOOOO*.......",
    *b"*OOOOOOOO*......",
    *b"*OOOO**OOO*.....",
    *b"*OOO*..*OOO*....",
    *b"*OO*....*OOO*...",
    *b"*O*......*OOO*..",
    *b"**........*OOO*.",
    *b"*..........*OOO*",
    *b"............*OO*",
    *b".............***",
];

fn init_mouse_cursor() {
    let mut writer = WRITER.lock();
    for y in 0..MOUSE_CURSOR_HEIGHT {
        for x in 0..MOUSE_CURSOR_WIDTH {
            match MOUSE_CURSOR_IMAGE[y][x] {
                b'*' => writer.write_pixel(MOUSE_X + x, MOUSE_Y + y, PaletteCode::Black),
                b'O' => writer.write_pixel(MOUSE_X + x, MOUSE_Y + y, PaletteCode::White),
                _ => (),
            };
        }
    }
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
pub const CHAR_HEIGHT: usize = 8;
pub const CHAR_WIDTH: usize = 8;
pub const VGA_ROWS: usize = VGA_HEIGHT / CHAR_HEIGHT;
pub const VGA_COLS: usize = VGA_WIDTH / CHAR_WIDTH;

use volatile::Volatile;

#[repr(transparent)]
struct Buffer {
    pixels: [[Volatile<PaletteCode>; VGA_WIDTH]; VGA_HEIGHT],
}

pub struct Writer {
    buffer: &'static mut Buffer,
    row_pos: usize,
    col_pos: usize,
}

impl Writer {
    pub fn write_pixel(&mut self, x: usize, y: usize, code: PaletteCode) {
        self.buffer.pixels[y][x].write(code);
    }

    pub fn fill_rect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, code: PaletteCode) {
        for y in y0..=y1 {
            for x in x0..=x1 {
                self.write_pixel(x, y, code);
            }
        }
    }

    pub fn clear_screen(&mut self, code: PaletteCode) {
        self.fill_rect(0, 0, VGA_WIDTH - 1, VGA_HEIGHT - 1, code);
        self.row_pos = 0;
        self.col_pos = 0;
    }

    pub fn write_char(&mut self, c: char, code: PaletteCode) {
        match c {
            '\n' => self.newline(),
            _ => {
                let rendered = match font8x8::BASIC_FONTS.get(c) {
                    Some(rendered) => rendered,
                    None => return self.write_unprintable_char(code),
                };
                for (y, byte) in rendered.iter().enumerate() {
                    for x in 0..8 {
                        if ((*byte) >> x) & 1 == 0 {
                            continue;
                        }
                        self.write_pixel(
                            self.col_pos * CHAR_WIDTH + x,
                            self.row_pos * CHAR_HEIGHT + y,
                            code,
                        );
                    }
                }

                self.col_pos += 1;
                if self.col_pos >= VGA_COLS {
                    self.newline();
                }
            }
        }
    }

    pub fn write_string(&mut self, s: &str, code: PaletteCode) {
        for c in s.chars() {
            self.write_char(c, code);
        }
    }

    pub fn write_unprintable_char(&mut self, code: PaletteCode) {
        let x0 = self.col_pos * CHAR_WIDTH;
        let y0 = self.row_pos * CHAR_HEIGHT;
        let x1 = x0 + CHAR_WIDTH - 1;
        let y1 = y0 + CHAR_HEIGHT - 1;
        self.fill_rect(x0, y0, x1, y1, code);
    }

    pub fn newline(&mut self) {
        // TODO: clear row when all rows are already used
        self.row_pos = (self.row_pos + 1) % VGA_ROWS;
        self.col_pos = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s, PaletteCode::White);
        Ok(())
    }
}

#[doc(hidden)]
/// Prints the given formatted string to the screen
/// through the global `WRITER` instance.
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;

    interrupts::without_interrupts(|| {
        WRITER
            .lock()
            .write_fmt(args)
            .expect("Printing to vga failed");
    });
}

/// Prints to the screen through the vga interface.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga::_print(format_args!($($arg)*));
    };
}

/// Prints to the screen through the vga interface, appending a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(
        concat!($fmt, "\n"), $($arg)*));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{eprint, eprintln};

    #[test_case]
    fn test_write_pixel() {
        eprint!("test_write_pixel... ");
        WRITER.lock().write_pixel(3, 5, PaletteCode::White);
        eprintln!("[ok]");
    }

    #[test_case]
    fn test_clear_screen() {
        eprint!("test_clear_screen... ");
        interrupts::without_interrupts(|| {
            let mut writer = WRITER.lock();
            writer.clear_screen(PaletteCode::White);
            assert_eq!(writer.row_pos, 0);
            assert_eq!(writer.col_pos, 0);
        });
        eprintln!("[ok]");
    }

    #[test_case]
    fn test_fill_output() {
        eprint!("test_fill_output... ");

        interrupts::without_interrupts(|| {
            let mut writer = WRITER.lock();
            writer.clear_screen(PaletteCode::White);
            writer.fill_rect(5, 10, 15, 20, PaletteCode::DarkGray);
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

        eprintln!("[ok]");
    }

    #[test_case]
    fn test_println_simple() {
        eprint!("test_println... ");
        println!("test_println_simple output");
        eprintln!("[ok]");
    }

    #[test_case]
    fn test_println_many() {
        eprint!("test_println_many... ");
        for _ in 0..200 {
            println!("test_println_many output");
        }
        eprintln!("[ok]");
    }

    #[test_case]
    fn test_println_output() {
        use crate::x86_64::instructions::interrupts;
        use core::fmt::Write;

        eprint!("test_println_output... ");

        let s = "Single line test string";
        interrupts::without_interrupts(|| {
            let mut writer = WRITER.lock();
            writer.clear_screen(PaletteCode::Blue);

            writeln!(writer, "\n{}", s).expect("writeln failed");
            for (i, c) in s.chars().enumerate() {
                let rendered = font8x8::BASIC_FONTS.get(c).unwrap();
                for (y, byte) in rendered.iter().enumerate() {
                    for x in 0..8 {
                        let pixel =
                            writer.buffer.pixels[CHAR_HEIGHT + y][CHAR_WIDTH * i + x].read();
                        let expected = if ((*byte) >> x) & 1 == 0 {
                            PaletteCode::Blue
                        } else {
                            PaletteCode::White
                        };
                        assert_eq!(pixel, expected);
                    }
                }
            }
        });

        eprintln!("[ok]");
    }
}
