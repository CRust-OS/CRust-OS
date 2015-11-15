#![feature(no_std)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(core_str_ext)]

#![no_std]

pub mod shims;
pub mod hypercalls;

#[no_mangle]
pub extern fn main() {
    hypercalls::block();
    hypercalls::say_hello();
    while true {}
}
