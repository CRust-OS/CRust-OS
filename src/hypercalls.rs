#[macro_export]
macro_rules! hypercall {
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

enum Hypercall {
    set_trap_table      = 0,
    mmu_update          = 1,
    set_gdt             = 2,
    stack_switch        = 3,
    set_callbacks       = 4,
    fpu_taskswitch      = 5,
    sched_op_compat     = 6,
    platform_op         = 7,
    set_debugreg        = 8,
    get_debugreg        = 9,
    update_descriptor   = 10,
    memory_op           = 12,
    multicall           = 13,
    update_va_mapping   = 14,
    set_timer_op        = 15,
    event_channel_op_compat = 16,
    xen_version         = 17,
    console_io          = 18,
    physdev_op_compat   = 19,
    grant_table_op      = 20,
    vm_assist           = 21,
    update_va_mapping_otherdomain = 22,
    iret                = 23,
    vcpu_op             = 24,
    set_segment_base    = 25,
    mmuext_op           = 26,
    xsm_op              = 27,
    nmi_op              = 28,
    sched_op            = 29,
    callback_op         = 30,
    xenoprof_op         = 31,
    event_channel_op    = 32,
    physdev_op          = 33,
    hvm_op              = 34,
    sysctl              = 35,
    domctl              = 36,
    kexec_op            = 37,
    tmem_op             = 38,
    xc_reserved_op      = 39,
    xen_pmu_op          = 40,
    arch_0              = 48,
    arch_1              = 49,
    arch_2              = 50,
    arch_3              = 51,
    arch_4              = 52,
    arch_5              = 53,
    arch_6              = 54,
    arch_7              = 55
}

#[macro_export]
macro_rules! CMD {
    ( CONSOLEIO ) => { 18 };
    ( SCHEDOP ) => { 29 };
    ( EVENT_CHANNEL_OP ) => { 32 };
}

#[macro_export]
macro_rules! CONSOLEIO {
    ( WRITE ) => {0};
    ( READ ) => {1};
}

#[macro_export]
macro_rules! SCHEDOP {
    (BLOCK) => {1};
    (foo) => {1}
}

#[macro_export]
macro_rules! EVENTCHANOP {
    ( BIND_INTERDOMAIN) => {0};
    ( BIND_VIRTQ ) => {1};
    ( BIND_PIRQ ) => {2};
    ( CLOSE ) => {3};
    ( SEND ) => {4};
    ( STATUS ) => {5};
    ( ALLOC_UNBOUND ) => {6};
    ( BIND_IPI ) => {7};
    ( BIND_VCPU ) => {8};
    ( UNMASK ) => {9};
    ( RESET ) => {10};
    (INIT_CONTROL) => {11};
    (EXPAND_ARRAY) => {12};
    (SET_PRIORITY) => {13};
}

// TODO: Use AsRef<str> 
#[no_mangle]
pub extern fn print(s : *const u8, len : usize) {
    hypercall!(CMD!(CONSOLEIO), CONSOLEIO!(WRITE), len, s);
}


#[no_mangle]
pub extern fn say_hello(){
    let hello = "hello FROM RUST";
    print(hello.as_ptr(), hello.len());
}

pub extern fn block(){
    hypercall!(CMD!(SCHEDOP), SCHEDOP!(BLOCK));
}
