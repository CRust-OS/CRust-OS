use std::sync::RwLock;
use xen::ffi::hypercalls::console_io::write;
use core::fmt;

static EMERGENCY_CONSOLE : RwLock<EmergencyConsoleInterface> = 
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
pub struct DEBUG;

impl fmt::Write for DEBUG {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buffer = EMERGENCY_CONSOLE.write().buffer;
        let mut i = 0;
        for c in s.as_bytes() {
            if *c == '\0' as u8 {
                break;
            } else {
                if i == LINE_LENGTH {
                    buffer[i] = '\0' as u8;
                    write(&buffer);
                    i = 0;
                }
                buffer[i] = *c;
                i = i + 1;
            }
        }

        buffer[i] = '\0' as u8;
        let to_write = &buffer[..i];
        write(to_write);
        Ok(())
    }
}
