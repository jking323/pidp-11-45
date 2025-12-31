pub mod gpio;
pub mod uart;

pub const PERIPHERAL_BASE: u64 = 0xFE000000;
pub const AUX_BASE: u64 = PERIPHERAL_BASE + 0x215000;
