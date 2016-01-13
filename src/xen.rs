#[repr(C)]
#[derive(Clone, Copy)]
pub struct DomID(u16);

impl DomID {
    pub const FIRST_RESERVED: DomID = DomID(0x7FF0);
    pub const SELF: DomID           = DomID(0x7FF0);
    pub const IO: DomID             = DomID(0x7FF1);
    pub const XEN: DomID            = DomID(0x7FF2);
    pub const COW: DomID            = DomID(0x7FF3);
    pub const INVALID: DomID        = DomID(0x7FF4);
    pub const IDLE: DomID           = DomID(0x7FFF);
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vcpu(u32);

pub type PFN = u64;
