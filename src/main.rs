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
#![feature(const_fn)]
//#![feature(core_str_ext)]
//#![feature(ptr_as_ref)]
#![no_std]
#![allow(dead_code)]              // XXX: For now, because a lot of unused structs
extern crate rlibc;
extern crate mm;
extern crate alloc;
extern crate collections;

mod std;
mod xen;

pub use xen::poweroff;
pub use xen::console_io::STDOUT;
use xen::start_info::start_info_page;
use core::fmt::Write;

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(_fmt: core::fmt::Arguments, _file_line: &(&'static str, u32)) -> ! {
    xen::emergency_console::print(b"panic_fmt!\n\0");
    xen::crash();
}

fn print_init_info(){
    let _ = writeln!(STDOUT, "Magic: {}\r", core::str::from_utf8(&start_info_page.magic).unwrap_or("ERROR"));
    let _ = writeln!(STDOUT, "nr_pages: {:#X}\r", start_info_page.nr_pages);
    let _ = writeln!(STDOUT, "shared_info: {:#X}\r", start_info_page.shared_info);
}


#[no_mangle]
pub extern fn prologue() {
    unsafe {
        xen::emergency_console::print(b"prologue!\n\0");
        use core::ptr;
        let null: *const u8 = ptr::null();
        let argv: *const *const u8 = &null;
        xen::emergency_console::print(b"mm::setup\n\0");
        mm::setup();
        xen::emergency_console::print(b"xen::console_io::initialize\n\0");
        xen::console_io::initialize();
        xen::emergency_console::print(b"xen::xenstore::initialize\n\0");
        xen::xenstore::initialize();
        xen::emergency_console::print(b"end of prologue!\n\0");
        let _result = main(0, argv);
        ()
    }
}

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    xen::emergency_console::print(b"main!\n\0");
    unsafe {
        let vm_name = xen::xenstore::XENSTORE.write().as_mut().unwrap().read("name").ok().unwrap();
        let _ = writeln!(STDOUT, "Hello world {}!\r", vm_name);
    }

    let x = mm::__rust_allocate(1, 16);
    let y = mm::__rust_allocate(1, 16);
    unsafe {
        *x = 100;
        *y = 2 * *x;
        if *x == 100 && *y == 200 {
            let _ = writeln!(STDOUT, "Assigned Properly!\r");
        }
        else {
            xen::emergency_console::print(b"Error Assigning!\n\0");
            let _ = writeln!(STDOUT, "Error Assigning\r");
        }

        if (y as usize - x as usize) == 16 {
            let _ = writeln!(STDOUT, "Alligned Properly\r");
        }
        else {
            xen::emergency_console::print(b"Error Alligning!\n\0");
            let _ = writeln!(STDOUT, "Error Alligning\r");
        }
    }
    print_init_info();
    0
}

