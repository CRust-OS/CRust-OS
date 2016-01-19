use hypercalls::Command;

pub mod xencons_ring;

#[allow(non_camel_case_types)]
enum SubCommand {
    write = 0,
    read  = 1
}

pub unsafe fn write(s : &[u8]) {
    //write_hypercall(s)
    xencons_ring::write(s);
}

unsafe fn write_hypercall(s : &[u8]) {
    hypercall!(i64, Command::console_io, SubCommand::write, s.len(), s.as_ptr());
}


pub fn console_init(){}

