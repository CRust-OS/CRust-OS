#[repr(usize)]
#[allow(non_camel_case_types)]
pub enum SubCommand {
    map_grant_ref = 0,
    unmap_grant_ref = 1,
    setup_table = 2,
    dump_table = 3,
    tranfer = 4,
    copy = 5,
    query_size = 6,
    unmap_and_replace = 7,
    set_version = 8,
    get_status_frames = 9,
    get_version = 10,
    swap_grant_ref = 11,
    cache_flush = 12
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PFN(pub u64);

//pub mod map_grant_ref;
//pub mod unmap_grant ref;
pub mod setup_table;
//pub mod dump_table;
//pub mod transfer;
//pub mod copy;
//pub mod query_size;
//pub mod unmap_and_replace;
//pub mod set_version;
//pub mod get_status_frames;
//pub mod get_version;
//pub mod swap_grant_ref;
//pub mod cache_flush;
