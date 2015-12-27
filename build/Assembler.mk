ifeq ($(PROFILE), DEBUG)
AS_ARGS += -g
else ifeq ($(PROFILE), RELEASE)
else
$(error unrecognized PROFILE value $(PROFILE))
endif

CPP = cpp
AS = as

ASM_TMP = $(TMP)/asm

$(ASM_TMP)/%.s: $(SRC)/%.S
	$(MKDIR) $(@D)
	$(CPP) $< $@

$(OBJ)/%.o: $(ASM_TMP)/%.s
	$(MKDIR) $(@D)
	$(AS) $(AS_ARGS) -o $@ $<

