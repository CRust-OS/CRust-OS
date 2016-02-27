#[derive(Debug)]
#[repr(usize)]
#[allow(non_camel_case_types)]
enum Command {
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

pub mod console_io {
    use xen::ffi::hypercalls::Command;
    
    #[derive(Debug)]
    #[repr(usize)]
    #[allow(non_camel_case_types)]
    pub enum SubCommand {
        write = 0,
        read  = 1
    }
    
    pub fn write(buf: &[u8]) {
        hypercall!(i64, Command::console_io, SubCommand::write, buf.len(), buf.as_ptr());
    }
}

//pub mod physdev_op_compat;

pub mod grant_table_op {
    use xen::ffi::*;
    
    #[repr(usize)]
    #[allow(non_camel_case_types)]
    enum SubCommand {
        map_grant_ref       = 0,
        unmap_grant_ref     = 1,
        setup_table         = 2,
        dump_table          = 3,
        tranfer             = 4,
        copy                = 5,
        query_size          = 6,
        unmap_and_replace   = 7,
        set_version         = 8,
        get_status_frames   = 9,
        get_version         = 10,
        swap_grant_ref      = 11,
        cache_flush         = 12
    }

    //pub mod map_grant_ref;
    //pub mod unmap_grant ref;
    
    #[derive(Debug)]
    #[repr(C)]
    struct SetupTableArgs {
        dom             : DomID,
        nr_frames       : u32,
        /// Output
        status          : i16,
        /// Output
        frame_list      : XenGuestHandle<PageFrameNumber>
    }
    
    /*
    pub unsafe fn arch_init_gnttab(nr_grant_frames : u32) {
        // TODO: FIX
        let frames = [0u64; 16];
        let mut args = SetupTableArgs {
            dom: DomID::SELF,
            nr_frames: nr_grant_frames,
            status: 0,
            frame_list: XenGuestHandle(PageFrameNumber(&frames[0] as *)) // OK because we know we have > 0 elements
        };
        let _result = hypercall!(
            i64,
            Command::grant_table_op,
            SubCommand::setup_table,
            &mut args as *mut SetupTableArgs,
            16,             // number of frames
            1u32            // number of arguments: 1
        );

        //map_frames(frames) // TODO maybe - 
    }
    */
    
    //pub mod dump_table;
    //pub mod transfer;
    //pub mod copy;
    //pub mod query_size;
    //pub mod unmap_and_replace;
    //pub mod set_version;
    //pub mod get_status_frames;
    //pub mod get_version;
    //pub mod swap_grant_ref;
    //pub mod cache_flush;
}

//pub mod vm_assist;
//pub mod update_va_mapping_otherdomain;
//pub mod iret;
//pub mod vcpu_op;
//pub mod set_segment_base;
//pub mod mmuext_op;
//pub mod xsm_op;
//pub mod nmi_op;

pub mod sched_op {
    use xen::ffi::hypercalls::Command;
    
    #[derive(Debug)]
    #[repr(usize)]
    #[allow(non_camel_case_types)]
    enum SubCommand {
        yield_          = 0,
        block           = 1,
        shutdown        = 2,
        poll            = 3,
        remote_shutdown = 4,
        shutdown_code   = 5,
        watchdog        = 6
    }

    //pub mod yield_;
    //pub mod block;
    
    #[derive(Debug)]
    #[repr(usize)]
    #[allow(non_camel_case_types)]
    pub enum ShutdownReason {
        poweroff    = 0,
        reboot      = 1,
        suspend     = 2,
        crash       = 3,
        watchdog    = 4
    }

    #[repr(C)]
    #[derive(Debug)]
    struct ShutdownArgs {
        reason: ShutdownReason
    }
    
    pub fn shutdown(reason: ShutdownReason) -> ! {
        hypercall!(
            isize,
            Command::sched_op,
            SubCommand::shutdown, 
            &ShutdownArgs {
                reason: reason
            } as *const ShutdownArgs
        );
        loop {}
    }
    
    //pub mod poll;
    //pub mod remote_shutdown;
    //pub mod shutdown_code;
    //pub mod watchdog;
}

//pub mod callback_op;
//pub mod xenoprof_op;

pub mod event_channel_op {
    use xen::ffi::hypercalls::Command;
    use xen::ffi::{DomID, EventChannel, Port, Vcpu};
    
    #[derive(Debug)]
    #[repr(usize)]
    #[allow(non_camel_case_types)]
    enum SubCommand {
        bind_interdomain    = 0,
        bind_virq           = 1,
        bind_pirq           = 2,
        close               = 3,
        send                = 4,
        status              = 5,
        alloc_unbound       = 6,
        bind_ipi            = 7,
        bind_vcpu           = 8,
        unmask              = 9,
        reset               = 10,
        init_control        = 11,
        expand_array        = 12,
        set_priority        = 13
    }

    #[derive(Debug)]
    #[repr(C)]
    struct BindInterdomainArgs {
        remote_dom: DomID,
        remote_port: Port,
        /// Output
        local_port: Port
    }
    
    #[derive(Debug)]
    #[repr(C)]
    struct BindVirqArgs {
        virq: Virq,
        cpu: Vcpu,
        /// Output
        port: Port
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum Virq {
        Timer       = 0,
        Debug       = 1,
        Console     = 2,
        DomExc      = 3,
        Tbuf        = 4,
        Debugger    = 6,
        Xenoprof    = 7,
        ConRing     = 8,
        PcpuState   = 9,
        MemEvent    = 10,
        XcReserved  = 11,
        Enomem      = 12,
        Xenpmu      = 13,
        Arch0       = 16,
        Arch1       = 17,
        Arch2       = 18,
        Arch3       = 19,
        Arch4       = 20,
        Arch5       = 21,
        Arch6       = 22,
        Arch7       = 23
    }
    
    //pub mod bind_pirq;
    
    pub fn close (p: Port) {
        unsafe {
            let mut args = CloseArgs { port: p };
            let _result = hypercall!(
                i64,
                Command::event_channel_op,
                SubCommand::close,
                &mut args as *mut CloseArgs
            );
        }
    }
    
    #[derive(Debug)]
    #[repr(C)]
    struct CloseArgs {
         port: Port
    }
    
    pub fn send(e: &EventChannel) {
        let EventChannel(port) = *e;
        let mut args = SendArgs { port : port };
        let _result = hypercall!(
            i64,
            Command::event_channel_op,
            SubCommand::send,
            &mut args as *mut _
        );
    }
    
    #[derive(Debug)]
    #[repr(C)]
    struct SendArgs {
        port: Port
    }
    
    //pub mod status;
    
    #[derive(Debug)]
    #[repr(C)]
    struct AllocUnboundArgs {
        dom: DomID,
        remote_dom: DomID,
        /// Output
        port: Port
    }
    
    //pub mod bind_ipi;
    //pub mod bind_vcpu;
    //pub mod unmask;
    //pub mod reset;
    //pub mod init_control;
    //pub mod expand_array;
    //pub mod set_priority;
}

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
