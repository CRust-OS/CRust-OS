#[allow(non_camel_case_types)]
#[repr(usize)]
pub enum Command {
    set_trap_table      = 0,
    mmu_update          = 1,
    set_gdt             = 2,
    stack_switch        = 3,
    set_callbacks       = 4,
    fpu_taskswitch      = 5,
    sched_op_compat     = 6,
    platform_op         = 7,
    set_debugreg        = 8,
    get_debugreg        = 9,
    update_descriptor   = 10,
    memory_op           = 12,
    multicall           = 13,
    update_va_mapping   = 14,
    set_timer_op        = 15,
    event_channel_op_compat = 16,
    xen_version         = 17,
    console_io          = 18,
    physdev_op_compat   = 19,
    grant_table_op      = 20,
    vm_assist           = 21,
    update_va_mapping_otherdomain = 22,
    iret                = 23,
    vcpu_op             = 24,
    set_segment_base    = 25,
    mmuext_op           = 26,
    xsm_op              = 27,
    nmi_op              = 28,
    sched_op            = 29,
    callback_op         = 30,
    xenoprof_op         = 31,
    event_channel_op    = 32,
    physdev_op          = 33,
    hvm_op              = 34,
    sysctl              = 35,
    domctl              = 36,
    kexec_op            = 37,
    tmem_op             = 38,
    xc_reserved_op      = 39,
    xen_pmu_op          = 40,
    arch_0              = 48,
    arch_1              = 49,
    arch_2              = 50,
    arch_3              = 51,
    arch_4              = 52,
    arch_5              = 53,
    arch_6              = 54,
    arch_7              = 55
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DomID(u16);

impl DomID {
    pub const FIRST_RESERVED: DomID = DomID(0x7FF0);
    pub const SELF: DomID           = DomID(0x7FF0);
    pub const IO: DomID             = DomID(0x7FF1);
    pub const XEN: DomID            = DomID(0x7FF2);
    pub const COW: DomID            = DomID(0x7FF3);
    pub const INVALID: DomID        = DomID(0x7FF4);
    pub const IDLE: DomID           = DomID(0x7FFF);
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vcpu(u32);

//pub mod set_trap_table;
//pub mod mmu_update;
//pub mod set_gdt;
//pub mod stack_switch;
//pub mod set_callbacks;
//pub mod fpu_taskswitch;
//pub mod sched_op_compat;
//pub mod platform_op;
//pub mod set_debugreg;
//pub mod get_debugreg;
//pub mod update_descriptor;
//pub mod memory_op;
//pub mod multicall;
//pub mod update_va_mapping;
//pub mod set_timer_op;
//pub mod event_channel_op_compat;
//pub mod xen_version;
pub mod console_io;
//pub mod physdev_op_compat;
pub mod grant_table_op;
//pub mod vm_assist;
//pub mod update_va_mapping_otherdomain;
//pub mod iret;
//pub mod vcpu_op;
//pub mod set_segment_base;
//pub mod mmuext_op;
//pub mod xsm_op;
//pub mod nmi_op;
pub mod sched_op;
//pub mod callback_op;
//pub mod xenoprof_op;
pub mod event_channel_op;
//pub mod physdev_op;
//pub mod hvm_op;
//pub mod sysctl;
//pub mod domctl;
//pub mod kexec_op;
//pub mod tmem_op;
//pub mod xc_reserved_op;
//pub mod xen_pmu_op;
//pub mod arch_0;
//pub mod arch_1;
//pub mod arch_2;
//pub mod arch_3;
//pub mod arch_4;
//pub mod arch_5;
//pub mod arch_6;
//pub mod arch_7;
