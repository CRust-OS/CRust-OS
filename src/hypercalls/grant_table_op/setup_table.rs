use xen;
use core::mem;
use hypercalls::Command;
use hypercalls::grant_table_op::{SubCommand};

const NR_RESERVED_ENTRIES   : u32 = 8;
const NR_GRANT_FRAMES       : u32 = 1;

#[repr(C)]
pub struct Args {
    // Input params
    dom             : xen::DomID,
    nr_frames       : u32,
    // OUT PARAMS
    status          : i16,      // See enum grant_status
    frame_list      : *mut xen::PFN
}

pub unsafe fn arch_init_gnttab(nr_grant_frames : u32) {
    // TODO: FIX
    let mut frames = [0u64; 16];
    let mut args = Args {
        dom: xen::DomID::SELF,
        nr_frames: nr_grant_frames,
        status: mem::zeroed(),
        frame_list: &mut frames[0] as *mut xen::PFN     // OK because we know we have > 0 elements
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
