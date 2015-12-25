PROFILE ?= DEBUG
TARGET ?= x86_64-unknown-none-gnu

OBJ = obj
BIN = bin
SRC = src
LIB = lib
TMP = tmp
DEPS = deps

RM = rm -rf
MKDIR = @mkdir -p
WGET = wget --no-verbose

.PHONY: all
all: $(BIN)/crust.gz

# Order-sensitive
include build/Utils.mk
include build/Rust.mk
include build/Assembler.mk
include build/Linking.mk
include build/libcore.mk

GENERATED = $(BIN) $(OBJ) $(TMP) $(DEPS) $(LIB)
.PHONY: clean
clean:
	$(RM) $(GENERATED)
