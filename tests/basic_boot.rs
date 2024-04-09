#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(catos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use catos::println;
use core::panic::PanicInfo;
use catos::vga_buffer::{BUFFER_HEIGHT, WRITER};


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();
    catos::hlt_loop();    
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    catos::test_panic_handler(info)
}
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


#[test_case]
fn test_println() {
    println!("test_println output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use x86_64::instructions::interrupts;
    use core::fmt::Write;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}

