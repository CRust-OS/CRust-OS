use core::ptr;
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
        let EventChannel(ref port) = *self;
        let p = unsafe { ptr::read(port) };
        close(p);
    }
}

impl EventChannel {
    pub unsafe fn notify(&mut self) -> Result<(), NegErrnoval> {
        let EventChannel(ref mut port) = *self;
        match send(port) {
            NegErrnoval::ALLGOOD => { Result::Ok(()) }
            e => { Result::Err(e) }
        }
    }
}
