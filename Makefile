#!/bin/bash

PROFILE ?= DEBUG
TARGET_TRIPLE ?= x86_64-unknown-none-gnu

# Macros, constants, etc.
include build/Utils.mk

.SECONDARY:

.PHONY: all
all: $(BIN)/crust.gz

.PHONY: clean
clean:
	$(RM) $(DIRTY)

# Modules (include is order-sensitive)
include $(BUILD)/Rust.mk
include $(BUILD)/Assembler.mk
include $(BUILD)/Linking.mk
include $(BUILD)/libcore.mk
include $(BUILD)/Debug.mk
