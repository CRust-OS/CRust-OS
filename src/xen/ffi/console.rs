const INPUT_RING_SIZE: usize = 1024;
const OUTPUT_RING_SIZE: usize = 2048;

#[repr(C)]
pub struct xencons_interface {
    pub in_         : [u8; INPUT_RING_SIZE],
    pub out         : [u8; OUTPUT_RING_SIZE],
    pub in_cons     : u32,
    pub in_prod     : u32,
    pub out_cons    : u32,
    pub out_prod    : u32
}

pub fn mod_output_ring_size(i: u32) -> u32 {
    i & ((OUTPUT_RING_SIZE as u32) - 1)
}