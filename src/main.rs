#![feature(riscv_ext_intrinsics)]
#![no_std]
#![no_main]

extern crate alloc;

use core::arch::riscv32;
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};
use riscv_rt::entry;
use riscv_rt_test::{print, println};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[entry]
unsafe fn main(hartid: usize) -> ! {
    loop {
        if hartid == COUNTER.load(Ordering::Relaxed) {
            print!("{}, ", hartid);
            COUNTER.fetch_add(1, Ordering::Relaxed);
            break;
        }

        riscv32::nop();
    }

    if hartid == 0 {
        let mut string = alloc::string::String::new();

        string.push_str("adasdasd");
        string.push_str("123123");

        println!("string: {}", string);
    }

    while COUNTER.load(Ordering::Relaxed) != 32 {}

    shutdown()
}

#[no_mangle]
pub extern "Rust" fn _mp_hook(hartid: usize) -> bool {
    hartid == 0
}

fn shutdown() -> ! {
    unsafe { (0x100000 as *mut u16).write_volatile(0x5555) };

    loop {
        unsafe { riscv32::wfi() }
    }
}

#[panic_handler]
fn panic_handle(info: &PanicInfo) -> ! {
    println!("\n{}", info);

    shutdown()
}
