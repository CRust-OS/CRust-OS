pub struct EventChannel(u32);

impl Drop for EventChannel {
    fn drop (&mut self) {
        use ::xen::hypercalls::Command;
        use ::xen::hypercalls::event_channel_op::{Port, SubCommand};
        use ::xen::hypercalls::event_channel_op::close::Args;
        unsafe {
            let &mut EventChannel(ref mut port) = self;
            let mut args = Args { port : Port(*port) };
            let _result = hypercall!(
                i64,
                Command::event_channel_op,
                SubCommand::close,
                &mut args as *mut Args
            );
        }
    }
}