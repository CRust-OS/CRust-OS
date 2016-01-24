//! Memory Manager for CRust-OS

#![no_std]
#![feature(allocator)]
#![feature(lang_items)]
#![allocator]


static mut heap_end : usize = 0;
static mut __errno_location : usize = 0;

extern {
    static HEAP : *const u8;
}

#[no_mangle]
pub extern fn setup() {
    sbrk(0);
}

///! Increase the end of the heap. Also initializes
///! heap on first call. Based off [newlib implementation]
///! (https://sourceware.org/newlib/libc.html#Syscalls)
#[no_mangle]
pub extern fn sbrk(incr: usize) -> *const u8 {
    unsafe {
        let previous_heap_end : usize;
        if heap_end == 0 {
                heap_end = HEAP as usize;
        }

        let previous_heap_end = heap_end;

        // detect unsiged overflow
        // TODO apparently Rust detects unsigned integer overflow...
        if (heap_end + incr) < heap_end {
            //TODO fail gracefully
            loop {};
        }
        heap_end += incr;
        previous_heap_end as *const u8
    }
}

#[no_mangle]
///! allocates `size` bytes with allignment `_align`
///! Currently, the pointer can't be freed
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    unsafe {
        heap_end = heap_end + size + (_align - size % _align);
        heap_end as *mut u8
    }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {

}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,  _align: usize) 
    -> *mut u8 {
        __rust_allocate(size, _align)
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize, _size: usize, _align: usize)
    -> usize {
        0
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
        size
}

