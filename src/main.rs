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

pub mod xen;
pub mod hypercalls;

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(_fmt: core::fmt::Arguments, _file_line: &(&'static str, u32)) -> ! {
    unsafe {
        hypercalls::console_io::write(b"panic_fmt!\n\0");
        hypercalls::sched_op::shutdown(&(hypercalls::sched_op::Shutdown { reason: hypercalls::sched_op::ShutdownReason::crash}) as *const hypercalls::sched_op::Shutdown)
    }
}

extern {
    static start_info: *const startinfo::start_info;
}

mod startinfo;
mod sharedinfo;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        hypercalls::console_io::write(b"Hello world!\n");
        hypercalls::sched_op::shutdown(&(hypercalls::sched_op::Shutdown { reason: hypercalls::sched_op::ShutdownReason::poweroff}) as *const hypercalls::sched_op::Shutdown);
    }
}
