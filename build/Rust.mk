#CARGO_ARGS += --release

CARGO = cargo
CARGO_ARGS += --target $(TARGET)

CARGO_DEPS += $(TARGET).json
CARGO_DEPS += crust.lds
CARGO_DEPS += Cargo.toml
CARGO_DEPS += Cargo.lock

# Caching libcrust.a dependencies; see build/Utils.mk
RUST_FILES = $(shell find $(SRC) -name "*.rs")
$(DEPS)/$(OUT_DIR)/crust.d: $(RUST_FILES)
-include $(DEPS)/$(OUT_DIR)/crust.d

$(OUT_DIR)/crust: $(CARGO_DEPS) $(BIN)/boot.o $(RLIBS)
	$(MKDIR) $(@D)
	RUSTFLAGS="--sysroot $(SYSROOT)" $(CARGO) build --target $(TARGET)
	@[ -e $@ ] && touch $@ # Cargo doesn't always update timestamp

.PHONY: cargo-clean
cargo-clean:
	$(CARGO) clean

clean: cargo-clean
