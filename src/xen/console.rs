use core::mem::size_of_val;
use std::io;
use std::io::Write;
use std::sync::RwLock;
use xen::arch::mem;
use xen::event_channels::*;
use xen::ffi::console::*;

pub static CONSOLE: RwLock<Option<Console>> = RwLock::new(Option::None);

pub struct Console {
    pub interface: xencons_interface,
    pub event_channel: EventChannel
}

impl Console {
    fn write_byte(&mut self, byte: u8) {
        while {
            let data = (self.interface.out_prod - self.interface.out_cons) as usize;
            unsafe { self.event_channel.notify(); }
            mem::mb();
            data >= size_of_val(&self.interface.out)
        } {}

        let ring_index = mod_output_ring_size(self.interface.out_prod) as usize;
        
        self.interface.out[ring_index] = byte;
        self.interface.out_prod = self.interface.out_prod.wrapping_add(1);
    }
}

impl io::Write for Console {
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

pub unsafe fn initialize(interface: xencons_interface, event_channel: EventChannel) {
    let console = CONSOLE.write();
    *console = Some (Console {
        event_channel: event_channel,
        interface: interface
    })
}