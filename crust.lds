PHDRS {
	elfnote PT_NOTE;
    text PT_LOAD;
}

SECTIONS {
	. = SIZEOF_HEADERS;
	elfnote : { *(elfnote) } :elfnote
    .text : { *(.text) } :text
}
