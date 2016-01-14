use hypercalls::Command;
use hypercalls::event_channel_op::SubCommand;
use hypercalls::event_channel_op::EventChannel;
use hypercalls::event_channel_op::Port;

pub unsafe fn call(pirq: Pirq) -> EventChannel {
    let flags: Flags = Flags::empty(); // TODO
    let mut args = Args { pirq: pirq, flags: flags, port: Port(0) };
    let _result = hypercall!(
        i64,
        Command::event_channel_op,
        SubCommand::bind_pirq,
        &mut args as *mut Args
    );
    EventChannel(args.port)
}

#[repr(C)]
struct Args {
    pirq: Pirq,
    flags: Flags,
    /// Output
    port: Port
}

//#[repr(C)]
//#[derive(Clone, Copy)]
pub enum Pirq {
}

bitflags! {
    flags Flags: u32 {
        const WILL_SHARE = 0b1
    }
}
