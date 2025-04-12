use core::alloc::{GlobalAlloc, Layout};

use crate::println;

#[global_allocator]
static GLOBAL_ALLOCATOR: BumpAllocator = BumpAllocator::new();

#[derive(Debug, Default)]
pub struct BumpAllocator {
    offset: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self { offset: 0 }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        println!(
            "\ncalled alloc with size: {}, align: {}",
            layout.size(),
            layout.align()
        );

        let heap_start = riscv_rt::heap_start() as usize;
        let ptr = (heap_start + self.offset) as *mut u8;

        let self_mut = self as *const _ as *mut Self;
        (*self_mut).offset += layout.size();

        ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
