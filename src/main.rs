#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr::write_volatile};

mod hal;
mod print;
mod uart;

unsafe extern "C" {
    static _end: u8;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".init")]
#[unsafe(naked)]
extern "C" fn _start() {
    core::arch::naked_asm!(
        "la sp, {stack}",
        "andi sp, sp, -16",
        "j {main}",
        stack = sym _end,
        main = sym main,
    )
}

extern "C" fn main(hartid: usize, fdt: usize) {
    // TODO: init from DT
    if hartid != 0 {
        loop {}
    }

    let fdt = unsafe { fdt::Fdt::from_ptr(fdt as *const u8).unwrap() };
    // find UART node in FDT and init UART
    if let Some(uart_node) = fdt.find_compatible(&["ns16550a"]) {
        uart::new_global(
            uart_node
                .reg()
                .unwrap()
                .nth(0)
                .unwrap()
                .starting_address
                .cast_mut(),
        );
    }

    println!("hello from {}", fdt.root().model());

    // syscon shutdown
    unsafe {
        write_volatile(0x100000 as *mut u32, 0x5555);
    }
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
