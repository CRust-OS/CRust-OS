use xen::ffi::{GuestPhysicalFrameNumber, MachineFrameNumber};
use core::ops::{Deref, DerefMut};

#[macro_use]
mod hypercall_macros;

pub mod mem;

const PAGE_SHIFT: usize = 12;

// via xen/arch/x86/xen/enlighten.c:124
const MACH2PHYS_VIRT_START: usize = 0xFFFF800000000000;
const MACH2PHYS_VIRT_END:   usize = 0xFFFF804000000000;
const MACH2PHYS_SHIFT: usize = 3;
const MACH2PHYS_NR_ENTRIES: usize = (MACH2PHYS_VIRT_END - MACH2PHYS_VIRT_START) >> MACH2PHYS_SHIFT;

#[repr(C)]
pub struct Mapping(*const [u64; MACH2PHYS_NR_ENTRIES]);
unsafe impl Sync for Mapping {}
pub static MACHINE_TO_PHYS_MAPPING: Mapping = Mapping(MACH2PHYS_VIRT_START as *const _);

impl<T> Deref for GuestPhysicalFrameNumber<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            let p = (self.0 << PAGE_SHIFT) as usize as *const Self::Target;
            &*p
        }
    }
}

impl<T> DerefMut for GuestPhysicalFrameNumber<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let p = (self.0 << PAGE_SHIFT) as usize as *mut Self::Target;
            &mut *p
        }
    }
}

impl<T> Deref for MachineFrameNumber<T> {
    type Target = GuestPhysicalFrameNumber<T>;
    fn deref(&self) -> &Self::Target {
        unsafe {
            let pfn = (*MACHINE_TO_PHYS_MAPPING.0)[self.0 as usize];
            let result = (&pfn) as *const u64 as *const GuestPhysicalFrameNumber<T>;
            &*result
        }
    }
}

impl<T> DerefMut for MachineFrameNumber<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let mut pfn = (*MACHINE_TO_PHYS_MAPPING.0)[self.0 as usize];
            let result = (&mut pfn) as *mut u64 as *mut GuestPhysicalFrameNumber<T>;
            &mut *result
        }
    }
}
