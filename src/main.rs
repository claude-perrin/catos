#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(catos::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use catos::println;
use bootloader::{BootInfo, entry_point};

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
    use catos::memory::BootInfoFrameAllocator;

    use x86_64::{structures::paging::{Page, Translate}, VirtAddr};

    println!("Hello World {}", "!");
    catos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe {memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // let page = Page::containing_address(VirtAddr::new(0));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    //
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // let addresses = [0xb8000, 0x201008, 0x0100_0020_1a10, boot_info.physical_memory_offset,];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }
    //
    //
    #[cfg(test)]
    test_main();

    println!("It did not crash ");
    catos::hlt_loop();

}


