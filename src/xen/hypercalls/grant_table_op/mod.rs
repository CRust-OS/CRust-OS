use core::fmt;
use super::DomID;
use core::convert::{From, Into};
use collections::String;

#[repr(usize)]
#[allow(non_camel_case_types)]
pub enum SubCommand {
    map_grant_ref = 0,
    unmap_grant_ref = 1,
    setup_table = 2,
    dump_table = 3,
    tranfer = 4,
    copy = 5,
    query_size = 6,
    unmap_and_replace = 7,
    set_version = 8,
    get_status_frames = 9,
    get_version = 10,
    swap_grant_ref = 11,
    cache_flush = 12
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PFN(pub u64);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GrantHandle(u32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GrantTableRef(u32);

#[repr(C)]
/// This is really just the full_page variant of the 'grant_entry_v2' union 
pub struct GrantEntry {
    flags: u16,
    domid: DomID,
    __pad: u32,
    frame: u64
}

#[repr(i16)]
#[derive(Copy, Clone)]
pub enum Status {
    OK = 0,
    GeneralError = -1,
    BadDomain = -2,
    BadGntRef = -3,
    BadHandle = -4,
    BadVirtAddr = -5,
    BadDevAddr = -6,
    NoDeviceSpace = -7,
    PermissionDenied = -8,
    BadPage = -9,
    BadCopyArg = -10,
    AddressTooBig = -11,
    EAgain = -12
}

impl From<i16> for Status {
    fn from(x : i16) -> Status {
        match x {
            0   => Status::OK,
            -1  => Status::GeneralError,
            -2  => Status::BadDomain,
            -3  => Status::BadGntRef,
            -4  => Status::BadHandle,
            -5  => Status::BadVirtAddr,
            -6  => Status::BadDevAddr,
            -7  => Status::NoDeviceSpace,
            -8  => Status::PermissionDenied,
            -9  => Status::BadPage,
            -10 => Status::BadCopyArg,
            -11 => Status::AddressTooBig,
            -12 => Status::EAgain,
            _ => Status::GeneralError
        }
    }
}

impl Into<Result<(), String>> for Status {
    fn into(self) -> Result<(), String> {
        match self {
            Status::OK  => Ok(()),
            s           => Err(format!("{}", s))
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let status_str = match self {
            &Status::OK => "Okay",
            &Status::GeneralError => "Undefined Error",
            &Status::BadDomain => "Unrecognized domain id",
            &Status::BadGntRef => "Invalid grant reference",
            &Status::BadHandle => "Invalid mapping handle",
            &Status::BadVirtAddr => "Invalid virtual address",
            &Status::BadDevAddr => "Invalid device address",
            &Status::NoDeviceSpace => "No spare translation slot in the I/O MMU",
            &Status::PermissionDenied => "Permission Denied",
            &Status::BadPage => "Bad Page",
            &Status::BadCopyArg => "Copy arguments cross page boundary",
            &Status::AddressTooBig => "Page address size too large",
            &Status::EAgain => "Operation not done; try again"
        };
        write!(f, "{}", status_str)
    }
}

pub mod map_grant_ref;
//pub mod unmap_grant_ref;
pub mod setup_table;
//pub mod dump_table;
//pub mod transfer;
//pub mod copy;
//pub mod query_size;
//pub mod unmap_and_replace;
//pub mod set_version;
//pub mod get_status_frames;
//pub mod get_version;
//pub mod swap_grant_ref;
//pub mod cache_flush;
