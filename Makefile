#!/bin/bash

SHELL = /bin/bash

PROFILE ?= debug
TARGET_TRIPLE ?= x86_64-unknown-xen

# Macros, constants, etc.
include build/Utils.mk

# All targets treated as secondary (implicit rules can be chained arbitrarily)
.SECONDARY:

.PHONY: all
all: $(TARGET)/crust

.PHONY: clean

# Modules (include is order-sensitive)
include $(BUILD)/malloc.mk
include $(BUILD)/Boot.mk
include $(BUILD)/Runtime.mk
include $(BUILD)/Rust.mk
include $(BUILD)/Debug.mk
