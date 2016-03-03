#[macro_use]
mod arch;

pub mod emergency_console;
mod ffi;
pub mod event_channels;
pub mod console;
pub mod xenstore;
pub mod mem;

use core::fmt;
use core::fmt::Write;
use core::str;
pub use xen::ffi::start_info::*;

use xen::ffi::hypercalls::sched_op;

pub use xen::emergency_console::DEBUG;

pub struct STDOUT;

impl fmt::Write for STDOUT {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        use std::io::Write;
        let result =
            console::CONSOLE
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

#[no_mangle]
pub extern fn poweroff() -> ! {
    sched_op::shutdown(sched_op::ShutdownReason::poweroff);
}

pub fn crash() -> ! {
    sched_op::shutdown(sched_op::ShutdownReason::crash);
}

#[allow(unused_variables, non_shorthand_field_patterns)]
pub unsafe fn initialize(info: StartInfoPage) {
    let StartInfoPage {
        magic:              magic,
        nr_pages:           nr_pages,
        shared_info:        shared_info,
        flags:              flags,
        store_mfn:          store_mfn,
        store_evtchn:       store_evtchn,
        console:            Console {
            DomU: DomU {
                mfn:        console_mfn,
                evtchn:     console_evtchn
            }
        },
        pt_base:            pt_base,
        nr_pt_frames:       nr_pt_frames,
        mfn_list:           mfn_list,
        mod_start:          mod_start,
        mod_len:            mod_len,
        cmd_line:           cmd_line,
        fist_p2m_pfn:       fist_p2m_pfn,
        nr_p2r_frames:      nr_p2r_frames
    } = info;
    
    writeln!(DEBUG, "prologue!").unwrap();
    writeln!(DEBUG, "Magic: {}", str::from_utf8(&magic).unwrap_or("ERROR")).unwrap();
    writeln!(DEBUG, "nr_pages: {}", nr_pages).unwrap();
    //writeln!(DEBUG, "shared_info: {}", shared_info).unwrap();
    writeln!(DEBUG, "console::initialize").unwrap();
    console::initialize(console_mfn.deref(), console_evtchn);
    writeln!(DEBUG, "xen::xenstore::initialize").unwrap();
    xenstore::initialize(store_mfn.deref(), store_evtchn);
    writeln!(DEBUG, "end of prologue!").unwrap();
}
