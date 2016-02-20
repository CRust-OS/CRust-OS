use super::{Status, GrantTableRef, GrantHandle};
use ::xen::hypercalls::DomID;

#[repr(C)]
#[derive(Clone, Copy)]
struct Args {
    // Input Parameters
    host_addr       : u64,
    flags           : u32,
    grant_ref       : GrantTableRef,
    dom             : DomID,

    // Output Parameters
    status          : Status,
    handle          : GrantHandle,
    dev_bus_addr    : u64
}
