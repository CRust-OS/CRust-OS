#![feature(lang_items)]
#![feature(collections)]
#![feature(asm)]
#![feature(stmt_expr_attributes)]
#![feature(type_macros)]
#![feature(associated_consts)]
#![feature(allocator)]
#![feature(alloc)]
#![feature(braced_empty_structs)] // XXX: For now
#![feature(start)]
#![feature(reflect_marker)]
#![feature(const_fn)]
//#![feature(core_str_ext)]
//#![feature(ptr_as_ref)]
#![no_std]
#![allow(dead_code)]              // XXX: For now, because a lot of unused structs
extern crate rlibc;
extern crate mm;
extern crate alloc;
extern crate collections;
extern crate nodrop;

#[macro_use]
mod std;
mod xen;

pub use xen::poweroff;
pub use xen::console_io::STDOUT;
pub use xen::mem::sbrk;
pub use xen::emergency_console::EMERGENCY_CONSOLE as DEBUG;
use xen::start_info::start_info_page;
use core::fmt::Write;
use alloc::boxed::Box;

#[lang = "eh_personality"]
extern fn eh_personality() {}

const LEN : usize = 3000;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(args: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    writeln!(STDOUT, "In file {} at line {} (XXX LINE COUNT IS CURRENTLY BROKEN)", file, line).unwrap();
    writeln!(STDOUT, "{:?}", args).unwrap();
    xen::crash();
}

fn print_init_info(){
    writeln!(STDOUT, "Magic: {}", core::str::from_utf8(&start_info_page.magic).unwrap_or("ERROR")).unwrap();
    writeln!(STDOUT, "nr_pages: {:#X}", start_info_page.nr_pages).unwrap();
    writeln!(STDOUT, "shared_info: {:#X}", start_info_page.shared_info).unwrap();
}


#[no_mangle]
pub extern fn prologue() {
    unsafe {
        writeln!(DEBUG, "prologue!").unwrap();
        use core::ptr;
        let null: *const u8 = ptr::null();
        let argv: *const *const u8 = &null;
        writeln!(DEBUG, "mm::setup").unwrap();
        writeln!(DEBUG, "xen::console_io::initialize").unwrap();
        xen::console_io::initialize();
        writeln!(DEBUG, "xen::xenstore::initialize").unwrap();
        xen::xenstore::initialize();
        writeln!(DEBUG, "end of prologue!").unwrap();
        let _result = main(0, argv);
    }
}

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    writeln!(DEBUG, "main!").unwrap();

    let _ = if let Err(e) = xen::grant_tables::init_grant_table() {
        writeln!(STDOUT, "Error initializing grant table: {}", e)
    } else {
        writeln!(STDOUT, "Initialized grant table")
    };

    let mut s = collections::String::new();
    writeln!(STDOUT, "Growing sequences of numbers to test allocation...");
    for _ in 0 .. 1 {
        for j in 0 .. 10 {
            s.push(('0' as u8 + j) as char);
            writeln!(STDOUT, "{}, {}, {}", &s, s.len(), s.as_ptr() as usize);
        }
    }


    unsafe {
        let vm_name = xen::xenstore::XENSTORE.write().as_mut().unwrap().read("name").unwrap();
        writeln!(STDOUT, "Hello world {}!", vm_name).unwrap();
        let key = "examplekey";
        let value = "examplevalue";
        xen::xenstore::XENSTORE.write().as_mut().unwrap().write(key, value).unwrap();
        writeln!(STDOUT, "Wrote!").unwrap();
//        let read = xen::xenstore::XENSTORE.write().as_mut().unwrap().read(key).unwrap();
//        writeln!(STDOUT, "wrote {}, read {}", value, read).unwrap();
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

    let _ = writeln!(DEBUG, "done!").unwrap();
    print_init_info();

    0
}

