use core::arch::riscv32;
use core::panic::PanicInfo;

use crate::println;

#[no_mangle]
extern "Rust" fn _mp_hook(hartid: usize) -> bool {
    hartid == 0
}

pub fn shutdown() -> ! {
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
