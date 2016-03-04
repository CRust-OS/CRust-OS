use core::fmt;
use core::mem::size_of_val;
use std::io;
use std::io::Write;
use std::sync::RwLock;
use xen::ffi::mem;
use xen::event_channels::*;
use xen::ffi::console::*;

static CONSOLE: RwLock<Option<Console<'static>>> = RwLock::new(Option::None);

pub fn initialize(console: &'static mut xencons_interface, event_channel: EventChannel) {
    *(CONSOLE.write()) = Some(Console { interface: console, event_channel: event_channel})
}

pub struct STDOUT;

impl fmt::Write for STDOUT {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        use std::io::Write;
        let result =
            CONSOLE
                .write()
                .as_mut()
                .unwrap()
                .write(s.as_bytes());
        match result {
            Ok(_) => { Result::Ok(()) }
            Err(_) => { Result::Err(fmt::Error) }
        }
    }
}

pub struct Console<'a> {
    interface: &'a mut xencons_interface,
    event_channel: EventChannel
}

impl<'a> Console<'a> {
     fn write_byte(&mut self, byte: u8) {
        while {
            let data = (self.interface.out_prod - self.interface.out_cons) as usize;
            let _result = unsafe { self.event_channel.notify() };
            mem::mb();
            data >= size_of_val(&self.interface.out)
        } {}

        let ring_index = mod_output_ring_size(self.interface.out_prod) as usize;
        
        self.interface.out[ring_index] = byte;
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
        let _result = unsafe { self.event_channel.notify() };

        Ok(buf.len())
    }
}
