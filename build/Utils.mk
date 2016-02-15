# Compilers
$(CC) = gcc

# Source folders
SRC = src
BUILD = build
BOOT = boot

# Generated folders
OBJ = obj
DIRTY += $(OBJ)
BIN = bin
DIRTY += $(BIN)
TMP = tmp
DIRTY += $(TMP)
DEPS = deps
DIRTY += $(DEPS)
# Cleaned by Cargo
TARGET = target/$(TARGET_TRIPLE)/$(PROFILE)
TARGET_FILE = $(TARGET_TRIPLE).json

# Commands
RM = rm -rf
MKDIR = @mkdir -p
WGET = wget --no-verbose
ECHO = @echo

# Macros
lowercase = $(shell echo ${$1,,})

# When writing a target declaration with wildcard dependencies, caching the dependencies in a .d file means that deleting a dependency will result in rebuilding. (Wildcard dependencies usually do not pick up on deletions)
$(DEPS)/%.d:
	$(MKDIR) $(@D)
	@echo "$*: $^" > $@

.PHONY: var_%
var_%:
	$(if $($*),,$(error Usage: make $*=... [OPTIONS] [TARGET]))
