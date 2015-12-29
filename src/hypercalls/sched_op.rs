use hypercalls::Hypercall;

#[allow(non_camel_case_types)]
enum Command {
    _yield          = 0,
    block           = 1,
    shutdown        = 2,
    poll            = 3,
    remote_shutdown = 4,
    shutdown_code   = 5,
    watchdog        = 6
}

pub unsafe fn _yield() {
    hypercall!(i64, Hypercall::sched_op, Command::_yield);
}

pub unsafe fn block() {
    hypercall!(i64, Hypercall::sched_op, Command::block);
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
    hypercall!(i64, Hypercall::sched_op, Command::shutdown, args);
    loop {} // unreachable
}
