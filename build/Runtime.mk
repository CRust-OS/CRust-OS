# Patch and recompile libcore to remove float support

SYSROOT = sysroot
LIBS = libcore liballoc libcollections libstd_unicode
RLIB_OUT_DIR = sysroot/lib/rustlib/$(TARGET)/lib
RLIBS = $(patsubst %, $(RLIB_OUT_DIR)/%.rlib, $(LIBS))

$(RLIBS): $(RLIB_OUT_DIR)/%.rlib:
	$(MKDIR) $(@D)
	rustc -Z no-landing-pads --sysroot $(SYSROOT) --target $(TARGET) -g $(shell rustc --print sysroot)/lib/rustlib/src/rust/src/$*/lib.rs --out-dir $(@D)

test: $(RLIB_OUT_DIR)/libcore.rlib

$(RLIB_OUT_DIR)/liballoc.rlib:         $(RLIB_OUT_DIR)/libcore.rlib
$(RLIB_OUT_DIR)/libstd_unicode.rlib:   $(RLIB_OUT_DIR)/libcore.rlib
$(RLIB_OUT_DIR)/libcollections.rlib:   $(RLIB_OUT_DIR)/liballoc.rlib $(RLIB_OUT_DIR)/libstd_unicode.rlib

DIRTY += $(SYSROOT)
