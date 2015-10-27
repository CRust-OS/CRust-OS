SECTIONS {
    . = 0x10000;
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
