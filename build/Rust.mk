ifeq ($(PROFILE), DEBUG)
	TARGET = target/$(TARGET_TRIPLE)/debug
else ifeq ($(PROFILE), RELEASE)
	TARGET = target/$(TARGET_TRIPLE)/release
	CARGO_ARGS += --release
else
	$(error unrecognized PROFILE value $(PROFILE))
endif

DIRTY += $(TARGET)

CARGO = cargo
CARGO_ARGS += --target $(TARGET_TRIPLE)
CARGO_DEPS += $(TARGET_TRIPLE).json

# Caching libcrust.a dependencies; see build/Utils.mk
RUST_FILES = $(shell find $(SRC) -name "*.rs")
$(DEPS)/$(TARGET)/libcrust.a.d: $(RUST_FILES)
-include $(DEPS)/$(TARGET)/libcrust.a.d

$(TARGET)/libcrust.a: $(CARGO_DEPS)
	$(MKDIR) $(@D)
	$(warning if the following fails with "error: can't find crate for `core`", you need to `make lib/libcore.rlib` and put it in your rust toolchain directory under lib/rustlib/$(TARGET_TRIPLE)/lib.)
	$(CARGO) build $(CARGO_ARGS)

$(OBJ)/libcrust.a: $(TARGET)/libcrust.a
	$(MKDIR) $(@D)
	cp $< $@
