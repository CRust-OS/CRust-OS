#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Reason {
    poweroff    = 0,
    reboot      = 1,
    suspend     = 2,
    crash       = 3,
    watchdog    = 4
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Args {
    pub reason: Reason
}
