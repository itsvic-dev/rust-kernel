#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

mod hal;
mod mm;
mod print;
mod syscon;
mod uart;

extern "C" fn main(hartid: usize, fdt: usize) {
    // TODO: SMP
    if hartid != 0 {
        loop {}
    }

    let fdt = unsafe { fdt::Fdt::from_ptr(fdt as *const u8).unwrap() };
    uart::new_global(&fdt);

    println!("main: hello from {}", fdt.root().model());

    hal::init();
    mm::bitmap::init(&fdt);

    syscon::init(&fdt);

    syscon::poweroff();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("--- KERNEL PANIC ---\n{}", info);
    loop {}
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".init")]
#[unsafe(naked)]
extern "C" fn _start() {
    core::arch::naked_asm!(
        "la gp, __global_pointer$",
        "la sp, _end",
        "andi sp, sp, -16",
        "tail {main}",
        main = sym main,
    )
}
