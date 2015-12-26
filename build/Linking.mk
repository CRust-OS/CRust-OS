LD = ld
LD_ARGS += -nostdlib
LD_ARGS += -L $(OBJ)
LD_ARGS += --gc-sections
LD_ARGS += -T crust.lds
LD_DEPS += crust.lds

# Caching crust dependencies; see build/Utils.mk
ASM_FILES = $(shell find $(SRC) -name "*.S")
OBJ_FILES = $(patsubst $(SRC)/%.S,$(OBJ)/%.o,$(ASM_FILES))
$(DEPS)/$(BIN)/crust.d: $(ASM_FILES)
	$(MKDIR) $(@D)
	@echo "$(BIN)/crust: $(OBJ_FILES)" > $@
-include $(DEPS)/$(BIN)/crust.d

$(BIN)/crust: $(LD_DEPS) $(OBJ)/libcrust.a
	$(MKDIR) $(@D)
	$(LD) $(LD_ARGS) -o $@ $(OBJ_FILES) $(OBJ)/libcrust.a

$(BIN)/crust.gz: $(BIN)/crust
	gzip -f -9 -c $< > $@


