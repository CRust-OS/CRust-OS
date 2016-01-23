use ::xen::hypercalls::event_channel_op::Port;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Args {
    pub port: Port
}
