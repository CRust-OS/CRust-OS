use ::xen::hypercalls::Vcpu;
use ::xen::hypercalls::event_channel_op::Port;

#[repr(C)]
#[derive(Clone,Copy)]
pub struct Args {
    pub vcpu: Vcpu,
    /// Output
    pub port: Port
}

