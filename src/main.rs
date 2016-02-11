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
pub use xen::emergency_console::EMERGENCY_CONSOLE as DEBUG;
use xen::start_info::start_info_page;
use core::fmt::Write;

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(_fmt: core::fmt::Arguments, _file_line: &(&'static str, u32)) -> ! {
    writeln!(DEBUG, "panic_fmt!");
    xen::crash();
}

fn print_init_info(){
    let _ = writeln!(STDOUT, "Magic: {}", core::str::from_utf8(&start_info_page.magic).unwrap_or("ERROR"));
    let _ = writeln!(STDOUT, "nr_pages: {:#X}", start_info_page.nr_pages);
    let _ = writeln!(STDOUT, "shared_info: {:#X}", start_info_page.shared_info);
}


#[no_mangle]
pub extern fn prologue() {
    unsafe {
        writeln!(DEBUG, "prologue!");
        use core::ptr;
        let null: *const u8 = ptr::null();
        let argv: *const *const u8 = &null;
        writeln!(DEBUG, "mm::setup");
        mm::setup();
        writeln!(DEBUG, "xen::console_io::initialize");
        xen::console_io::initialize();
        writeln!(DEBUG, "xen::xenstore::initialize");
        xen::xenstore::initialize();
        writeln!(DEBUG, "end of prologue!");
        let _result = main(0, argv);
        ()
    }
}

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    writeln!(DEBUG, "main!");
    unsafe {
        let vm_name = xen::xenstore::XENSTORE.write().as_mut().unwrap().read("name").ok().unwrap();
        let _ = writeln!(STDOUT, "Hello world {}!", vm_name);
        let key = "examplekey";
        let value = "examplevalue";
        let wrote = xen::xenstore::XENSTORE.write().as_mut().unwrap().write(key, value);
        let _ = writeln!(STDOUT, "Wrote!");
        loop {}
        let read = xen::xenstore::XENSTORE.write().as_mut().unwrap().read(key).ok().unwrap();
        let _ = writeln!(STDOUT, "wrote {}, read {}", value, read);
    }

    let x = mm::__rust_allocate(1, 16);
    let y = mm::__rust_allocate(1, 16);
    unsafe {
        *x = 100;
        *y = 2 * *x;
        if *x == 100 && *y == 200 {
            let _ = writeln!(STDOUT, "Assigned Properly!");
        }
        else {
            writeln!(DEBUG, "Error Assigning!");
            let _ = writeln!(STDOUT, "Error Assigning");
        }

        if (y as usize - x as usize) == 16 {
            let _ = writeln!(STDOUT, "Alligned Properly");
        }
        else {
            writeln!(DEBUG, "Error Alligning!");
            let _ = writeln!(STDOUT, "Error Alligning");
        }
    }
    print_init_info();
    0
}

