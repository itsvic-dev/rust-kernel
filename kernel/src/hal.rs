use crate::println;

pub const MIE: u16 = 0x304; // machine interrupt-enable register
pub const MTVEC: u16 = 0x305; // machine trap handler base address
pub const MCAUSE: u16 = 0x342;

#[macro_export]
macro_rules! read_csr {
    ($csr:expr) => ({
        let value: u64;
        unsafe {
            core::arch::asm!("csrr {0}, {1}",
                out(reg) value, const $csr
            );
        }
        value
    })
}

#[macro_export]
macro_rules! write_csr {
    ($csr:expr, $value:expr) => ({
        unsafe {
            core::arch::asm!("csrw {1}, {0}",
                in(reg) $value, const $csr
            );
        }
    })
}

pub fn init() {
    println!("hal: setting up trap handler");
    write_csr!(MTVEC, handler_wrapper);
    write_csr!(MIE, 0xFF);
}

fn handler() {
    let cause = read_csr!(MCAUSE);
    let is_interrupt = (cause >> 63) == 1;
    let exc_code = cause & ((-1i64 as u64) >> 1);
    if is_interrupt {
        println!("hal: stub: trap handler encountered interrupt");
        return;
    }

    panic!(
        "fatal exception: {} (code {})",
        riscv_exc_code_to_str(exc_code),
        exc_code
    );
}

fn riscv_exc_code_to_str(exc_code: u64) -> &'static str {
    match exc_code {
        0 => "instruction address misaligned",
        1 => "instruction access fault",
        2 => "illegal instruction",
        3 => "breakpoint",
        4 => "load address misaligned",
        5 => "load access fault",
        6 => "store/AMO access misaligned",
        7 => "store/AMO access fault",
        8 => "environment call from U-mode",
        9 => "environment call from S-mode",
        11 => "environment call from M-mode",
        12 => "instruction page fault",
        13 => "load page fault",
        15 => "store/AMO page fault",
        16 => "double trap",
        18 => "software check",
        19 => "hardware error",
        _ => "unknown",
    }
}

#[unsafe(naked)]
extern "C" fn handler_wrapper() {
    // TODO: save state to stack
    core::arch::naked_asm!(
        "call {handler}",
        "mret",
        handler = sym handler,
    );
}
