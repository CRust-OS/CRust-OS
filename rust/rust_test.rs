// kernel.rs - a test for calling a rust program from mini-is

//#![crate_type="staticlib"]
//#![feature(no_std)]
//#![feature(lang_items)]
//#![feature(asm)]

//#![no_std]
//#![feature(collections)]
//#![feature(alloc)]

//extern crate collections;
//extern crate alloc;
//use collections::vec::Vec;
//use alloc::boxed::Box;


#[no_mangle]
pub extern fn loop_rs () {
    let i = 5;
    while i < 6 {};
}

macro_rules! hypercall3 {
    ( $name : expr, $a1 : expr, $a2 : expr, $a3 : expr ) => {
        unsafe { 
            let mut __res : u64;
            let mut __ign1 : u64;
            let mut __ign2 : u64;
            let mut __ign3 : u64;
            let a1 : i64 = $a1 as i64;
            let a2 : i64 = $a2 as i64;
            let a3 : i64 = $a3 as i64;
            //: "=a"(__res), "=edi"(__ign1), "=esi"(__ign2),"=d"(__ign3)
            asm!(concat!("call HYPERCALL_PAGE_OFFSET + ",$name,"*32")
                : "={ax}"(__res),"={Di}"(__ign1),"={Si}"(__ign2),"={dx}"(__ign3)
                : "1"(a1), "2"(a2), "3"(a3)				
                : "memory" 
                : "volatile"
                );					
        }
    };

}

macro_rules! CONSOLEIO {
    () => { 18 };
}

const CONSOLEIO_WRITE : usize = 0;
//const CONSOELIO_READ : usize = 1;

#[no_mangle]
pub extern fn print(s : *const u8, len : usize) {
    hypercall3!(CONSOLEIO!(), CONSOLEIO_WRITE, len, s);
}

const HELLO : &'static str = "HELLO FROM RUST";

#[no_mangle]
pub extern fn say_hello_rs(){
//    let a = ['A'; 16];
    print(HELLO.as_ptr(), HELLO.len());
//    while(true) {}
}
