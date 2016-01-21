#[allow(non_camel_case_types)]
pub enum SubCommand {
    _yield          = 0,
    block           = 1,
    shutdown        = 2,
    poll            = 3,
    remote_shutdown = 4,
    shutdown_code   = 5,
    watchdog        = 6
}

pub mod shutdown;