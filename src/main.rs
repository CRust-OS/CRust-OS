#![feature(lang_items)]
#![feature(asm)]
#![feature(stmt_expr_attributes)]
#![feature(type_macros)]
#![feature(associated_consts)]
#![feature(allocator)]
#![feature(alloc)]
#![feature(braced_empty_structs)] // XXX: For now
#![feature(start)]
//#![feature(core_str_ext)]
//#![feature(ptr_as_ref)]
#![no_std]
#![allow(dead_code)]              // XXX: For now, because a lot of unused structs
extern crate rlibc;
extern crate mm;
extern crate alloc;

mod xen;

pub use xen::poweroff;
pub use xen::console_io::STDOUT;
pub use xen::sbrk;
use xen::start_info::start_info_page;
use core::fmt::Write;
use alloc::boxed::Box;

#[lang = "eh_personality"]
extern fn eh_personality() {}

const LEN : usize = 400; //breaks at LEN=445;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(_fmt: core::fmt::Arguments, _file_line: &(&'static str, u32)) -> ! {
    xen::emergency_console::print(b"panic_fmt!\n\0");
    xen::crash();
}

fn print_init_info(){
    let _ = writeln!(STDOUT, "Magic: {}", core::str::from_utf8(&start_info_page.magic).unwrap_or("ERROR"));
    let _ = writeln!(STDOUT, "nr_pages: {:#X}", start_info_page.nr_pages);
    let _ = writeln!(STDOUT, "shared_info: {:#X}", start_info_page.shared_info);
}


#[no_mangle]
pub extern fn prologue() {
    use core::ptr;
    let null: *const u8 = ptr::null();
    let argv: *const *const u8 = &null;
    let result = main(0, argv);
    ()
}

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    xen::emergency_console::print(b"main!\n\0");
    let _ = writeln!(STDOUT, "Hello world!");

    let x = mm::__rust_allocate(1, 16);
    let y = mm::__rust_allocate(1, 16);

    unsafe {
        *x = 100;
        *y = 2 * *x;
        if *x == 100 && *y == 200 {
            xen::emergency_console::print(b"Assigned Properly!\n\0");
            let _ = writeln!(STDOUT, "Assigned Properly!");
        }
        else {
            xen::emergency_console::print(b"Error Assigning!\n\0");
            let _ = writeln!(STDOUT, "Error Assigning");
        }
    }

    let x = Box::new(12);

    if *x == 12 {
        xen::emergency_console::print(b"Box Worked!\n\0");
    }
    else {
        xen::emergency_console::print(b"Box Failed!\n\0");
    }
    
    let mut a = Box::new([0; LEN]);
    for i in 1..LEN {
        a[i] = i;
    }

    for i in 1..LEN {
        if a[i] != i {
            xen::emergency_console::print(b"Memory Error\n\0");
        }
    }

    print_init_info();

    0
}

