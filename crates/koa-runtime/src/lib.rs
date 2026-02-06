//! Koa Runtime Library
//!
//! This library provides the runtime support for Koa programs.

pub mod gc;
pub mod async_;
pub mod alloc;

pub use gc::{GarbageCollector, WriteBarrier};
pub use async_::{AsyncRuntime, Task, sleep};
pub use alloc::{BumpAllocator, ArenaAllocator};

/// Runtime version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
