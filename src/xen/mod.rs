#[macro_use]
mod arch;

pub mod emergency_console;
mod hypercalls;
pub mod event_channels;
mod grant_tables;
pub mod start_info;
pub mod console_io;
pub mod xenstore;


fn shutdown(reason: hypercalls::sched_op::shutdown::Reason) -> ! {
    use self::hypercalls::Command;
    use self::hypercalls::sched_op::SubCommand;
    use self::hypercalls::sched_op::shutdown::Args;
    hypercall!(
        isize,
        Command::sched_op,
        SubCommand::shutdown, 
        &Args {
            reason: reason
        } as *const Args
    );
    loop {}
}

#[no_mangle]
pub extern fn poweroff() -> ! {
    shutdown(hypercalls::sched_op::shutdown::Reason::poweroff);
}

pub fn crash() -> ! {
    shutdown(hypercalls::sched_op::shutdown::Reason::crash);
}
