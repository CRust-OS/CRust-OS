# Source folders
SRC = src
BUILD = build

# Generated folders
OBJ = obj
BIN = bin
LIB = lib
TMP = tmp
DEPS = deps
DIRTY += $(OBJ) $(BIN) $(LIB) $(TMP) $(DEPS)

# Commands
RM = rm -rf
MKDIR = @mkdir -p
WGET = wget --no-verbose
ECHO = @echo

# When writing a target declaration with wildcard dependencies, caching the dependencies in a .d file means that deleting a dependency will result in rebuilding. (Wildcard dependencies usually do not pick up on deletions)
$(DEPS)/%.d:
	$(MKDIR) $(@D)
	@echo "$*: $^" > $@

.PHONY: var_%
var_%:
	$(if $($*),,$(error Usage: make $*=... [OPTIONS] [TARGET]))
