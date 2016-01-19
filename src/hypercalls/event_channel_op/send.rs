use hypercalls::Command;
use hypercalls::event_channel_op::{Port, SubCommand, EventChannel};

#[repr(C)]
#[derive(Clone,Copy)]
pub struct Args {
    port: Port
}

pub unsafe fn send(prt : u32){
    let mut args = Args { port : Port(prt) };
    let _result = hypercall!(
        i64,
        Command::event_channel_op,
        SubCommand::send,
        &mut args as *mut Args
    );
}
