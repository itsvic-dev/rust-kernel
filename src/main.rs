#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr::write_volatile};

mod uart;

#[unsafe(no_mangle)]
fn _start() {
    uart::uart_init();

    println!("hello, world!");

    // syscon shutdown
    unsafe {
        write_volatile(0x100000 as *mut u32, 0x5555);
    }
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // syscon shutdown
    unsafe {
        write_volatile(0x100000 as *mut u32, 0x5555);
    }
    loop {}
}
