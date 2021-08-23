#![feature(naked_functions, asm)]
#![no_std]
#![no_main]

#[macro_use]
mod console;

mod start;
mod mm;
mod sbi;

#[cfg_attr(not(test), panic_handler)]
#[allow(unused)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
