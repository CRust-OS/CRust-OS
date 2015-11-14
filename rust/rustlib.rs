#![crate_type="staticlib"]
#![feature(no_std)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(core_str_ext)] 

#![no_builtins]
#![no_std]
// needed for compilation, see:
// https://github.com/rust-lang/rust/blob/master/src/doc/trpl/lang-items.md
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

pub mod rust_test;
pub mod rlibc;
