#![no_std]
#![no_main]
mod registers;

use crate::registers::gpio;
use crate::registers::uart;
use core::{
    panic::PanicInfo,
    ptr::{read_volatile, write_volatile},
};
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

core::arch::global_asm!(
    ".global _start",
    "_start:",
    "ldr x0, =0x80000",
    "mov sp, x0",
    "bl main",
    "b ."
);

fn delay(count: u64) {
    for _ in 0..count {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    uart::puts("Hello World\r\n");
    unsafe {
        let mut gpio_fsel4 = core::ptr::read_volatile(gpio::GPIO_GPFSEL4 as *const u32);
        gpio_fsel4 &= !(0b111 << 21);
        gpio_fsel4 |= 0b001 << 21;
        core::ptr::write_volatile(gpio::GPIO_GPFSEL4 as *mut u32, gpio_fsel4);

        loop {
            core::ptr::write_volatile(gpio::GPIO_GPSET1 as *mut u32, 1 << 15);
            delay(1_000_000);
            core::ptr::write_volatile(gpio::GPIO_GPCLR1 as *mut u32, 1 << 15);
            delay(1_000_000);
        }
    }
}
