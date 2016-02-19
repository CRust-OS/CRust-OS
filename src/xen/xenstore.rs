use std::io::{self, Write, Read};
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
use xen::console_io::STDOUT;

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

impl Write for xenstore_domain_interface {
    //Listing 8.4 in The Definitive Guide to the Xen Hypervisor
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() > XENSTORE_RING_SIZE {
            Result::Err(io::Error::new(io::ErrorKind::InvalidData, "Too much data!"))
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

impl Read for xenstore_domain_interface {
    //Listing 8.5 in The Definitive Guide to the Xen Hypervisor
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
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
    unsafe fn send(&mut self, type_: xsd_sockmsg_type, params: &[&str]) -> io::Result<(xsd_sockmsg_type, String)> {
        use core::fmt::Write;
        let req_id = req_counter.fetch_add(1, Ordering::Relaxed) as u32;
        // params are passed null-terminated
        let len = params.iter().fold (0, |acc, &x| acc + x.len() + 1) as u32;
        let msg = xsd_sockmsg {
            type_: type_,
            req_id: req_id,
            tx_id: 0,
            len: len
        };
        writeln!(STDOUT, "Sending {:?}: {:?}", msg, params).unwrap();
        let msg_slice = slice::from_raw_parts(&msg as *const _ as *const u8, size_of::<xsd_sockmsg>());
        try!(self.interface.write(msg_slice));

        for p in params {
            try!(self.interface.write(p.as_bytes()));
            try!(self.interface.write("\0".as_bytes()));
        }

        self.event_channel.notify();

        let mut response: xsd_sockmsg = uninitialized();
        let response_slice = slice::from_raw_parts_mut(&mut response as *mut _ as *mut u8, size_of::<xsd_sockmsg>());
        try!(self.interface.read(response_slice));
        
        match (response.req_id, response.tx_id) {
            (req_id, 0) if req_id == msg.req_id => {
                let mut result_vec = Vec::with_capacity(response.len as usize);
                result_vec.resize(response.len as usize, 0);
                self.interface.read(result_vec.as_mut_slice()).ok();
                let result = String::from_utf8(result_vec).unwrap();
                writeln!(STDOUT, "Receiving {:?}: {:?}", response, result).unwrap();
                Result::Ok((response.type_, result))
            }
            (_, 0) => {
                Result::Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Received a reply with a non-zero transaction ID (expected 0, actual {})",  msg.req_id)
                ))
            }
            (req_id, _) => {
                Result::Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Received a reply for the wrong request ID (expected {}, actual {})", req_id, msg.req_id)
                ))
            }
        }
    }

    //Listing 8.7 in The Definitive Guide to the Xen Hypervisor
    pub unsafe fn write(&mut self, key: &str, value: &str) -> io::Result<()> {
        use core::fmt::Write;
        let result = try!(self.send(xsd_sockmsg_type::Write, &[key, value]));
        
        match result {
            (xsd_sockmsg_type::Error, ref s) if s == "EACCES\0" => {
                Result::Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    format!("Can't write to key {}: access denied", key)
                ))
            }
            (xsd_sockmsg_type::Error, s) => {
                Result::Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Generic xenstore write error {}", s)
                ))
            }
            _ => { Result::Ok(()) }
        }
    }

    //Listing 8.8 in The Definitive Guide to the Xen Hypervisor
    pub unsafe fn read(&mut self, key: &str) -> io::Result<Option<String>> {
        use core::fmt::Write;
        let result = try!(self.send(xsd_sockmsg_type::Read, &[key]));

        match result {
            (xsd_sockmsg_type::Error, ref s) if s == "ENOENT\0" => { Result::Ok(None) }
            (xsd_sockmsg_type::Error, s) => {
                Result::Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Generic xenstore read error {}", s)
                ))
            }
            (_, s) => { Result::Ok(Some(s)) }
        }
    }

    pub unsafe fn get_permissions(&mut self, key: &str) -> io::Result<String> {
        use core::fmt::Write;
        let result = try!(self.send(xsd_sockmsg_type::GetPerms, &[key]));

        match result {
            (xsd_sockmsg_type::Error, s) => {
                Result::Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Generic xenstore read error {}", s)
                ))
            }
            (_, s) => { Result::Ok(s) }
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
    type_: xsd_sockmsg_type,
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
