use std::sync::RwLock;
use ::xen::hypercalls;
use ::xen::hypercalls::console_io;
use core::fmt;

pub static EMERGENCY_CONSOLE : EmergencyConsole = 
    EmergencyConsole { 
        interface: RwLock::new(
            EmergencyConsoleInterface { 
                buffer: [0; LINE_LENGTH + 1]
            }
        )
    };

const LINE_LENGTH : usize = 75;

pub struct EmergencyConsole {
    interface: RwLock<EmergencyConsoleInterface>
}

struct EmergencyConsoleInterface {
    buffer: [u8; LINE_LENGTH + 1]
}

impl EmergencyConsole {
    fn notify(buf: &[u8], i: usize) {
        hypercall!(i64, hypercalls::Command::console_io, console_io::SubCommand::write, i, buf.as_ptr());
    }
}

impl<'a> fmt::Write for &'a EmergencyConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let buffer = self.interface.write().buffer;
        let mut i = 0;
        for c in s.as_bytes() {
            if i < LINE_LENGTH {
                buffer[i] = *c;
                i = i + 1;
            } else {
                EmergencyConsole::notify(&buffer, LINE_LENGTH + 1);
                i = 0;
            }
        }

        if i != 0 {
            buffer[i] = '\0' as u8;
            i += 1;
            EmergencyConsole::notify(&buffer, i);
        }
        Ok(())
    }
}
