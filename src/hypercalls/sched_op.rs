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

pub fn block() {
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

struct Shutdown {
    reason: ShutdownReason
}

pub fn shutdown(reason : ShutdownReason) -> ! {
    hypercall!(i64, Hypercall::sched_op, Command::shutdown, &(Shutdown { reason: reason }) as *const Shutdown);
    loop {} // unreachable
}
