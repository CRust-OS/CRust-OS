# Compilers
CC = gcc
CPP = cpp
AS = as
AR = ar
LD = ld
TARGET_FILE = $(TARGET_TRIPLE).json

# Commands
RM = rm -rf
MKDIR = @mkdir -p
WGET = wget --no-verbose
ECHO = @echo -e

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

# Macros
lowercase = $(shell echo ${$1,,})

# When writing a target declaration with wildcard dependencies, caching the dependencies in a .d file means that deleting a dependency will result in rebuilding. (Wildcard dependencies usually do not pick up on deletions)
$(DEPS)/%.d:
	$(MKDIR) $(@D)
	$(ECHO) "$*: $^" > $@

# Expect variable to be defined
.PHONY: var_%
var_%:
	$(if $($*),,$(error Usage: make $*=... [OPTIONS] [TARGET]))
