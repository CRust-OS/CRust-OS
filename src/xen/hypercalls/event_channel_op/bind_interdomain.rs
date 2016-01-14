use ::xen::hypercalls::DomID;
use ::xen::hypercalls::event_channel_op::Port;

#[repr(C)]
#[derive(Clone,Copy)]
pub struct Args {
    pub remote_dom: DomID,
    pub remote_port: Port,
    /// Output
    pub local_port: Port
}
