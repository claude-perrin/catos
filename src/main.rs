#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(catos::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use catos::println;
use catos::serial_println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("MAIN panic");
    catos::test_panic_handler(info)
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World {}", "!");
    serial_println!("MAIN _start");
    #[cfg(test)]
    test_main();
    loop {}
}


