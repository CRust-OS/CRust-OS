use ::xen::hypercalls::*;
use ::xen::hypercalls::event_channel_op::*;

pub struct EventChannel(u32);

impl Drop for EventChannel {
    fn drop (&mut self) {
        unsafe {
            let &mut EventChannel(ref mut port) = self;
            let mut args = close::Args { port : Port(*port) };
            let _result = hypercall!(
                i64,
                Command::event_channel_op,
                SubCommand::close,
                &mut args as *mut close::Args
            );
        }
    }
}


// TODO: FIX, arg shouldn't be a u32
pub unsafe fn send(prt : u32){
    let mut args = send::Args { port : Port(prt) };
    let _result = hypercall!(
        i64,
        Command::event_channel_op,
        SubCommand::send,
        &mut args as *mut _
    );
}
