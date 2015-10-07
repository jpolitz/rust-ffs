use libc::{c_char, size_t, c_void};

extern {
    pub fn printk(fmt: *const c_char);
    pub fn k_malloc(sz: size_t) -> *mut c_void;
    pub fn k_realloc(p: *mut c_void, sz: size_t) -> *mut c_void;
    pub fn k_free(p: *mut c_void);
}
