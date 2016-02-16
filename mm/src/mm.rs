//! Memory Manager for CRust-OS

#![no_std]
#![feature(allocator)]
#![feature(lang_items)]
#![allocator]

#[no_mangle]
///! error number, used by dlmalloc familu
pub static mut __errno_location : usize = 0;

extern {
    fn dlmalloc(sz : usize) -> *mut u8;
    fn dlfree(p : *mut u8);
    fn dlrealloc_in_place(p : *mut u8, sz : usize) -> *mut u8;
    fn dlrealloc(p : *mut u8, sz : usize) -> *mut u8;
}

#[no_mangle]
///! allocates `size` bytes with allignment `_align`
///! Currently, the pointer can't be freed
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    unsafe {
        dlmalloc(size)
    }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    unsafe {
        dlfree(ptr);
    }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,  _align: usize) 
    -> *mut u8 {
        unsafe {
            //TODO: what if old_size != _size
            dlrealloc(ptr, size)
        }
}

#[no_mangle]
#[allow(unused_variables)]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize, _size: usize, _align: usize)
    -> usize {
        unsafe {
            //TODO: what if old_size != _size
            dlrealloc_in_place(_ptr, _size);
            _size
        }
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
        size
}

