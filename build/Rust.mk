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

$(OBJ)/libcrust.a: $(CARGO_DEPS) $(LIB)/libcore.rlib
	$(MKDIR) $(@D)
	$(CARGO) build $(CARGO_ARGS)

