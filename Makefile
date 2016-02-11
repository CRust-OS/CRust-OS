#!/bin/bash

PROFILE ?= debug
ARCH ?= x86_64
TARGET_TRIPLE ?= x86_64-unknown-xen

# Macros, constants, etc.
include build/Utils.mk

.SECONDARY:

.PHONY: all
all: $(TARGET)/crust

.PHONY: clean
clean:
	$(CARGO) clean
	$(RM) $(DIRTY)

# Modules (include is order-sensitive)
include $(BUILD)/Boot.mk
include $(BUILD)/Runtime.mk
include $(BUILD)/Rust.mk
include $(BUILD)/Debug.mk
