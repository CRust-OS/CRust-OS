use ::xen::hypercalls;
use ::xen::hypercalls::console_io;

pub fn print(s : &[u8]) {
    hypercall!(i64, hypercalls::Command::console_io, console_io::SubCommand::write, s.len(), s.as_ptr());
}