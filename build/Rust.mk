#CARGO_ARGS += --release

CARGO = cargo
CARGO_ARGS += --target $(TARGET_TRIPLE)
CARGO_DEPS += $(TARGET_TRIPLE).json

# Caching libcrust.a dependencies; see build/Utils.mk
RUST_FILES = $(shell find $(SRC) -name "*.rs")
$(DEPS)/$(TARGET)/crust.d: $(RUST_FILES)
-include $(DEPS)/$(TARGET)/crust.d

$(TARGET)/crust: $(CARGO_DEPS) $(BIN)/boot.o
	$(MKDIR) $(@D)
	$(warning if the following fails with "error: can't find crate for `core`" or "the crate `core` has been compiled with ...", you need to `make lib/libcore.rlib` and put it in your rust toolchain directory under lib/rustlib/$(TARGET_TRIPLE)/lib.)
	$(CARGO) build $(CARGO_ARGS)
	# Cargo doesn't always update timestamp
	[ -e $@ ] && touch $@ 
