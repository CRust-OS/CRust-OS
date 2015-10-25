#include "xen/elfnote.h"

// Based on the ELF specification, under "Note Section"
#define ELFNOTE_ENTRY(type, value) \
.subsection type; \
.p2align 2; \
.long 2f - 1f; \
.long 4f - 3f; \
.long type; \
1: .asciz "Xen"; \
2: .p2align 2; \
3: value; \
4: .p2align 2; \

.section "elfnote"
	PAGE_SIZE = 0x1000
	//See xen/elfnote.h for more info
	ELFNOTE_ENTRY(XEN_ELFNOTE_ENTRY,          .quad 0)
	ELFNOTE_ENTRY(XEN_ELFNOTE_HYPERCALL_PAGE, .quad 2 * PAGE_SIZE)
	ELFNOTE_ENTRY(XEN_ELFNOTE_VIRT_BASE,      .quad 0x0)
	ELFNOTE_ENTRY(XEN_ELFNOTE_XEN_VERSION,    .asciz "xen-4.9")
	ELFNOTE_ENTRY(XEN_ELFNOTE_GUEST_OS,       .asciz "crust")
	ELFNOTE_ENTRY(XEN_ELFNOTE_GUEST_VERSION,  .asciz "0.1")
	ELFNOTE_ENTRY(XEN_ELFNOTE_LOADER,         .asciz "generic")
	ELFNOTE_ENTRY(XEN_ELFNOTE_FEATURES,
	        .asciz "") //look at xen/features.h for info
	
