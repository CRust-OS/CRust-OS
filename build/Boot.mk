AS_ARGS += -g

CPP = cpp
AS = as
AR = ar

$(OBJ)/%.s: $(BOOT)/%.S
	$(MKDIR) $(@D)
	$(CPP) $< $@

$(OBJ)/%.o: $(OBJ)/%.s
	$(MKDIR) $(@D)
	$(AS) $(AS_ARGS) -o $@ $<

ASM_FILES = $(shell find $(BOOT) -name "*.S")
OBJ_FILES = $(patsubst $(BOOT)/%.S,$(OBJ)/%.o,$(ASM_FILES))
$(DEPS)/$(BIN)/boot.a.d: $(ASM_FILES)
	$(MKDIR) $(@D)
	$(ECHO) "$(BIN)/boot.a: $(OBJ_FILES)" > $@
-include $(DEPS)/$(BIN)/boot.a.d

$(BIN)/boot.a:
	$(MKDIR) $(@D)
	$(ECHO) Assembling $(ASM_FILES)...
	$(ECHO) Bundling $(OBJ_FILES)...
	$(AR) rcs $@ $^
