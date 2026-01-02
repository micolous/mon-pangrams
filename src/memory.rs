use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicIsize, Ordering::Relaxed},
};

// From https://doc.rust-lang.org/std/alloc/struct.System.html
// Modified to use `isize` instead of `usize` so we can track memory usage
// decreases more easily.
struct Counter;
static ALLOCATED: AtomicIsize = AtomicIsize::new(0);
static PEAK: AtomicIsize = AtomicIsize::new(0);
unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = unsafe { System.alloc(layout) };
        if !ret.is_null() {
            let r = ALLOCATED.fetch_add(layout.size() as isize, Relaxed);
            PEAK.fetch_max(r, Relaxed);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            System.dealloc(ptr, layout);
        }
        ALLOCATED.fetch_sub(layout.size() as isize, Relaxed);
    }
}

#[global_allocator]
static A: Counter = Counter;

/// Get `(current, peak)` memory usage.
pub fn get_memory_stats() -> (isize, isize) {
    (ALLOCATED.load(Relaxed), PEAK.load(Relaxed))
}
