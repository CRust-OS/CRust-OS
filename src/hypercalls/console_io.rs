use hypercalls::Hypercall;

#[allow(non_camel_case_types)]
enum Command {
    write = 0,
    read  = 1
}

pub unsafe fn write(s : &[u8]) {
    hypercall!(i64, Hypercall::console_io, Command::write, s.len(), s.as_ptr());
}
