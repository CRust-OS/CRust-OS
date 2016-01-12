use hypercalls::Command;

#[allow(non_camel_case_types)]
enum SubCommand {
    _yield          = 0,
    block           = 1,
    shutdown        = 2,
    poll            = 3,
    remote_shutdown = 4,
    shutdown_code   = 5,
    watchdog        = 6
}

pub unsafe fn _yield() {
    hypercall!(i64, Command::sched_op, SubCommand::_yield);
}

pub unsafe fn block() {
    hypercall!(i64, Command::sched_op, SubCommand::block);
}

#[allow(non_camel_case_types)]
pub enum ShutdownReason {
    poweroff    = 0,
    reboot      = 1,
    suspend     = 2,
    crash       = 3,
    watchdog    = 4
}

pub struct Shutdown {
    pub reason: ShutdownReason
}

pub unsafe fn shutdown(args: *const Shutdown) -> ! {
    hypercall!(i64, Command::sched_op, SubCommand::shutdown, args);
    loop {} // unreachable
}
