extern {
    static start_info_page: *const start_info;
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct start_info {
    pub magic:          [u8; 32],
    pub nr_pages:           u64,
    pub shared_info:        u64,
    pub flags:              u32,
    pub store_pfn_t:        u64,
    pub store_evtchn:       u32,
    pub domU:               domU,           // XXX: Rust currently doens't provde a nice way to handle C unions, right now, only doing the domU version
    pub pt_base:            u64,
    pub nr_pt_frames:       u64,
    pub mfn_list:           u64,
    pub mod_start:          u64,
    pub mod_len:            u64,
    pub cmd_line:           [u8; 1024],
    pub fist_p2m_pfn:       u64,
    pub nr_p2r_frames:      u64
}

#[repr(C)]
pub struct domU{
    pub mfn:        u64,
    pub evtchn:     u32
}
