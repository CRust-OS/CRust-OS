const XENSTORE_RING_SIZE : usize = 1024;

#[repr(C)]
pub struct xenstore_domain_interface {
    pub req: [u8; XENSTORE_RING_SIZE],
    pub rsp: [u8; XENSTORE_RING_SIZE],
    pub req_cons: u32,
    pub req_prod: u32,
    pub rsp_cons: u32,
    pub rsp_prod: u32
}

#[derive(Debug)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct xsd_sockmsg {
    pub type_: xsd_sockmsg_type,
    pub req_id: u32,
    pub tx_id: u32,
    pub len: u32
}

#[derive(Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum xsd_sockmsg_type {
    Debug       = 0,
    Directory   = 1,
    Read        = 2,
    GetPerms    = 3,
    Watch       = 4,
    Unwatch     = 5,
    TransactionStart = 6,
    TransactionEnd = 7,
    Introduce   = 8,
    Release     = 9,
    GetDomainPath = 10,
    Write       = 11,
    Mkdir       = 12,
    Rm          = 13,
    SetPerms    = 14,
    WatchEvent  = 15,
    Error       = 16,
    IsDomainIntroduced = 17,
    Resume      = 18,
    SetTarget   = 19,
    Restrict    = 20,
    ResetWatches  = 21,

    Invalid     = 0xffff /* Guaranteed to remain an invalid type */
}

pub fn mod_ring_size(i: u32) -> u32 {
    i & ((XENSTORE_RING_SIZE as u32) - 1)
}