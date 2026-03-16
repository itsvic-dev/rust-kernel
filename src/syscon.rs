use core::ptr::write_volatile;

use crate::println;

#[derive(Clone, Copy)]
struct Syscon {
    addr: *mut u32,
    reboot_value: Option<usize>,
    poweroff_value: Option<usize>,
}

static mut SYSCON_INSTANCE: Option<Syscon> = None;

pub fn init(fdt: &fdt::Fdt) {
    if let Some(syscon_node) = fdt.find_compatible(&["syscon"]) {
        let addr = syscon_node.reg().unwrap().nth(0).unwrap().starting_address;

        let poweroff = fdt
            .find_compatible(&["syscon-poweroff"])
            .map(|n| n.property("value").unwrap().as_usize().unwrap());

        let reboot = fdt
            .find_compatible(&["syscon-reboot"])
            .map(|n| n.property("value").unwrap().as_usize().unwrap());

        unsafe {
            SYSCON_INSTANCE = Some(Syscon {
                addr: addr as *mut u32,
                reboot_value: reboot,
                poweroff_value: poweroff,
            })
        }

        println!("found syscon at {:x}", addr as usize);
    }
}

#[allow(unused)]
pub fn reboot() {
    unsafe {
        SYSCON_INSTANCE.inspect(|syscon| {
            syscon
                .reboot_value
                .inspect(|reboot| write_volatile(syscon.addr, *reboot as u32));
        });
    }
}

pub fn poweroff() {
    unsafe {
        SYSCON_INSTANCE.inspect(|syscon| {
            syscon
                .poweroff_value
                .inspect(|poweroff| write_volatile(syscon.addr, *poweroff as u32));
        });
    }
}
