SECTIONS {
    . = 0x0;
    .hypercall_page = 0x0;
    . = HYPERCALL_PAGE_END;
    . = STACK_TOP;
    .stack = 0x0;
    . = STACK_BOTTOM;
    .text : {
        *(.text)
    } /* = 0x9090 */
    .note : {
        *(.note)
        *(.note.*)
    }
    .data : {
        *(.data)
        *(.data.*)
    }
}
