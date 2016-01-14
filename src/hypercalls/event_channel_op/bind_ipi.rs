use xen::Vcpu;
use hypercalls::Command;
use hypercalls::event_channel_op::SubCommand;
use hypercalls::event_channel_op::EventChannel;
use hypercalls::event_channel_op::Port;

#[repr(C)]
#[derive(Clone,Copy)]
pub struct Args {
    vcpu: Vcpu,
    port: Port
}

pub unsafe fn call(vcpu: Vcpu) -> EventChannel {
    let mut args = Args { vcpu: vcpu, port: Port(0) };
    let _result = hypercall!(
        i64,
        Command::event_channel_op,
        SubCommand::bind_ipi,
        &mut args as *mut Args
    );
    EventChannel(args.port)
}

