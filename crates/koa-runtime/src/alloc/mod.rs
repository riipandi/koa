//! Memory allocator - Bump pointer allocator
//!
//! A fast bump pointer allocator for Koa.

use std::ptr::NonNull;

/// Bump pointer allocator
pub struct BumpAllocator {
    start: NonNull<u8>,
    current: NonNull<u8>,
    end: NonNull<u8>,
}

unsafe impl Send for BumpAllocator {}
unsafe impl Sync for BumpAllocator {}

impl BumpAllocator {
    pub fn new(size: usize) -> Option<Self> {
        let layout = std::alloc::Layout::from_size_align(size, 8).ok()?;
        let start = unsafe { std::alloc::alloc(layout) };

        if start.is_null() {
            return None;
        }

        let start = NonNull::new(start)?;
        let end = unsafe { NonNull::new_unchecked(start.as_ptr().add(size)) };

        Some(Self {
            start,
            current: start,
            end,
        })
    }

    pub fn allocate(&mut self, size: usize, _align: usize) -> Option<NonNull<u8>> {
        let new_current = unsafe { self.current.as_ptr().add(size) };

        if new_current > self.end.as_ptr() {
            return None; // Out of memory
        }

        let ptr = self.current;
        self.current = unsafe { NonNull::new_unchecked(new_current) };

        Some(ptr)
    }

    pub fn reset(&mut self) {
        self.current = self.start;
    }

    pub fn used(&self) -> usize {
        self.current.as_ptr() as usize - self.start.as_ptr() as usize
    }

    pub fn capacity(&self) -> usize {
        self.end.as_ptr() as usize - self.start.as_ptr() as usize
    }
}

impl Drop for BumpAllocator {
    fn drop(&mut self) {
        let layout = unsafe { std::alloc::Layout::from_size_align_unchecked(self.capacity(), 8) };
        unsafe {
            std::alloc::dealloc(self.start.as_ptr(), layout);
        }
    }
}

/// Arena allocator for short-lived allocations
pub struct ArenaAllocator {
    // Arena allocator state
}

impl ArenaAllocator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn allocate(&mut self, _size: usize) -> Option<NonNull<u8>> {
        // Arena allocation
        None
    }
}

impl Default for ArenaAllocator {
    fn default() -> Self {
        Self::new()
    }
}
