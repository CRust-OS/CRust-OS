mod xencons_ring;

pub fn write<T>(s : T) where T : AsRef<str> {
    unsafe {
        xencons_ring::write(s.as_ref().as_bytes());
    }
}

//unsafe fn write_hypercall(s : &[u8]) {
    //hypercall!(i64, Command::console_io, SubCommand::write, s.len(), s.as_ptr());
//}


// TODO
pub fn console_init(){}

