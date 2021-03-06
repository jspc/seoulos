use lazy_static::lazy_static;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::gdt;
use crate::hlt_loop;
use crate::println;

pub mod pic;

trait InterruptController {
    fn init(&self);
    fn eoi(&self, _: u8);
    fn index_u8(&self, interrupt: &str) -> u8;
    fn index_usize(&self, interrupt: &str) -> usize;
    fn read_scancode(&self) -> u8;
}

#[cfg(feature = "pic")]
pub static IC: pic::PIC = pic::new();

#[cfg(all(feature = "pic", feature = "apic"))]
compile_error!("feature \"pic\" and feature \"apic\" cannot be enabled at the same time- choose an interrupt scheme");

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[IC.index_usize("timer")].set_handler_fn(timer_interrupt_handler);
        idt[IC.index_usize("keyboard")].set_handler_fn(keyboard_interrupt_handler);

        idt.page_fault.set_handler_fn(page_fault_handler);

        idt
    };
}

pub fn init() -> Result<(), ()> {
    init_idt();
    IC.init();

    Ok(())
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    IC.eoi(IC.index_u8("timer"));
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);

    hlt_loop();
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let scancode = IC.read_scancode();

    crate::task::keyboard::add_scancode(scancode);

    IC.eoi(IC.index_u8("keyboard"));
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn test_breakpoint_exception() {
        // invoke a breakpoint exception
        // if the test goes past this exception then happy day:
        // we've trapped the exception properly
        x86_64::instructions::interrupts::int3();
    }
}
