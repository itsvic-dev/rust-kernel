use crate::uart::UART_INSTANCE;

pub fn print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        UART_INSTANCE.inspect(|writer| writer.clone().write_fmt(args).unwrap());
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::print::print(format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::print!("\n");
    }};

    ($($arg:tt)*) => {{
        $crate::print!("{}\n", format_args!($($arg)*));
    }}
}
