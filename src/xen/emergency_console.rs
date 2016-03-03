use std::sync::RwLock;
use ::xen::hypercalls;
use ::xen::hypercalls::console_io;
use core::fmt;

static _EMERGENCY_CONSOLE : RwLock<EmergencyConsoleInterface> = 
    RwLock::new(
        EmergencyConsoleInterface { 
            buffer: [0; LINE_LENGTH + 1]
        }
    );

const LINE_LENGTH : usize = 75;

struct EmergencyConsoleInterface {
    buffer: [u8; LINE_LENGTH + 1]
}

#[allow(non_camel_case_types)]
pub struct EMERGENCY_CONSOLE;

impl EMERGENCY_CONSOLE {
    fn notify(buf: &[u8], i: usize) {
        hypercall!(i64, hypercalls::Command::console_io, console_io::SubCommand::write, i, buf.as_ptr());
    }
}

impl fmt::Write for EMERGENCY_CONSOLE {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buffer = _EMERGENCY_CONSOLE.write().buffer;
        let mut i = 0;
        for c in s.as_bytes() {
            if *c == '\0' as u8 {
                break;
            } else {
                if i < LINE_LENGTH {
                    buffer[i] = *c;
                    i = i + 1;
                } else {
                    EMERGENCY_CONSOLE::notify(&buffer, LINE_LENGTH + 1);
                    i = 0;
                }
            }
        }

        if i != 0 {
            buffer[i] = '\0' as u8;
            EMERGENCY_CONSOLE::notify(&buffer, i);
        }
        Ok(())
    }
}
