use ::xen::hypercalls::DomID;
use ::xen::hypercalls::grant_table_op::{Status, PFN};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Args {
    pub dom             : DomID,
    pub nr_frames       : u32,
    /// Output Parameter
    pub status          : Status,      // See enum grant_status
    /// Output Parameter
    pub frame_list      : *mut PFN
}
