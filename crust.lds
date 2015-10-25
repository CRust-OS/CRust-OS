PHDRS {
	elfnote PT_NOTE;
}

SECTIONS {
	. = SIZEOF_HEADERS;
	elfnote : { *(elfnote) } :elfnote
}
