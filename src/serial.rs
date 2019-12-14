use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _eprint(args: ::core::fmt::Arguments) {
    use crate::x86_64::instructions::interrupts;
    use core::fmt::Write;

    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        $crate::serial::_eprint(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($fmt:expr) => ($crate::eprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::eprint!(
        concat!($fmt, "\n"), $($arg)*));
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn test_eprintln_simple() {
        eprint!("test_eprintln_simple... ");
        eprintln!("[ok]");
    }
}
