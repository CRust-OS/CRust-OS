use hypercalls::Command;
use hypercalls::event_channel_op::SubCommand;
use hypercalls::event_channel_op::Port;
use hypercalls::event_channel_op::EventChannel;

impl Drop for EventChannel {
    fn drop(&mut self) {
        unsafe {
            call(self);
        }
    }
}

unsafe fn call(evt_chn: &mut EventChannel) {
    let &mut EventChannel(ref mut port) = evt_chn;
    let mut args = Args { port : *port };
    let _result = hypercall!(
        i64,
        Command::event_channel_op,
        SubCommand::close,
        &mut args as *mut Args
    );
}

#[repr(C)]
struct Args {
    port: Port
}
