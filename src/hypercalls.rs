macro_rules! hypercall0 {
    ( $name : expr ) => {
        unsafe { 
            let mut __res : u64 = 0;
            asm!(concat!("call HYPERCALL_PAGE + ",$name,"*32")
                : "={ax}"(__res)
                : 
                : "memory" 
                : "volatile"
                );					
        }
    };
}

macro_rules! hypercall1 {
    ( $name : expr, $a1 : expr ) => {
        unsafe { 
            let mut __res : u64 = 0;
            let mut __ign1 : u64 = 0;
            let a1 : i64 = $a1 as i64;
            asm!(concat!("call HYPERCALL_PAGE + ",$name,"*32")
                : "={ax}"(__res),"={Di}"(__ign1)
                : "1"(a1)
                : "memory" 
                : "volatile"
                );					
        }
    };
}

macro_rules! hypercall2 {
    ( $name : expr, $a1 : expr, $a2 : expr ) => {
        unsafe { 
            let mut __res : u64 = 0;
            let mut __ign1 : u64 = 0;
            let mut __ign2 : u64 = 0;
            let a1 : i64 = $a1 as i64;
            let a2 : i64 = $a2 as i64;
            asm!(concat!("call HYPERCALL_PAGE + ",$name,"*32")
                : "={ax}"(__res),"={Di}"(__ign1),"={Si}"(__ign2)
                : "1"(a1), "2"(a2)
                : "memory" 
                : "volatile"
                );					
        }
    };
}

macro_rules! hypercall3 {
    ( $name : expr, $a1 : expr, $a2 : expr, $a3 : expr ) => {
        unsafe { 
            let mut __res : u64 = 0;
            let mut __ign1 : u64 = 0;
            let mut __ign2 : u64 = 0;
            let mut __ign3 : u64 = 0;
            let a1 : i64 = $a1 as i64;
            let a2 : i64 = $a2 as i64;
            let a3 : i64 = $a3 as i64;
            //: "=a"(__res), "=edi"(__ign1), "=esi"(__ign2),"=d"(__ign3)
            asm!(concat!("call HYPERCALL_PAGE + ",$name,"*32")
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
const CONSOELIO_READ : usize = 1;

#[no_mangle]
pub extern fn print(s : *const u8, len : usize) {
    hypercall3!(CONSOLEIO!(), CONSOLEIO_WRITE, len, s);
}


#[no_mangle]
pub extern fn say_hello(){
    let hell = "hell FROM RUST";
    print(hell.as_ptr(), hell.len());
}

macro_rules! SCHEDOP {
    () => {29};
}

const SCHEDOP_BLOCK : usize = 1;
pub extern fn block(){
    hypercall1!(SCHEDOP!(), SCHEDOP_BLOCK);
}
