use core::fmt;
use core::marker::PhantomData;

#[macro_use]
mod arch;
pub use self::arch::*;

pub mod console;
pub mod hypercalls;
pub mod shared_info;
pub mod start_info;
pub mod xenstore;

#[derive(Debug)]
#[repr(C)]
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Port(u32);

#[derive(Debug)]
pub struct EventChannel(Port);

impl Drop for EventChannel {
    fn drop(&mut self) {
        let EventChannel(port) = *self;
        hypercalls::event_channel_op::close(port);
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct XenGuestHandle<T>(*mut T);

#[repr(C)]
pub struct MachineFrameNumber<T>(u64, PhantomData<T>);
#[repr(C)]
pub struct GuestPhysicalFrameNumber<T>(u64, PhantomData<T>);

impl<T> fmt::Debug for MachineFrameNumber<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MachineFrameNumber ({:?})", self.0)
    }
}

impl<T> fmt::Debug for GuestPhysicalFrameNumber<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GuestPhysicalFrameNumber ({:?})", self.0)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct PageFrameNumber(u64);

#[derive(Debug)]
#[repr(C)]
pub struct Vcpu(u32);
