#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(alloc_error_handler)]
#![test_runner(sos_core::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga;

extern crate alloc;

use alloc::string::String;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use sos_core::allocator;
use sos_core::memory;
use sos_core::memory::frame_allocator::BootInfoFrameAllocator;
use sos_core::task::{executor::Executor, keyboard, Task};
use x86_64::VirtAddr;

use crate::vga::WRITER;

const VERSION: &str = env!("CARGO_PKG_VERSION");

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    title();

    init_printer(
        "Initialising the Global Descriptor Table",
        &sos_core::gdt::init,
    );
    init_printer(
        "Initialising interrupt handlers",
        &sos_core::interrupts::init,
    );
    init_printer("Enabling interrupts", &sos_core::enable_interrupts);

    // This entire chunk is because I can't quite suss out how to do all of this
    // outside of main; there's an error wrt phys_mem_offset and mapper not being
    // static, but making them static does some other tedious erroring.
    //
    // Instead, then, we can just mimic the same output. I'm sure as I start to
    // get rust a bit more this will all go away, but I've been scratching my head
    // for an hour or two and I just want to move on for now.
    let msg = "Starting memory allocator";
    print!("[***] {}\r", msg);
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    match allocator::init(&mut mapper, &mut frame_allocator) {
        Ok(_) => {
            WRITER.lock().colour_code = vga::colours::ColourCode::new(
                vga::colours::Colour::Green,
                vga::colours::Colour::Black,
            );
            print!("[OK ] ");
        }

        Err(_) => {
            WRITER.lock().colour_code = vga::colours::ColourCode::new(
                vga::colours::Colour::Red,
                vga::colours::Colour::Black,
            );
            print!("[Err] ");
        }
    }

    WRITER.lock().reset_colour();
    println!("{}", msg);
    println!();

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(String::from("async_test"), example_task()));
    executor.spawn(Task::new(
        String::from("keyboard"),
        keyboard::print_keypresses(),
    ));

    println!("kernel task table:");
    println!("{}", executor.table());

    executor.run();
}

fn title() {
    WRITER.lock().colour_code =
        vga::colours::ColourCode::new(vga::colours::Colour::Pink, vga::colours::Colour::Black);
    println!(
        r" ____                   _  ___  ____
/ ___|  ___  ___  _   _| |/ _ \/ ___|
\___ \ / _ \/ _ \| | | | | | | \___ \
 ___) |  __/ (_) | |_| | | |_| |___) |
|____/ \___|\___/ \__,_|_|\___/|____/

"
    );

    WRITER.lock().colour_code =
        vga::colours::ColourCode::new(vga::colours::Colour::LightCyan, vga::colours::Colour::Black);
    println!("Welcome to SeoulOS Core");

    WRITER.lock().colour_code =
        vga::colours::ColourCode::new(vga::colours::Colour::Yellow, vga::colours::Colour::Black);
    println!("ver: {}", VERSION);

    WRITER.lock().colour_code =
        vga::colours::ColourCode::new(vga::colours::Colour::LightRed, vga::colours::Colour::Black);
    println!("booting....\n");

    WRITER.lock().reset_colour();
}

fn init_printer(msg: &str, f: &dyn Fn() -> Result<(), ()>) {
    print!("[***] {}\r", msg);

    match f() {
        Ok(_) => {
            WRITER.lock().colour_code = vga::colours::ColourCode::new(
                vga::colours::Colour::Green,
                vga::colours::Colour::Black,
            );
            print!("[OK ] ");
        }

        Err(_) => {
            WRITER.lock().colour_code = vga::colours::ColourCode::new(
                vga::colours::Colour::Red,
                vga::colours::Colour::Black,
            );
            print!("[Err] ");
        }
    }

    WRITER.lock().reset_colour();
    println!("{}", msg);
}

async fn async_number() -> u32 {
    42
}

async fn async_string() -> String {
    String::from("coming back to where you started is not the same as never leaving")
}

async fn example_task() {
    let number = async_number().await;
    let s = async_string().await;

    println!(
        "async self test. do not adjust your kernel. the meaning is {}. {}",
        number, s
    );
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
