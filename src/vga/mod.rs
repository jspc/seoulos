use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

pub mod colours;
pub mod writer;

lazy_static! {
    pub static ref WRITER: Mutex<writer::Writer> = Mutex::new(writer::Writer::new(0, colours::ColourCode::new(colours::Colour::Yellow, colours::Colour::Black)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
