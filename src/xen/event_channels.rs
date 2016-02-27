pub use xen::ffi::EventChannel;

use xen::ffi::hypercalls::event_channel_op::send;

impl EventChannel {
    pub unsafe fn notify(&self) {
        send(&self);
    }
}
