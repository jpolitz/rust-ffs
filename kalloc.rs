extern crate alloc;
extern crate libc;

use raw;
use libc::{c_void, size_t};

pub fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    let p = unsafe { raw::k_malloc(size as u64) as *mut u8 };
    if p.is_null() {
        return alloc::heap::EMPTY as *mut u8;
    } else {
        return p;
    }
}

pub fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    unsafe { raw::k_free(ptr as *mut c_void) };
}

pub fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize,
                         align: usize) -> *mut u8 {
    let new_p = unsafe { raw::k_realloc(ptr as *mut c_void, size as size_t) };
    if new_p.is_null() {
        return alloc::heap::EMPTY as *mut u8;
    } else {
        return new_p as *mut u8;
    }
}

/*
 * The Linux kernel does not support in-place reallocation.
 */
pub fn __rust_reallocate_inplace(ptr: *mut u8, old_size: usize, size: usize,
                                 align: usize) -> usize {
    return __rust_usable_size(old_size, align);
}

pub fn __rust_usable_size(size: usize, align: usize) -> usize {
    return 0;
}
