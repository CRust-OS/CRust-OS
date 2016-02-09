use ::xen::hypercalls::Vcpu;
use ::xen::hypercalls::event_channel_op::Port;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Args {
    pub virq: Virq,
    pub vcpu: Vcpu,
    /// Output
    pub port: Port
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Virq {
    Timer       = 0,
    Debug       = 1,
    Console     = 2,
    DomExc      = 3,
    Tbuf        = 4,
    Debugger    = 6,
    Xenoprof    = 7,
    ConRing     = 8,
    PcpuState   = 9,
    MemEvent    = 10,
    XcReserved  = 11,
    Enomem      = 12,
    Xenpmu      = 13,
    Arch0       = 16,
    Arch1       = 17,
    Arch2       = 18,
    Arch3       = 19,
    Arch4       = 20,
    Arch5       = 21,
    Arch6       = 22,
    Arch7       = 23
}


