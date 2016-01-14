extern {
    static start_info_page: *const start_info;
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct start_info {
    pub magic:          [u8; 32],
    nr_pages:           u64,
    shared_info:        u64,
    flags:              u32,
    store_pfn_t:        u64,
    store_evtchn:       u64,
    domU:               domU,           // XXX: Rust currently doens't provde a nice way to handle C unions, right now, only doing the domU version
    pt_base:            u64,
    nr_pt_frames:       u64,
    mfn_list:           u64,
    mod_start:          u64,
    mod_len:            u64,
    cmd_line:           [u8; 1024],
    fist_p2m_pfn:       u64,
    nr_p2r_frames:      u64
}

#[repr(C)]
pub struct domU{
    mfn:        u64,
    evtchn:     u64
}
