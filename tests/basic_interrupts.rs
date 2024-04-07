#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(catos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use catos::init;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    catos::test_panic_handler(info)
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}


