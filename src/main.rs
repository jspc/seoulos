#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sos_core::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
mod serial;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
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
