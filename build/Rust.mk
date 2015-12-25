ifeq ($(PROFILE), DEBUG)
else ifeq ($(PROFILE), RELEASE)
	CARGO_ARGS += --release
else
	$(error unrecognized PROFILE value $(PROFILE))
endif

CARGO = cargo
CARGO_ARGS += --target $(TARGET)
CARGO_DEPS += $(TARGET).json

# Caching libcrust.a dependencies; see build/Utils.mk
RUST_FILES = $(shell find $(SRC) -name "*.rs")
$(DEPS)/$(OBJ)/libcrust.a.d: $(RUST_FILES)
-include $(DEPS)/$(OBJ)/libcrust.a.d

$(OBJ)/libcrust.a: $(CARGO_DEPS)
	$(MKDIR) $(@D)
	$(warning if the following fails with "error: can't find crate for `core`", you need to `make lib/libcore.rlib` and put it in your rust toolchain directory under lib/rustlib/$(TARGET)/lib.)
	$(CARGO) build $(CARGO_ARGS)

