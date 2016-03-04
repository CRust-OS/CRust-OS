pub mod emergency_console;
pub mod ffi;
mod event_channels;
pub mod console;
pub mod xenstore;
pub mod mem;

pub use xen::event_channels::EventChannel;
use core::fmt::Write;
use core::str;
use xen::ffi::start_info::*;
use xen::ffi::hypercalls::*;

pub use xen::console::STDOUT;
pub use xen::emergency_console::DEBUG;

#[no_mangle]
pub extern fn poweroff() -> ! {
    sched_op::shutdown(sched_op::ShutdownReason::poweroff);
}

pub fn crash() -> ! {
    sched_op::shutdown(sched_op::ShutdownReason::crash);
}

#[allow(unused_variables, non_shorthand_field_patterns)]
pub unsafe fn initialize(info: &'static mut StartInfoPage) {
    let StartInfoPage {
        magic:              ref magic,
        nr_pages:           ref nr_pages,
        shared_info:        ref shared_info,
        flags:              ref flags,
        store_mfn:          ref mut store_mfn,
        store_evtchn:       store_evtchn,
        console:            Console {
            DomU: DomU {
                mfn:        ref mut console_mfn,
                evtchn:     console_evtchn
            }
        },
        pt_base:            ref pt_base,
        nr_pt_frames:       ref nr_pt_frames,
        mfn_list:           ref mfn_list,
        mod_start:          ref mod_start,
        mod_len:            ref mod_len,
        cmd_line:           ref cmd_line,
        first_p2m_pfn:      ref first_p2m_pfn,
        nr_p2m_frames:      nr_p2m_frames
    } = *info;
    
    writeln!(DEBUG, "prologue!").unwrap();
    writeln!(DEBUG, "Magic: {}", str::from_utf8(magic).unwrap_or("ERROR")).unwrap();
    writeln!(DEBUG, "nr_pages: {}", nr_pages).unwrap();

    mem::first_p2m_pfn = **first_p2m_pfn;
    mem::nr_p2m_frames = nr_p2m_frames;
    writeln!(DEBUG, "console::initialize").unwrap();
    console::initialize(console_mfn, EventChannel::new(console_evtchn));
    writeln!(DEBUG, "xen::xenstore::initialize").unwrap();
    xenstore::initialize(store_mfn, EventChannel::new(store_evtchn));
    writeln!(DEBUG, "end of prologue!").unwrap();
}
