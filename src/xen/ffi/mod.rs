use core::fmt;
use core::ptr;
//use core::ops::{Deref, DerefMut};
use core::marker::PhantomData;

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

impl<T> fmt::Debug for MachineFrameNumber<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MachineFrameNumber ({:?})", self.0)
    }
}

/*
// XXX review, extract, and document
impl<T> Deref for MachineFrameNumber<T> {
    type Target = T;
    fn deref(&self) -> &T {
        let &MachineFrameNumber(page_number, _) = self;
        unsafe {
            let ptr = (page_number << 12) as usize as *const T;
            &*ptr
        }
    }
}

// XXX review, extract, and document
impl<T> DerefMut for MachineFrameNumber<T> {
    fn deref_mut(&mut self) -> &mut T {
        let &mut MachineFrameNumber(page_number, _) = self;
        unsafe {
            let ptr = (page_number << 12) as usize as *mut T;
            &mut *ptr
        }
    }
}
*/

// XXX review, extract, and document
// TODO switch to DerefMove once it becomes available
impl<T> MachineFrameNumber<T> {
    pub fn deref(self) -> T {
        let MachineFrameNumber(page_number, _) = self;
        let ptr = (page_number << 12) as usize as *mut T;
        unsafe {
            ptr::read(ptr)
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct PageFrameNumber(u64);

#[derive(Debug)]
#[repr(C)]
pub struct Vcpu(u32);