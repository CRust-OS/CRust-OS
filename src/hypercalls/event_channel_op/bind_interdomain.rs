use xen::*;

#[repr(C)]
#[derive(Clone,Copy)]
pub struct Args {
    pub remote_dom: DomID,
    pub remote_port: Port,
    local_port: Port
}
