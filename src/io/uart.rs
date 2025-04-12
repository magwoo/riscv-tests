use core::arch::riscv32;

use crate::{extra::shutdown, println};

const UART_BASE: *const u8 = 0x1000_0000 as _;

pub struct Uart;

impl Uart {
    pub fn write_char(char: u8) {
        unsafe { core::ptr::write_volatile(0x1000_0000 as *mut u8, char) }
    }

    pub fn read_char() -> Option<char> {
        let first_char = Self::read_byte()?;

        let len = utf8_char_length(first_char);
        let mut buf = [first_char, 0, 0, 0];

        for i in buf.iter_mut().take(len).skip(1) {
            *i = Self::read_byte()?;
        }

        let char = unsafe { str::from_utf8_unchecked(&buf).chars().next()? };

        special_char_map(char)
    }

    pub fn read_char_blocked() -> char {
        loop {
            if let Some(ch) = Self::read_char() {
                return ch;
            }

            riscv32::nop();
        }
    }

    pub fn read_byte() -> Option<u8> {
        let status = unsafe { UART_BASE.add(5).read_volatile() };

        if status & 1 != 0 {
            special_byte_map(unsafe { UART_BASE.read_volatile() })
        } else {
            None
        }
    }

    pub fn read_byte_blocked() -> u8 {
        loop {
            if let Some(ch) = Self::read_byte() {
                return ch;
            }

            riscv32::nop();
        }
    }
}

fn special_char_map(char: char) -> Option<char> {
    match char {
        '\r' => println!(),
        _ => return Some(char),
    };

    None
}

fn special_byte_map(byte: u8) -> Option<u8> {
    match byte {
        3 => shutdown(),
        _ => return Some(byte),
    };

    None
}

fn utf8_char_length(first_byte: u8) -> usize {
    match first_byte {
        b if b < 0x80 => 1,
        b if b & 0xE0 == 0xC0 => 2,
        b if b & 0xF0 == 0xE0 => 3,
        b if b & 0xF8 == 0xF0 => 4,
        _ => 1,
    }
}
