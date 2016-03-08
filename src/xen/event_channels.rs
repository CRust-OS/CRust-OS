pub use xen::ffi::Port;
use xen::ffi::hypercalls::NegErrnoval;

use xen::ffi::hypercalls::event_channel_op::*;

#[derive(Debug)]
pub struct EventChannel(Port);

impl EventChannel {
    pub fn new(p : Port) -> EventChannel {
        EventChannel(p)
    }
}

impl Drop for EventChannel {
    fn drop(&mut self) {
        let EventChannel(port) = *self;
        close(port);
    }
}

impl EventChannel {
    pub unsafe fn notify(&self) -> Result<(), NegErrnoval> {
        let EventChannel(port) = *self;
        match send(&port) {
            NegErrnoval::ALLGOOD => { Result::Ok(()) }
            e => { Result::Err(e) }
        }
    }
}
