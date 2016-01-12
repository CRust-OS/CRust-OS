use xen::*;

#[repr(C)]
#[derive(Clone,Copy)]
pub struct Args {
    pub dom: DomID,
    pub remote_dom: DomID,
    port: Port
}

