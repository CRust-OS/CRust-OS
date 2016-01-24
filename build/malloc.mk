MALLOC_TMP = $(TMP)/malloc

MALLOC_SRC = mm/src/malloc.c
MALLOC_OUT = $(MALLOC_TMP)/malloc.o

CFLAGS := -nodefaultlibs -c -fno-stack-protector
DEFINE := -DHAVE_MMAP=0 -DHAVE_MREMAP=

$(MALLOC_TMP)/malloc: $(MEMORY_DIR)
	$(MKDIR) $(@D)
	$(cc) $(CFLAGS) $(MALLOC_SRC) -o $(MALLOC_OUT)

