AS_ARGS += -g

$(OBJ)/%.s: $(BOOT)/%.S
	$(MKDIR) $(@D)
	$(CPP) $< $@

$(OBJ)/%.o: $(OBJ)/%.s
	$(MKDIR) $(@D)
	$(AS) $(AS_ARGS) -o $@ $<

ASM_FILES = $(shell find $(BOOT) -name "*.S")
OBJ_FILES = $(patsubst $(BOOT)/%.S,$(OBJ)/%.o,$(ASM_FILES)) $(MALLOC_OUT)

# Generate automatic prerequisites
$(DEPS)/$(BIN)/boot.o.d: $(ASM_FILES) 
	$(MKDIR) $(@D)
	$(ECHO) "$(BIN)/boot.o: $(OBJ_FILES)" > $@
-include $(DEPS)/$(BIN)/boot.o.d

$(BIN)/boot.o: $(OBJ_FILES)
	$(MKDIR) $(@D)
	$(ECHO) Bundling $(OBJ_FILES)...
	$(LD) --relocatable -o $@ $(OBJ_FILES)
