LD = ld
LD_ARGS += -nostdlib
# LD_ARGS += -L $(OBJ)
LD_ARGS += --gc-sections
LD_ARGS += -T crust.lds
LD_DEPS += crust.lds

$(BIN)/crust: $(LD_DEPS) $(BIN)/boot.o $(BIN)/libcrust.a
	$(ECHO) $(ASM_FILES)
	$(ECHO) $(OBJ_FILES)
	$(MKDIR) $(@D)
	$(LD) $(LD_ARGS) -o $@ $(BIN)/boot.o $(BIN)/libcrust.a

$(BIN)/crust.gz: $(BIN)/crust
	gzip -f -9 -c $< > $@


