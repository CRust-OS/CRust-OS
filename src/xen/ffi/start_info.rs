use super::*;

//#[derive(Debug)] //TODO
#[repr(C)]
pub struct StartInfoPage {
    pub magic:              [u8; 32],
    pub nr_pages:           u64,
    pub shared_info:        MachineFrameNumber<shared_info::SharedInfoPage>,
    pub flags:              u32,
    pub store_mfn:          MachineFrameNumber<xenstore::xenstore_domain_interface>,
    pub store_evtchn:       Port,
    pub console:            Console,           // XXX: Rust currently doens't provde a nice way to handle C unions, right now, only doing the domU version
    pub pt_base:            u64,
    pub nr_pt_frames:       u64,
    pub mfn_list:           u64,
    pub mod_start:          u64,
    pub mod_len:            u64,
    pub cmd_line:           [u8; 1024],
    pub first_p2m_pfn:      GuestPhysicalFrameNumber<*const usize>,
    pub nr_p2m_frames:      u64,
}

#[derive(Debug)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct Console {
    pub DomU: DomU
}

#[derive(Debug)]
#[repr(C)]
pub struct DomU {
    pub mfn:        MachineFrameNumber<console::xencons_interface>,
    pub evtchn:     Port
}
