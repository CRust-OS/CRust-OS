use core::mem;
use xen::*;
use hypercalls::Command;
use hypercalls::event_channel_op::SubCommand;
use hypercalls::event_channel_op::Port;
use hypercalls::event_channel_op::_Port;

pub unsafe fn call(virq: Virq, vcpu: Vcpu) -> Port {
    let port = mem::uninitialized();
    let mut args = Args { virq: virq, vcpu: vcpu, port: port };
    let _result = hypercall!(
        i64,
        Command::event_channel_op,
        SubCommand::bind_virq,
        &mut args as *mut Args
    );
    Port(args.port)
}

#[repr(C)]
struct Args {
    virq: Virq,
    vcpu: Vcpu,
    /// Output
    port: _Port
}

#[repr(C)]
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


