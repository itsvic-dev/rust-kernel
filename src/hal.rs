pub const MHARTID: u16 = 0xF14;

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
