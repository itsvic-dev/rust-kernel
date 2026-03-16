// hardcoded for now, needs to be read from DTB
use core::ptr::write_volatile;

const UART_ADDR: *mut u8 = 0x10000000 as *mut u8;

pub fn uart_init() {
    unsafe {
        write_volatile(UART_ADDR.offset(3), 3); // 8-bit words
        write_volatile(UART_ADDR.offset(2), 1); // enable FIFOs
        write_volatile(UART_ADDR.offset(1), 1); // enable receiver interrupts
    }
}

fn uart_putc(c: u8) {
    unsafe {
        write_volatile(UART_ADDR, c);
    }
}

pub fn uart_puts(s: &str) {
    for c in s.bytes() {
        uart_putc(c);
    }
}

pub struct Writer;
impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        uart_puts(s);
        Ok(())
    }
}

pub fn print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    Writer {}.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::uart::print(format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! println {
    () => {{
        print!("\n");
    }};

    ($($arg:tt)*) => {{
        print!("{}\n", format_args!($($arg)*));
    }}
}
