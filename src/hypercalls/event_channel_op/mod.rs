#[allow(non_camel_case_types)]
enum SubCommand {
    bind_interdomain    = 0,
    bind_virq           = 1,
    bind_pirq           = 2,
    close               = 3,
    send                = 4,
    status              = 5,
    alloc_unbound       = 6,
    bind_ipi            = 7,
    bind_vcpu           = 8,
    unmask              = 9,
    reset               = 10,
    init_control        = 11,
    expand_array        = 12,
    set_priority        = 13
}

// Port implements Drop, which is incompatible with repr(C)
// https://github.com/rust-lang/rust/issues/24585
// Use _Port when interfacing with the hypervisor
#[derive(Copy, Clone)]
struct Port(u32);
pub struct EventChannel(Port);

//pub mod bind_interdomain;
pub mod bind_virq;
//pub mod bind_pirq;
pub mod close;
//pub mod send;
//pub mod status;
//pub mod alloc_unbound;
//pub mod bind_ipi;
//pub mod bind_vcpu;
//pub mod unmask;
//pub mod reset;
//pub mod init_control;
//pub mod expand_array;
//pub mod set_priority;
