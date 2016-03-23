#!/bin/bash

SHELL = /bin/bash

PROFILE ?= debug
TOOLCHAIN ?= nightly
TARGET_TRIPLE ?= x86_64-unknown-xen
RUSTLIB ?= $(HOME)/.multirust/toolchains/$(TOOLCHAIN)/lib/rustlib/$(TARGET_TRIPLE)/lib
OUT_DIR = target/$(TARGET_TRIPLE)/$(PROFILE)

# Macros, constants, etc.
include build/Utils.mk

# All targets treated as secondary (implicit rules can be chained arbitrarily)
.SECONDARY:

.PHONY: all
all: $(OUT_DIR)/crust

.PHONY: clean
clean:
	$(RM) $(DIRTY)

# Modules (include is order-sensitive)
include $(BUILD)/malloc.mk
include $(BUILD)/Boot.mk
include $(BUILD)/Runtime.mk
include $(BUILD)/Rust.mk
include $(BUILD)/Debug.mk
