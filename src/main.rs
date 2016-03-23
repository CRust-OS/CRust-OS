#![feature(lang_items)]
#![feature(collections)]
#![feature(asm)]
#![feature(stmt_expr_attributes)]
#![feature(type_macros)]
#![feature(associated_consts)]
#![feature(allocator)]
#![feature(alloc)]
#![feature(start)]
#![feature(reflect_marker)]
#![feature(const_fn)]
#![feature(type_ascription)]
#![feature(unique)]
//#![feature(core_str_ext)]
//#![feature(ptr_as_ref)]
#![no_std]
#![allow(dead_code)]              // XXX: For now, because a lot of unused structs
extern crate rlibc;
extern crate mm;
extern crate alloc;
extern crate collections;
#[macro_use]
extern crate x86;

#[macro_use]
mod std;
mod xen;

pub use xen::poweroff;
pub use xen::STDOUT;
pub use xen::mem::sbrk;
pub use xen::DEBUG;
use core::fmt::Write;
use alloc::boxed::Box;

#[lang = "eh_personality"]
unsafe extern fn eh_personality() {
    int!(3);
    xen::crash();
}

#[lang = "eh_unwind_resume"]
unsafe extern fn eh_unwind_resume(_args: *mut u8) -> ! {
    int!(3);
    xen::crash();
}

const LEN : usize = 3000;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(args: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    writeln!(STDOUT, "In file {} at line {} (XXX LINE COUNT IS CURRENTLY BROKEN)", file, line).unwrap();
    writeln!(STDOUT, "{:?}", args).unwrap();
    xen::crash();
}

#[no_mangle]
pub extern fn prologue(start_info_page : *mut xen::ffi::start_info::StartInfoPage) {
    unsafe {
        use core::ptr;
        xen::initialize(ptr::read(start_info_page));
        writeln!(STDOUT, "args: {}", core::str::from_utf8(&ptr::read(start_info_page).cmd_line).unwrap_or("ERROR")).unwrap();
        let null: *const u8 = ptr::null();
        let argv: *const *const u8 = &null;
        let _result = main(0, argv);
    }
}

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    writeln!(STDOUT, "main!").unwrap();
    let mut s = collections::String::new();
    writeln!(STDOUT, "Growing sequences of numbers to test allocation...").unwrap();
    for _ in 0 .. 1 {
        for j in 0 .. 10 {
            s.push(('0' as u8 + j) as char);
            writeln!(STDOUT, "{}, {}, {}", &s, s.len(), s.as_ptr() as usize).unwrap();
        }
    }

    unsafe {
        let xenstore = xen::xenstore::XENSTORE.write().as_mut().unwrap();
        let vm_name = xenstore.read("name").unwrap().unwrap();
        writeln!(STDOUT, "Hello world {}!", vm_name).unwrap();
        let perms = xenstore.get_permissions("name").unwrap();
        writeln!(STDOUT, "Can name be changed: {}", perms).unwrap();
    }

    let x = Box::new(12);

    if *x == 12 {
        writeln!(STDOUT, "Box Worked").unwrap();
    }
    else {
        writeln!(STDOUT, "Box Failed").unwrap();
    }
    
    let mut a = Box::new([0; LEN]);
    for i in 1..LEN {
        a[i] = i;
    }

    for i in 1..LEN {
        if a[i] != i {
            writeln!(STDOUT, "Error in Memory").unwrap();
        }
    }

    writeln!(DEBUG, "done!").unwrap();

    0
}

