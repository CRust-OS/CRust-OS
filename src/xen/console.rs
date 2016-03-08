use core::fmt;
use core::mem::size_of_val;
use core::ptr::Unique;
use std::io;
use std::io::Write;
use std::sync::RwLock;
use xen::ffi::mem;
use xen::event_channels::*;
use xen::ffi::console::*;

static CONSOLE: RwLock<Option<Console>> = RwLock::new(Option::None);

pub fn initialize(console: Unique<xencons_interface>, event_channel: EventChannel) {
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

pub struct Console {
    interface: Unique<xencons_interface>,
    event_channel: EventChannel
}

impl Console {
     fn write_byte(&mut self, byte: u8) {
        let interface = unsafe { self.interface.get_mut() };
        while {
            let data = (interface.out_prod - interface.out_cons) as usize;
            let _result = unsafe { self.event_channel.notify() };
            mem::mb();
            data >= size_of_val(&interface.out)
        } {}

        let ring_index = mod_output_ring_size(interface.out_prod) as usize;
        
        interface.out[ring_index] = byte;
        interface.out_prod = interface.out_prod.wrapping_add(1);
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
        let _result = unsafe { self.event_channel.notify() };

        Ok(buf.len())
    }
}
