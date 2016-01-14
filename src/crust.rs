#![feature(lang_items)]
#![feature(asm)]
#![feature(stmt_expr_attributes)]
#![feature(type_macros)]
#![feature(associated_consts)]
#![feature(allocator)]
//#![feature(core_str_ext)]
//#![feature(ptr_as_ref)]
#![no_std]
#![allow(dead_code)]              // XXX: For now, because a lot of unused structs
#![feature(braced_empty_structs)] // XXX: For now
extern crate rlibc;

pub mod xen;
pub mod hypercalls;
pub mod mm;

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    unsafe {
        hypercalls::console_io::write(b"Panic!\n\0");
        hypercalls::sched_op::shutdown(&(hypercalls::sched_op::Shutdown { reason: hypercalls::sched_op::ShutdownReason::crash}) as *const hypercalls::sched_op::Shutdown)
    }
}

mod startinfo;
mod sharedinfo;

#[no_mangle]
pub extern fn main(_x : *const startinfo::start_info) {
    mm::setup();
    unsafe {
        hypercalls::console_io::write(b"Hello world!\n");
        hypercalls::sched_op::shutdown(&(hypercalls::sched_op::Shutdown { reason: hypercalls::sched_op::ShutdownReason::poweroff}) as *const hypercalls::sched_op::Shutdown);
    }
}
