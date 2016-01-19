use ::xen::hypercalls::DomID;
use ::xen::hypercalls::event_channel_op::Port;

#[repr(C)]
#[derive(Clone,Copy)]
pub struct Args {
    pub dom: DomID,
    pub remote_dom: DomID,
    /// Output
    pub port: Port
}

