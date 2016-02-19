ENTRY(_start)

/* all sections are padded with 0xCC (int 3, meaning breakpoint) in case we accidentally jump into uninitialized memory */

SECTIONS {
    .bss : ALIGN(0x1000) {
        STACK_BOTTOM = .;
        . += 0x50000;
        STACK_TOP = .;
        HYPERCALL_PAGE = .;
        . += 0x1000;
        *(.bss);
        *(.bss.*);
    } = 0xCC 
    .text : {
        *(.text);
        *(.text.*);
    } = 0xCC
    .data : {
        *(.data);
        *(.data.*);
    } = 0xCC
    .note : {
        *(.note.Xen);
    }
    HEAP = .;
    . += 0x0100;
    HEAD_END =.;
}
