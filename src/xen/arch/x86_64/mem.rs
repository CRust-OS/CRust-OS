#[inline]
pub unsafe fn mb(){
    asm!("mfence" : : : "memory" : "volatile");
}

#[inline]
pub unsafe fn wmb(){
    asm!("sfence" : : : "memory" : "volatile");
}

const PAGE_SHIFT : u64 = 12;
const VIRT_START : u64 = 0x0;
const HYPERVISOR_VIRT_START : *const u64 = 0xFFFF800000000000 as *const u64;

#[inline]
pub fn mfn_to_virt(m: u64) -> u64 {
    to_virt(mfn_to_pfn(m) << PAGE_SHIFT)
}

#[inline]
pub fn to_virt(m : u64) -> u64 {
    m + VIRT_START
}

#[inline]
pub fn mfn_to_pfn(m : u64) -> u64 {
    unsafe {
        *(HYPERVISOR_VIRT_START.offset(m as isize)) as u64
    }
}

