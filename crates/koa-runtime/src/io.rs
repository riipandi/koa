//! I/O operations
//!
//! Provides input/output functions for Koa programs.
//!
//! # Implementation
//!
//! These functions use Rust's standard library (`println!`, `print!`, etc.)
//! internally, but expose a C ABI interface for FFI (Foreign Function Interface)
//! compatibility with Koa/LLVM.
//!
//! # Why C ABI?
//!
//! - **LLVM Compatibility**: LLVM IR knows how to call C ABI functions
//! - **FFI Boundary**: C ABI is the standard for cross-language interoperability
//! - **Rust Implementation**: Actual I/O uses Rust's std::io::println!, etc.
//!
//! # Architecture
//!
//! ```text
//! Koa Code → extern declarations → LLVM IR → C ABI Call → Rust Implementation (using println!)
//! ```

use std::ffi::CString;
use std::os::raw::c_char;

/// Print a string to stdout with newline
///
/// This function is called from Koa programs and uses Rust's `println!` macro internally.
/// The C ABI (`extern "C")` is only for the FFI boundary, not for using C functions.
#[unsafe(no_mangle)]
pub extern "C" fn koa_println(s: *const c_char) {
    unsafe {
        if s.is_null() {
            println!();
            return;
        }

        let c_str = std::ffi::CStr::from_ptr(s);
        match c_str.to_str() {
            Ok(string) => println!("{}", string),
            Err(_) => eprintln!("[Invalid UTF-8 string]"),
        }
    }
}

/// Print a string to stdout without newline
///
/// This function uses Rust's `print!` macro internally.
/// The C ABI is only for FFI compatibility with Koa.
#[unsafe(no_mangle)]
pub extern "C" fn koa_print(s: *const c_char) {
    unsafe {
        if s.is_null() {
            return;
        }

        let c_str = std::ffi::CStr::from_ptr(s);
        match c_str.to_str() {
            Ok(string) => print!("{}", string),
            Err(_) => eprint!("[Invalid UTF-8 string]"),
        }
    }
}

/// Print formatted string to stdout
/// Note: This is a simplified stub, proper formatting is TODO
///
/// TODO: Implement proper format string parsing
#[unsafe(no_mangle)]
pub extern "C" fn koa_printf(format: *const c_char) {
    unsafe {
        if format.is_null() {
            return;
        }

        let c_str = std::ffi::CStr::from_ptr(format);
        match c_str.to_str() {
            Ok(fmt) => print!("{}", fmt),
            Err(_) => eprint!("[Invalid format string]"),
        }
    }
}

/// Print a string to stdout (legacy name, uses C's puts)
#[unsafe(no_mangle)]
pub extern "C" fn puts(s: *const c_char) -> i32 {
    unsafe {
        if s.is_null() {
            println!();
            return 0;
        }

        let c_str = std::ffi::CStr::from_ptr(s);
        match c_str.to_str() {
            Ok(string) => println!("{}", string),
            Err(_) => eprintln!("[Invalid UTF-8 string]"),
        }
        0
    }
}

/// Print formatted string (legacy name, uses C's printf)
/// Note: This is a simplified stub, proper formatting is TODO
#[unsafe(no_mangle)]
pub extern "C" fn printf(format: *const c_char) -> i32 {
    unsafe {
        if format.is_null() {
            return 0;
        }

        let c_str = std::ffi::CStr::from_ptr(format);
        match c_str.to_str() {
            Ok(fmt) => print!("{}", fmt),
            Err(_) => eprint!("[Invalid format string]"),
        }
        0
    }
}

/// Get string length
#[unsafe(no_mangle)]
pub extern "C" fn koa_strlen(s: *const c_char) -> i32 {
    unsafe {
        if s.is_null() {
            return 0;
        }

        let c_str = std::ffi::CStr::from_ptr(s);
        c_str.to_bytes().len() as i32
    }
}

/// Read a line from stdin
/// Returns a newly allocated string that must be freed by the caller
#[unsafe(no_mangle)]
pub extern "C" fn koa_readline() -> *mut c_char {
    use std::io;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Remove trailing newline if present
            if input.ends_with('\n') {
                input.pop();
            }

            match CString::new(input) {
                Ok(c_string) => c_string.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a string allocated by koa_readline
#[unsafe(no_mangle)]
pub extern "C" fn koa_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
