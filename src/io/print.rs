use core::fmt::{self, Write};

use super::uart::Uart;
use crate::sync::SpinLock;

pub struct Writer;

static WRITE_LOCK: SpinLock = SpinLock::new();

impl Write for Writer {
    fn write_str(&mut self, str: &str) -> fmt::Result {
        for b in str.bytes() {
            Uart::write_char(b);
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITE_LOCK.lock();
    Writer.write_fmt(args).unwrap();
    unsafe { WRITE_LOCK.unlock() }
}
