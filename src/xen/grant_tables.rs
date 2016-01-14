use core::mem;
use ::xen::hypercalls::{Command, DomID};
use ::xen::hypercalls::grant_table_op::{SubCommand, PFN};
use ::xen::hypercalls::grant_table_op::setup_table::Args;

const NR_RESERVED_ENTRIES   : u32 = 8;
const NR_GRANT_FRAMES       : u32 = 1;

pub unsafe fn arch_init_gnttab(nr_grant_frames : u32) {
    // TODO: FIX
    let frames = [0u64; 16];
    let mut args = Args {
        dom: DomID::SELF,
        nr_frames: nr_grant_frames,
        status: mem::zeroed(),
        frame_list: &mut PFN(frames[0]) as *mut PFN     // OK because we know we have > 0 elements
    };
    let _result = hypercall!(
        i64,
        Command::grant_table_op,
        SubCommand::setup_table,
        &mut args as *mut Args,
        1u32            // number of arguments: 1
    );

    //map_frames(frames) // TODO maybe - 
}

pub fn init_gnttab(){
}