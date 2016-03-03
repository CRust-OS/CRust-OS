use xen::ffi::{GuestPhysicalFrameNumber, MachineFrameNumber};
use core::ops::{Deref, DerefMut};
use core::ptr;
use std::ops::DerefMove;

#[macro_use]
mod hypercall_macros;

pub mod mem;

const PAGE_SHIFT: usize = 12;

// via xen/arch/x86/xen/enlighten.c:124
const MACH2PHYS_VIRT_START: usize = 0xFFFF800000000000;
const MACH2PHYS_VIRT_END:   usize = 0xFFFF804000000000;
const MACH2PHYS_SHIFT: usize = 3;
const MACH2PHYS_NR_ENTRIES: usize = (MACH2PHYS_VIRT_END - MACH2PHYS_VIRT_START) >> MACH2PHYS_SHIFT;
static mut machine_to_phys_mapping: *const [usize; MACH2PHYS_NR_ENTRIES] = MACH2PHYS_VIRT_START as *const _;

impl<T> Deref for GuestPhysicalFrameNumber<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            let p = (self.0 << PAGE_SHIFT) as usize as *const _;
            &*p
        }
    }
}

impl<T> DerefMut for GuestPhysicalFrameNumber<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let p = (self.0 << PAGE_SHIFT) as usize as *mut _;
            &mut *p
        }
    }
}

impl<T> DerefMove for GuestPhysicalFrameNumber<T> {
    fn deref_move(self) -> Self::Target {
        unsafe {
            let p = (self.0 << PAGE_SHIFT) as usize as *mut Self::Target;
            ptr::read(p)
        }
    }
}


impl<T> Deref for MachineFrameNumber<T> {
    type Target = GuestPhysicalFrameNumber<T>;
    fn deref(&self) -> &Self::Target {
        unsafe {
            let p = (*machine_to_phys_mapping)[self.0 as usize] as *const _;
            &*p
        }
    }
}

impl<T> DerefMut for MachineFrameNumber<T> {
    fn deref_mut<'a>(&mut self) -> &mut Self::Target {
        unsafe {
            let p = (*machine_to_phys_mapping)[self.0 as usize] as *mut Self::Target;
            &mut *p
        }
    }
}

impl<T> DerefMove for MachineFrameNumber<T> {
    fn deref_move(self) -> Self::Target {
        unsafe {
            let p = (*machine_to_phys_mapping)[self.0 as usize] as *mut Self::Target;
            ptr::read(p)
        }
    }
}

