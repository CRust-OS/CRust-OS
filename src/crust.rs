#![feature(lang_items)]
#![feature(asm)]
#![feature(type_macros)]
//#![feature(core_str_ext)]
//#![feature(ptr_as_ref)]
#![no_std]
#![allow(dead_code)]     // XXX: For now, because a lot of unused structs
extern crate rlibc;

pub mod hypercalls;

#[lang = "eh_personality"]
extern fn eh_personality() {}

unsafe fn crash() -> ! {
    //SCHEDOP_shutdown;
    loop {}
}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    hypercalls::console_io::write(b"Panic!\n\0");
    hypercalls::sched_op::shutdown(hypercalls::sched_op::ShutdownReason::crash)
}

mod startinfo;
mod sharedinfo;

#[no_mangle]
pub extern fn main(_x : *const startinfo::start_info) {
    //panic!();
    hypercalls::console_io::write(b"Hello world!\n");
    hypercalls::sched_op::shutdown(hypercalls::sched_op::ShutdownReason::poweroff);
}

//extern  {
    //pub static HYPERVISOR_start_info : startinfo::shared_info;
//}
