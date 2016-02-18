#CARGO_ARGS += --release

CARGO = cargo
CARGO_ARGS += --target $(TARGET_TRIPLE)
RUSTC_ARGS += -Z no-landing-pads
CARGO_DEPS += $(TARGET_FILE) crust.lds

# Caching libcrust.a dependencies; see build/Utils.mk
RUST_FILES = $(shell find $(SRC) -name "*.rs")
$(DEPS)/$(TARGET)/crust.d: $(RUST_FILES)
-include $(DEPS)/$(TARGET)/crust.d

$(TARGET)/crust: $(CARGO_DEPS) $(BIN)/boot.o runtime
	$(MKDIR) $(@D)
	$(warning if the following fails with "error: can't find crate for `core`" or "the crate `core` has been compiled with ...", you need to `multirust update` and `make clean-runtime`.)
	$(CARGO) rustc $(CARGO_ARGS) -- $(RUSTC_ARGS)
	@[ -e $@ ] && touch $@ # Cargo doesn't always update timestamp
