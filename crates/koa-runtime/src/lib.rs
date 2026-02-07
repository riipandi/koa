//! Koa Runtime Library
//!
//! This library provides the runtime support for Koa programs.
//! It implements I/O operations, memory management, garbage collection,
//! and async runtime features.

pub mod alloc;
pub mod async_runtime;
pub mod gc;
pub mod io;

pub use alloc::{ArenaAllocator, BumpAllocator};
pub use async_runtime::{AsyncRuntime, Task, sleep};
pub use gc::{GarbageCollector, WriteBarrier};
pub use io::{koa_print, koa_printf, koa_println};

/// Runtime version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the Koa runtime
/// This should be called before any Koa code runs
#[unsafe(no_mangle)]
pub extern "C" fn koa_init() {
    // Initialize runtime systems
    // TODO: Initialize GC, allocator, etc.
}

/// Cleanup the Koa runtime
/// This should be called after Koa code finishes
#[unsafe(no_mangle)]
pub extern "C" fn koa_cleanup() {
    // Cleanup runtime systems
    // TODO: Cleanup GC, allocator, etc.
}
