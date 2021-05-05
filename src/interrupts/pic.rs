use pic8259_simple::ChainedPics;
use spin;

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
const PS2_PORT: u16 = 0x60;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Index {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl Index {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub struct PIC {
    pics: spin::Mutex<ChainedPics>,
}

pub const fn new() -> PIC {
    PIC{pics: spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) }) }
}

impl crate::interrupts::InterruptController for PIC {
    fn init(&self) -> () {
        unsafe{ self.pics.lock().initialize() };
    }

    fn eoi(&self, idx: u8) -> () {
        unsafe{ self.pics.lock().notify_end_of_interrupt(idx) };
    }

    fn index_u8(&self, interrupt: &str) -> u8 {
        match interrupt {
            "timer" => Index::Timer.as_u8(),
            "keyboard" => Index::Keyboard.as_u8(),
            &_ => panic!("unknown interrupt {}", interrupt),
        }
    }

    fn index_usize(&self, interrupt: &str) -> usize {
        match interrupt {
            "timer" => Index::Timer.as_usize(),
            "keyboard" => Index::Keyboard.as_usize(),
            &_ => panic!("unknown interrupt {}", interrupt),
        }
    }

    fn read_scancode(&self) -> u8 {
        use x86_64::instructions::port::Port;

        let mut port = Port::new(PS2_PORT);
        unsafe { port.read() }
    }
}
