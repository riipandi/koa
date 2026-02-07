//! Koa Runtime Library
//!
//! This library provides the runtime support for Koa programs.

pub mod alloc;
pub mod async_;
pub mod gc;

pub use alloc::{ArenaAllocator, BumpAllocator};
pub use async_::{AsyncRuntime, Task, sleep};
pub use gc::{GarbageCollector, WriteBarrier};

/// Runtime version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
