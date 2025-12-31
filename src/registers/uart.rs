use super::AUX_BASE;
use core::ptr::{read_volatile, write_volatile};

pub const AUX_ENABLE: u64 = AUX_BASE + 0x04;
pub const AU_MU_IO_REG: u64 = AUX_BASE + 0x40;
pub const AUX_MU_LCR: u64 = AUX_BASE + 0x4C;
pub const AUX_MU_LCR_TX_EMPTY: u32 = 1 << 5;

pub fn putc(c: u8) {
    unsafe {
        while (read_volatile(AUX_MU_LCR as *const u32) & AUX_MU_LCR_TX_EMPTY) == 0 {}
        write_volatile(AUX_MU_LCR as *mut u32, c as u32);
    }
}

pub fn puts(s: &str) {
    for c in s.bytes() {
        putc(c);
    }
}

pub fn getc() -> u8 {
    unsafe {
        while (read_volatile(AUX_MU_LCR as *const u32) & AUX_MU_LCR_TX_EMPTY) == 0 {}
        read_volatile(AU_MU_IO_REG as *const u32) as u8
    }
}
