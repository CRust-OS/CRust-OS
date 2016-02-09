use ::xen::start_info::start_info_page;
use ::xen::event_channels::send;
use ::xen::arch::mem::*;
use ::xen::ring_buffer::{WritableRing,ReadableRing};

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

impl WritableRing for xencons_interface {
    fn output_buffer(&mut self) -> &mut [u8] {
        &mut(self.output)
    }
    fn get_output_consumer_idx(&self) -> usize {
        self.out_cons as usize
    }
    fn get_output_producer_idx(&self) -> usize {
        self.out_prod as usize
    }
    fn set_output_consumer_idx(&mut self, out_cons: usize) {
        self.out_cons = out_cons as u32;
    }
    fn set_output_producer_idx(&mut self, out_prod : usize) {
        self.out_prod = out_prod as u32;
    }
}

impl ReadableRing for xencons_interface {
    fn input_buffer(&mut self) -> &mut [u8] {
        &mut(self.input)
    }
    fn get_input_consumer_idx(&self) -> usize {
        self.in_cons as usize
    }
    fn get_input_producer_idx(&self) -> usize {
        self.in_prod as usize
    }
    fn set_input_consumer_idx(&mut self, in_cons: usize) {
        self.in_cons = in_cons as u32;
    }
    fn set_input_producer_idx(&mut self, in_prod : usize) {
        self.in_prod = in_prod as u32;
    }
}

pub unsafe fn write(s : &[u8]) {
    let intf_ptr = mfn_to_virt((*start_info_page).console.domU.mfn) as *mut xencons_interface;
    let intf = intf_ptr.as_mut().unwrap();
    intf.write_notify(s, (*start_info_page).console.domU.evtchn);
}
