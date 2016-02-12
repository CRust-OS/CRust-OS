use core::mem::size_of_val;
use std::io;
use std::sync::RwLock;
use xen::arch::mem;
use ::xen::event_channels::*;

pub static CONSOLE: RwLock<Option<Console<'static>>> = RwLock::new(Option::None);

pub struct Console<'a> {
    pub interface: &'a mut xencons_interface,
    pub event_channel: EventChannel
}

const INPUT_RING_SIZE: usize = 1024;
const OUTPUT_RING_SIZE: usize = 2048;

#[repr(C)]
pub struct xencons_interface {
    input       : [u8; INPUT_RING_SIZE],           // renamed because 'in' is a keyword
    output      : [u8; OUTPUT_RING_SIZE],
    in_cons     : u32,
    in_prod     : u32,
    out_cons    : u32,
    out_prod    : u32
}

impl<'a> Console<'a> {
    fn write_byte(&mut self, byte: u8) {
        while {
            let data = (self.interface.out_prod - self.interface.out_cons) as usize;
            unsafe { self.event_channel.notify(); }
            mem::mb();
            data >= size_of_val(&self.interface.output)
        } {}

        let ring_index = mod_output_ring_size(self.interface.out_prod) as usize;
        
        self.interface.output[ring_index] = byte;
        self.interface.out_prod = self.interface.out_prod.wrapping_add(1);
    }
}

impl<'a> io::Write for Console<'a> {
    //Listing 6.4 in The Definitive Guide to the Xen Hypervisor
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for b in buf {
            if *b == '\n' as u8 {
                self.write_byte('\r' as u8);
                self.write_byte('\n' as u8);
            } else {
                self.write_byte(*b);
            }
        }
        unsafe { self.event_channel.notify(); }

        Ok(buf.len())
    }
}

fn mod_output_ring_size(i: u32) -> u32 {
    i & ((OUTPUT_RING_SIZE as u32) - 1)
}
