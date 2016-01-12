use xen::*;

#[repr(C)]
#[derive(Clone,Copy)]
pub struct Args {
    pub vcpu: Vcpu,
    port: Port
}

