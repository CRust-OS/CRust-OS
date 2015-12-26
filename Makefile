PROFILE ?= DEBUG
TARGET_TRIPLE ?= x86_64-unknown-none-gnu

# Source folders
SRC = src
BUILD = build

# Generated folders
OBJ = obj
BIN = bin
LIB = lib
TMP = tmp
DEPS = deps
DIRTY += $(OBJ) $(BIN) $(LIB) $(TMP) $(DEPS)

# Commands
RM = rm -rf
MKDIR = @mkdir -p
WGET = wget --no-verbose
ECHO = @echo

.SECONDARY:

.PHONY: all
all: $(BIN)/crust.gz

# Modules (include is order-sensitive)
include $(BUILD)/Utils.mk
include $(BUILD)/Rust.mk
include $(BUILD)/Assembler.mk
include $(BUILD)/Linking.mk
include $(BUILD)/libcore.mk

.PHONY: clean
clean:
	$(RM) $(DIRTY)
