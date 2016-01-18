#CARGO_ARGS += --release

CARGO = cargo
CARGO_ARGS += --target $(TARGET_TRIPLE)
RUSTC_ARGS += -Z no-landing-pads
CARGO_DEPS += $(TARGET_TRIPLE).json

# Caching libcrust.a dependencies; see build/Utils.mk
RUST_FILES = $(shell find $(SRC) -name "*.rs")
$(DEPS)/$(TARGET)/libcrust.a.d: $(RUST_FILES)
-include $(DEPS)/$(TARGET)/libcrust.a.d

$(TARGET)/libcrust.a: $(CARGO_DEPS)
	$(MKDIR) $(@D)
	$(ECHO) Building $^...
	$(warning if the following fails with "error: can't find crate for `core`" or "the crate `core` has been compiled with ...", you need to `make lib/libcore.rlib` and put it in your rust toolchain directory under lib/rustlib/$(TARGET_TRIPLE)/lib.)
	$(CARGO) rustc $(CARGO_ARGS) -- $(RUSTC_ARGS)
	# Cargo doesn't always update timestamp
	[ -e $@ ] && touch $@ 

$(BIN)/libcrust.a: $(TARGET)/libcrust.a
	$(MKDIR) $(@D)
	cp $< $@
