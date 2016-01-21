#[repr(u32)]
enum xsd_sockmsg_type {
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

struct xsd_sockmsg {
    _type: xsd_sockmsg_type,
    req_id: u32,
    tx_id: u32,
    len: u32
}
