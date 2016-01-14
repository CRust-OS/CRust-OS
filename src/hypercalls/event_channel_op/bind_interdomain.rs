use xen::DomID;
use hypercalls::Command;
use hypercalls::event_channel_op::SubCommand;
use hypercalls::event_channel_op::EventChannel;
use hypercalls::event_channel_op::Port;

pub unsafe fn call(remote_dom: DomID, remote_port: EventChannel) -> EventChannel {
    let mut args = Args { remote_dom: remote_dom, remote_port: remote_port.0, local_port: Port(0) };
    let _result = hypercall!(
        i64,
        Command::event_channel_op,
        SubCommand::bind_virq,
        &mut args as *mut Args
    );
    EventChannel(args.local_port)
}

#[repr(C)]
#[derive(Clone,Copy)]
pub struct Args {
    remote_dom: DomID,
    remote_port: Port,
    /// Output
    local_port: Port
}
