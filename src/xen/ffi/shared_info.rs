#[repr(C)]
pub struct SharedInfoPage {
    pub vcpu_info:          [VcpuInfo; 32],
    pub evtchn_pending:     [u64; 64],
    pub evtchn_mask:        [u64; 64],
    pub wc_version:         u32,
    pub wc_sec:             u32,
    pub wc_nsec:            u32,
    pub arch_shared_info:   ArchSharedInfo
}

#[repr(C)]
pub struct ArchSharedInfo {
    pub max_pfn:                        u64,
    pub pfn_to_mfn_frame_list_list:     u64,
    pub nmi_reason:                     u64,
    pub p2m_cr3:                        u64,
    pub p2m_vaddr:                      u64,
    pub p2m_generation:                 u64
}

#[repr(C)]
pub struct VcpuInfo {
    pub evtchn_upcall_pending:          u8,
    pub evtchn_upcall_mask:             u8,
    pub evtchn_pending_sel:             u64,
    pub arch:                           ArchVcpuInfo,
    pub time:                           VcpuTimeInfo
}

#[repr(C)]
pub struct ArchVcpuInfo {
    pub cr2: u64,
    pub pad: u64
}

#[repr(C)]
pub struct VcpuTimeInfo {
    pub version:            u32,
    pub pad0:               u32,
    pub tsc_timestamp:      u64,
    pub system_time:        u64,
    pub tsc_to_system_mul:  u32,
    pub tsc_shift:          u8,
    pub pad1:               [u8; 3]
}
