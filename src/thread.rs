use core::arch::riscv32;
use core::ptr;
use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

type TaskFunc = fn();

static TASK_PTR: AtomicPtr<TaskFunc> = AtomicPtr::new(ptr::null_mut());
static TASK_PENDING: AtomicBool = AtomicBool::new(false);

pub fn spawn(func: TaskFunc) {
    let func_ptr = &func as *const _ as *mut TaskFunc;
    TASK_PTR.store(func_ptr, Ordering::Relaxed);
    TASK_PENDING.store(true, Ordering::Release);
}

pub fn lock_hart(_hartid: usize) -> ! {
    loop {
        if TASK_PENDING.swap(false, Ordering::Acquire) {
            let task_ptr = TASK_PTR.load(Ordering::Relaxed);
            unsafe { (*task_ptr)() };
        }

        riscv32::nop();
    }
}
