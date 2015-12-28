#![feature(lang_items)]
#![feature(asm)]
#![feature(type_macros)]
//#![feature(core_str_ext)]
//#![feature(ptr_as_ref)]
#![no_std]
#![allow(dead_code)]     // XXX: For now, because a lot of unused structs
extern crate rlibc;

pub mod shims;
pub mod hypercalls;
mod startinfo;
mod sharedinfo;


#[no_mangle]
pub extern fn main(_x : *const startinfo::start_info) {
    hypercalls::console_io::write(b"Hello world!\n\0");
}

//extern  {
    //pub static HYPERVISOR_start_info : startinfo::shared_info;
//}
