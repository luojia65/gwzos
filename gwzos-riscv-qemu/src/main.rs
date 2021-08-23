#![feature(naked_functions, asm)]
#![no_std]
#![no_main]

mod start;
mod mm;

#[cfg_attr(not(test), panic_handler)]
#[allow(unused)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
