use xen::*;

#[repr(C)]
#[derive(Clone,Copy)]
pub struct Args {
    port: Port
}
