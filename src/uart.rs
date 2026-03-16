// hardcoded for now, needs to be read from DTB
use core::ptr::write_volatile;

static mut UART_INSTANCE: Option<UART> = None;

pub fn new_global(addr: *mut u8) {
    let writer = UART::new(addr);
    writer.init();
    unsafe {
        UART_INSTANCE = Some(writer);
    }
}

#[derive(Clone, Copy)]
pub struct UART {
    addr: *mut u8,
}

impl UART {
    fn new(addr: *mut u8) -> Self {
        Self { addr }
    }

    fn init(&self) {
        unsafe {
            write_volatile(self.addr.offset(3), 3); // 8-bit words
            write_volatile(self.addr.offset(2), 1); // enable FIFOs
            write_volatile(self.addr.offset(1), 1); // enable receiver interrupts
        }
    }

    fn putc(&self, c: u8) {
        unsafe {
            write_volatile(self.addr, c);
        }
    }

    fn puts(&self, s: &str) {
        for c in s.bytes() {
            self.putc(c);
        }
    }
}

impl core::fmt::Write for UART {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.puts(s);
        Ok(())
    }
}

pub fn print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        UART_INSTANCE.inspect(|writer| writer.clone().write_fmt(args).unwrap());
    }
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
