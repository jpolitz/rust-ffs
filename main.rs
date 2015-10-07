#![feature(core_str_ext)]
#![feature(libc)]
#![feature(alloc)]
#![feature(heap_api)]
#![feature(no_std)]

#![allow(dead_code)]
#![allow(unused_variables)]

#![no_std]

extern crate libc;

use libc::{c_void};

#[macro_use]
mod macros;
mod raw;
mod kalloc;

#[no_mangle]
pub fn rust_main() {
    println!("rust-ffs kernel module loaded successfully");
}

#[no_mangle]
pub fn device_read(file: *mut c_void, buf: *mut u8, sz: u64, loff: *mut c_void) -> i64 {
    return 0;
}

#[no_mangle]
pub fn device_write(file: *mut c_void, buf: *const u8, sz: u64, loff: *mut c_void) -> i64 {
    return 0;
}
