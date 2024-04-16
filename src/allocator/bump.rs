use core::ptr::null_mut;
use core::ptr;

use super::{align_down, Locked};
use alloc::alloc::{GlobalAlloc, Layout};

macro_rules! try_null {
    ( $e:expr ) => {
        match $e {
            None => return null_mut(),
            Some(e) => e,
        }
    };
}

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    } 


    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = self.heap_end;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();
        let next = bump.next as usize;
        let heap_start = bump.heap_start as usize;

        let next = try_null!(next.checked_sub(layout.size()));

        let next = align_down(next, layout.align());

        if next < heap_start{
            return null_mut();
        }

        bump.allocations += 1;
        bump.next = next;
        next as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout){
        let mut bump = self.lock();

        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_end;
        }
    }
}
