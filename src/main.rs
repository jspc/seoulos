#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sos_core::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use sos_core::memory;
use sos_core::memory::frame_allocator::BootInfoFrameAllocator;
use x86_64::VirtAddr;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    sos_core::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut _mapper = unsafe { memory::init(phys_mem_offset) };

    let mut _frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    #[cfg(test)]
    test_main();

    sos_core::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sos_core::test_panic_handler(info);
}
