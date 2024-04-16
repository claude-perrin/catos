#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(catos::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use catos::println;
use bootloader::{BootInfo, entry_point};

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

entry_point!(kernel_main);

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    catos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use catos::serial_println;
    serial_println!("MAIN panic");
    catos::test_panic_handler(info)
}


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use catos::memory;
    use catos::allocator;

    use x86_64::{structures::paging::{Page, Translate}, VirtAddr};

    println!("Hello World {}", "!");
    catos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe {memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());


    let reference_counted = Rc::new(vec![1,2,3]);
    let cloned_reference = Rc::clone(&reference_counted);
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("current reference count is {} now", Rc::strong_count(&cloned_reference));


    #[cfg(test)]
    test_main();

    println!("It did not crash ");
    catos::hlt_loop();

}


