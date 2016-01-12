use hypercalls::Command;

#[allow(non_camel_case_types)]
enum SubCommand {
    write = 0,
    read  = 1
}

pub unsafe fn write(s : &[u8]) {
    hypercall!(i64, Command::console_io, SubCommand::write, s.len(), s.as_ptr());
}
