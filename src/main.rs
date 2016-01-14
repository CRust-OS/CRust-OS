#![feature(lang_items)]
#![feature(asm)]
#![feature(stmt_expr_attributes)]
#![feature(type_macros)]
#![feature(associated_consts)]
#![feature(braced_empty_structs)] // XXX: For now
#![feature(start)]
//#![feature(core_str_ext)]
//#![feature(ptr_as_ref)]
#![no_std]
#![allow(dead_code)]              // XXX: For now, because a lot of unused structs
extern crate rlibc;

mod xen;

pub use xen::poweroff;

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(_fmt: core::fmt::Arguments, _file_line: &(&'static str, u32)) -> ! {
    xen::emergency_console::print(b"panic_fmt!\n\0");
    xen::crash();
}

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    xen::emergency_console::print(b"main!\n\0");
    0
}
