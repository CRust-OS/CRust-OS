use core::mem;
use hypercalls::Command;
use hypercalls::event_channel_op::SubCommand;
use hypercalls::event_channel_op::Port;
use hypercalls::event_channel_op::_Port;

impl Drop for Port {
    fn drop(&mut self) {
        unsafe {
            call(self);
        }
    }
}

unsafe fn call(port: &mut Port) {
    // the mem::swap dance here is going to be unnecessary once Rust has drop-by-value
    // https://internals.rust-lang.org/t/pre-rfc-allow-by-value-drop/1845
    let mut args = Args { port: mem::uninitialized() };
    let &mut Port(ref mut port_) = port;
    mem::swap(port_, &mut args.port);
    let _result = hypercall!(
        i64,
        Command::event_channel_op,
        SubCommand::close,
        &mut args as *mut Args
    );
    mem::swap(port_, &mut args.port);
}

#[repr(C)]
struct Args {
    port: _Port
}
