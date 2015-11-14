SECTIONS {
    .bss : ALIGN(0x1000) {
        HYPERCALL_PAGE = .;
        . += 0x1000;
        STACK_TOP = .;
        . += 0x2000;
        STACK_BOTTOM = .;
        *(.bss);
	*(.bss.*);
    }

    .data : {
        *(.data);
        *(.data.*);
    }
    .note : {
        *(.note);
        *(.note.*);
    }
}
