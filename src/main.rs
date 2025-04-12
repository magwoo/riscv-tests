#![feature(riscv_ext_intrinsics)]
#![no_std]
#![no_main]

use core::arch::riscv32;
use riscv_rt::entry;
use riscv_rt_test::uart::Uart;
use riscv_rt_test::{print, println, thread};

#[entry]
unsafe fn main(hartid: usize) -> ! {
    if hartid == 0 {
        println!("Hello from main hart!");

        thread::spawn(|| loop {
            print!(
                "{}",
                Uart::read_char_blocked() // Uart::read_char_blocked()
            )
        });

        // for _ in 0..1000000 {
        //     riscv32::nop();
        // }

        loop {}
    } else {
        thread::lock_hart(hartid)
    }
}

pub fn getchar() -> u8 {
    const UART_BASE: usize = 0x1000_0000;

    loop {
        unsafe {
            let status = (UART_BASE as *mut u8).add(5).read_volatile();
            if status & 1 != 0 {
                return (UART_BASE as *mut u8).read_volatile();
            } else {
                riscv32::nop();
            }
        }
    }
}
