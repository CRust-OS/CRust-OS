#[allow(non_camel_case_types)]
enum Command {
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

type Port = u32;

#[repr(C)]
#[derive(Clone,Copy)]
struct Send {
    port : Port
}
