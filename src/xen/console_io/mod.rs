mod xencons_ring;
use core::fmt;
use ::xen::arch::mem::mfn_to_virt;
use ::xen::start_info::start_info_page;
use ::xen::event_channels::EventChannel;

pub unsafe fn initialize() {
    let console = xencons_ring::CONSOLE.write();
    *console = Some (xencons_ring::Console {
        event_channel: EventChannel(start_info_page.console.domU.evtchn),
        interface: &mut *(mfn_to_virt(start_info_page.console.domU.mfn) as *mut _)
    })
}

pub fn write<T>(s : T) where T : AsRef<str> {
        xencons_ring::CONSOLE
            .write()
            .as_mut()
            .unwrap()
            .write(s.as_ref().as_bytes())
}

pub struct STDOUT;

impl fmt::Write for STDOUT {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(s);
        Ok(())
    }
}
