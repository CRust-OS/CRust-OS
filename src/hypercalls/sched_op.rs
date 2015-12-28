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
    hypercall!(i64, Command::block);
}
