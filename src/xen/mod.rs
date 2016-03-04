pub mod emergency_console;
pub mod ffi;
mod event_channels;
pub mod console;
pub mod xenstore;
pub mod mem;

pub use xen::event_channels::EventChannel;
use core::fmt;
use core::fmt::Write;
use core::str;
use std::ops::DerefMove;
use xen::ffi::start_info::*;
use xen::ffi::hypercalls::*;

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
        first_p2m_pfn:      first_p2m_pfn,
        nr_p2m_frames:      nr_p2m_frames
    } = info;
    
    writeln!(DEBUG, "prologue!").unwrap();
    writeln!(DEBUG, "Magic: {}", str::from_utf8(&magic).unwrap_or("ERROR")).unwrap();
    writeln!(DEBUG, "nr_pages: {}", nr_pages).unwrap();
    mem::first_p2m_pfn = first_p2m_pfn.deref_move();
    mem::nr_p2m_frames = nr_p2m_frames;
    writeln!(DEBUG, "console::initialize").unwrap();
    console::initialize(console_mfn.deref_move().deref_move(), EventChannel::new(console_evtchn));
    writeln!(STDOUT, "Console initialized!").unwrap();
    writeln!(DEBUG, "xen::xenstore::initialize").unwrap();
    xenstore::initialize(store_mfn.deref_move().deref_move(), EventChannel::new(store_evtchn));
    writeln!(DEBUG, "end of prologue!").unwrap();
}
