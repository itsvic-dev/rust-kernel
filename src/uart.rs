// hardcoded for now, needs to be read from DTB
use core::ptr::write_volatile;

use fdt::Fdt;

pub static mut UART_INSTANCE: Option<UART> = None;

pub fn new_global(fdt: &Fdt) {
    // find UART node in FDT and init UART
    if let Some(uart_node) = fdt.find_compatible(&["ns16550a"]) {
        let addr = uart_node
            .reg()
            .unwrap()
            .nth(0)
            .unwrap()
            .starting_address
            .cast_mut();
        let writer = UART::new(addr);
        writer.init();
        unsafe {
            UART_INSTANCE = Some(writer);
        }
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
