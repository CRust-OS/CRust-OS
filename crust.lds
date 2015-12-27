ENTRY(_start)

/* all sections are padded with 0xCC (int 3, meaning breakpoint) in case we accidentally jump into uninitialized memory */

SECTIONS {
    .bss : ALIGN(0x1000) {
        HYPERCALL_PAGE = .;
        . += 0x1000;
        STACK_TOP = .;
        . += 0x2000;
        STACK_BOTTOM = .;
        *(.bss);
        *(.bss.*);
    } = 0xCC 
    .text : ALIGN(0x1000) {
        *(.text);
        *(.text.*);
    } = 0xCC
    .data : ALIGN(0x1000) {
        *(.data);
        *(.data.*);
    } = 0xCC
    .note : {
        *(.note);
        *(.note.*);
    }
}
