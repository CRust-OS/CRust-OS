#![feature(no_std)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(core_str_ext)]
#![feature(ptr_as_ref)]
#![no_std]
#![allow(dead_code)]     // XXX: For now, because a lot of unused structs
extern crate rlibc;

pub mod shims;
#[macro_use]
pub mod hypercalls;
pub mod events;
mod startinfo;
mod sharedinfo;
mod console;


#[no_mangle]
pub extern fn main(x : *const startinfo::start_info) {
    // hypercalls::say_hello();
    hypercalls::block();
    while true {}
}


//extern  {
    //pub static HYPERVISOR_start_info : startinfo::shared_info;
//}
