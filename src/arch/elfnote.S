#include <xen/elfnote.h>

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

/* Guest name and version */
XEN_ELFNOTE(XEN_ELFNOTE_GUEST_OS, .asciz "CRustOS")
XEN_ELFNOTE(XEN_ELFNOTE_GUEST_VERSION, .asciz "0.1")

/* PV loader */
XEN_ELFNOTE(XEN_ELFNOTE_LOADER, .asciz "generic")
XEN_ELFNOTE(XEN_ELFNOTE_HYPERCALL_PAGE, .quad HYPERCALL_PAGE)

/* Xen ABI information */
XEN_ELFNOTE(XEN_ELFNOTE_XEN_VERSION, .asciz "xen-3.0")
