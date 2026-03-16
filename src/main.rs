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

fn main() {
    // TODO: init from DT
    uart::new_global(0x1000_0000 as *mut u8);
    let hart_id = read_csr!(hal::MHARTID);
    println!("hello from hart {hart_id}");

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
