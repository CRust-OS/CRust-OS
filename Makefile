#!/bin/bash

SHELL = /bin/bash

PROFILE ?= debug
TOOLCHAIN ?= nightly
TARGET ?= x86_64-unknown-xen
OUT_DIR = target/$(TARGET)/$(PROFILE)

# Macros, constants, etc.
include build/Utils.mk

# All targets treated as secondary (implicit rules can be chained arbitrarily)
.SECONDARY:

# Turn off implicit rules
.SUFFIXES:

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
