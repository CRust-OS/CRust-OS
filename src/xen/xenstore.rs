use std::io;
use core::*;
use core::iter::*;
use core::mem::*;
use core::sync::atomic::*;
use std::sync::RwLock;
use xen::arch::mem::*;
use xen::event_channels::*;
use xen::start_info::start_info_page;
use alloc::raw_vec::RawVec;
use collections::{String, Vec};

pub static XENSTORE: RwLock<Option<XenStore<'static>>> = RwLock::new(Option::None);


pub struct XenStore<'a> {
    interface: &'a mut xenstore_domain_interface,
    event_channel: EventChannel
}

static mut req_counter : AtomicIsize = AtomicIsize::new(1);

#[repr(C)]
struct xenstore_domain_interface {
    req: [u8; XENSTORE_RING_SIZE],
    rsp: [u8; XENSTORE_RING_SIZE],
    req_cons: u32,
    req_prod: u32,
    rsp_cons: u32,
    rsp_prod: u32
}

const XENSTORE_RING_SIZE : usize = 1024;

impl io::Write for xenstore_domain_interface {
    //Listing 8.4 in The Definitive Guide to the Xen Hypervisor
    fn write(&mut self, buf: &[u8]) -> Result<usize, &'static str> {
        if buf.len() > XENSTORE_RING_SIZE {
            Result::Err("Too much data!")
        } else {
            let mut i = self.req_prod;
            let result = buf.len();

            for &b in buf {
                while {
                    let data = i - self.req_cons;
                    mb();
                    data >= (XENSTORE_RING_SIZE as u32)
                } {}
                let ring_index = mod_ring_size(i) as usize;
                self.req[ring_index] = b;
                i = i + 1;
            }

            wmb();
            self.req_prod = i;
            Ok(result)
        }
    }
}

impl io::Read for xenstore_domain_interface {
    //Listing 8.5 in The Definitive Guide to the Xen Hypervisor
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, &'static str> {
        let mut i = self.rsp_cons;
        let result = buf.len();

        for b in buf {
            while {
                let data = self.rsp_prod - i;
                mb();
                data == 0
            } {}
            let ring_index = mod_ring_size(i) as usize;
            *b = self.rsp[ring_index];
            i = i + 1;
        }
        self.rsp_cons = i;
        Ok(result)
    }
}

impl xenstore_domain_interface {
    //Listing 8.6 in The Definitive Guide to the Xen Hypervisor
    unsafe fn ignore(&mut self, bytes: usize) {
        use std::io::Read;
        if bytes != 0 {
            let vec = RawVec::<u8>::with_capacity(bytes);
            let slice = slice::from_raw_parts_mut(vec.ptr(), bytes);
            let _ignored = self.read(slice);
        }
    }
}

impl<'a> XenStore<'a> {

    //Listing 8.7 in The Definitive Guide to the Xen Hypervisor
    pub unsafe fn write(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
        use std::io::Write;
        let req_id = req_counter.fetch_add(1, Ordering::Relaxed) as u32;
        let msg = xsd_sockmsg {
            _type: xsd_sockmsg_type::Write,
            req_id: req_id,
            tx_id: 0,
            len: (key.len() + value.len() + 2) as u32
        };
        let msg_slice = slice::from_raw_parts(&msg as *const _ as *const u8, size_of::<xsd_sockmsg>());
        try!(self.interface.write(msg_slice));
        try!(self.interface.write(key.as_bytes()));
        try!(self.interface.write("\0".as_bytes()));
        try!(self.interface.write(value.as_bytes()));
        try!(self.interface.write("\0".as_bytes()));
        self.event_channel.notify();
        Result::Ok(())
    }

    //Listing 8.8 in The Definitive Guide to the Xen Hypervisor
    pub unsafe fn read(&mut self, key: &str) -> Result<String, &'static str> {
        use std::io::{Write, Read};
        let req_id = req_counter.fetch_add(1, Ordering::Relaxed) as u32;
        let mut msg = xsd_sockmsg {
            _type: xsd_sockmsg_type::Read,
            req_id: req_id,
            tx_id: 0,
            len: (key.len() + 1) as u32
        };
        let msg_slice = slice::from_raw_parts_mut(&mut msg as *mut _ as *mut u8, size_of::<xsd_sockmsg>());
        try!(self.interface.write(msg_slice));
        try!(self.interface.write(key.as_bytes()));
        try!(self.interface.write("\0".as_bytes()));
        self.event_channel.notify();

        self.interface.read(msg_slice).ok();
        if msg.req_id == req_id && msg.tx_id == 0 {
            let mut result_vec = Vec::with_capacity(msg.len as usize);
            result_vec.resize(msg.len as usize, '\0' as u8);
            self.interface.read(result_vec.as_mut_slice()).ok();
            Result::Ok(String::from_utf8(result_vec).ok().unwrap())
        } else {
            self.interface.ignore(msg.len as usize);
            Result::Err("Received a reply for the wrong request ID")
        }
    }
}

fn mod_ring_size(i: u32) -> u32 {
    i & ((XENSTORE_RING_SIZE as u32) - 1)
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
struct xsd_sockmsg {
    _type: xsd_sockmsg_type,
    req_id: u32,
    tx_id: u32,
    len: u32
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
enum xsd_sockmsg_type {
    Debug       = 0,
    Directory   = 1,
    Read        = 2,
    GetPerms    = 3,
    Watch       = 4,
    Unwatch     = 5,
    TransactionStart = 6,
    TransactionEnd = 7,
    Introduce   = 8,
    Release     = 9,
    GetDomainPath = 10,
    Write       = 11,
    Mkdir       = 12,
    Rm          = 13,
    SetPerms    = 14,
    WatchEvent  = 15,
    Error       = 16,
    IsDomainIntroduced = 17,
    Resume      = 18,
    SetTarget   = 19,
    Restrict    = 20,
    ResetWatches  = 21,

    Invalid     = 0xffff /* Guaranteed to remain an invalid type */
}

pub unsafe fn initialize() {
    let xenstore_ptr = XENSTORE.write();
    *xenstore_ptr = Some (XenStore {
        interface: &mut *(mfn_to_virt(start_info_page.store_mfn) as *mut _),
        event_channel: EventChannel(start_info_page.store_evtchn)
    });
}
