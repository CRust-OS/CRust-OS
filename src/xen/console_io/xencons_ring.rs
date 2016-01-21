use start_info_page;
use ::xen::event_channels::send;
use ::xen::arch::mem::*;

type XENCONS_RING_IDX = u32;

#[repr(C)]
struct xencons_interface {
    input       : [u8; 1024],           // renamed because 'in' is a keyword
    output      : [u8; 2048],
    in_cons     : XENCONS_RING_IDX,
    in_prod     : XENCONS_RING_IDX,
    out_cons    : XENCONS_RING_IDX,
    out_prod    : XENCONS_RING_IDX
}

const PAGE_SHIFT : u64 = 12;
const VIRT_START : u64 = 0x0;
const HYPERVISOR_VIRT_START : *const u64 = 0xFFFF800000000000 as *const u64;

#[inline]
fn mfn_to_virt(m: u64) -> u64 {
    to_virt(mfn_to_pfn(m) << PAGE_SHIFT)
}

#[inline]
fn to_virt(m : u64) -> u64 {
    m + VIRT_START
}

#[inline]
fn mfn_to_pfn(m : u64) -> u64 {
    unsafe {
        *(HYPERVISOR_VIRT_START.offset(m as isize)) as u64
    }
}

pub unsafe fn write(s : &[u8]) {
    let mut sent = 0usize;

    let intf = mfn_to_virt((*start_info_page).domU.mfn) as *mut xencons_interface;

    let cons = (*intf).out_cons as usize;
    let mut prod = (*intf).out_prod as usize;

    mb();

    while (sent < s.len()) && ((prod - cons) < (*intf).output.len()) {
        let idx = prod & ((*intf).output.len() - 1); // mask the index to make sure we don't overflow
        prod = prod + 1;
        (*intf).output[idx] = s[sent];
        sent = sent + 1;
    }

    wmb();

    (*intf).out_prod = prod as u32;

    send((*start_info_page).domU.evtchn);
}
