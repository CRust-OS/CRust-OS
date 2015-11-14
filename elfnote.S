// #include <arch/x86/page.h>

#include <xen/elfnote.h>
PAGE_SIZE = 0x1000;
HYPERCALL_PAGE = 0x0;
HYPERCALL_PAGE_OFFSET = HYPERCALL_PAGE * PAGE_SIZE;
HYPERCALL_PAGE_END = HYPERCALL_PAGE_OFFSET + PAGE_SIZE;
STACK_TOP = (HYPERCALL_PAGE + 1) * PAGE_SIZE;
STACK_LENGTH = 0x2;
STACK_BOTTOM = STACK_TOP + (STACK_LENGTH * PAGE_SIZE)

#define XEN_ELFNOTE(type, desc)                     \
    .pushsection .note.Xen                        ; \
    .align 4                                      ; \
    .long 2f - 1f         /* namesz */            ; \
    .long 4f - 3f         /* descsz */            ; \
    .long type            /* type   */            ; \
1:.asciz "Xen"            /* name   */            ; \
2:.align 4                                        ; \
3:desc                    /* desc   */            ; \
4:.align 4                                        ; \
    .popsection

// HYPERCALLS
__HYPERVISOR_console_io=18

// ARGS FOR console_io
/*
 * Commands to HYPERVISOR_console_io().
 */
CONSOLEIO_write=0
CONSOLEIO_read=1

/* Guest name and version */
XEN_ELFNOTE(XEN_ELFNOTE_GUEST_OS, .asciz "CRustOS")
XEN_ELFNOTE(XEN_ELFNOTE_GUEST_VERSION, .asciz "0.1")

/* PV loader */
XEN_ELFNOTE(XEN_ELFNOTE_LOADER, .asciz "generic")
XEN_ELFNOTE(XEN_ELFNOTE_HYPERCALL_PAGE, .quad HYPERCALL_PAGE_OFFSET)

/* Xen ABI information */
XEN_ELFNOTE(XEN_ELFNOTE_XEN_VERSION, .asciz "xen-3.0")
// XEN_ELFNOTE(XEN_ELFNOTE_FEATURES, .asciz "!writable_page_tables|pae_pgdir_above_4gb")
// XEN_ELFNOTE(XEN_ELFNOTE_PAE_MODE, .asciz "yes")

.text
.global _start, HYPERCALL_PAGE, fmod, fmodf, HYPERCALL_PAGE_OFFSET, STACK_TOP, STACK_LENGTH, STACK_BOTTOM, HYPERCALL_PAGE_END

_start:
    mov %esp, STACK_BOTTOM
    mov %ebp, STACK_BOTTOM
    call say_hello_rs
    hlt    
// call loop_rs

fmodf:
    ret

fmod:
    ret

.data
    hello: .asciz "Hello, Xen!\n"
    hello_len=13

/* PV entry point. */
// GLOBAL(_start)

    /* Stash start_info_t pointer from domain build. */
    // mov %rsi, start_info
// 
    // /* Move onto own stack. */
    // mov $boot_stack + PAGE_SIZE, %esp
// 
    // call xtf_main
// 
    // /* panic() if xtf_main manages to return. */
    // lea main_err_msg(%rip), %rdi
    // call panic
// 
// SIZE(_start)
// 
// .section .rodata.str1, "aMS", @progbits, 1
// 
// main_err_msg: .asciz "xtf_main() returned\n"
// SIZE(main_err_msg)
