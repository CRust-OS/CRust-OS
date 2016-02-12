/// Current end of heap. Updated by `sbrk()`
static mut heap_end : usize = 0;

/// Start of the heap, from `crust.lds`
extern {
    static HEAP : *const u8;
}

#[no_mangle]
/// Increase the end of the heap. Initializes heap on first call. Based off [newlib implementation]
/// (https://sourceware.org/newlib/libc.html#Syscalls)
pub extern fn sbrk(incr: usize) -> *const u8 {
    unsafe {
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
pub extern fn setup() {
    sbrk(0);
}
