use ::xen::hypercalls::*;
use ::xen::hypercalls::grant_table_op::PFN;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Args {
    pub dom             : DomID,
    pub nr_frames       : u32,
    /// Output
    pub status          : i16,      // See enum grant_status
    /// Output
    pub frame_list      : *mut PFN
}