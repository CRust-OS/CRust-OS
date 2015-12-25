# When writing a target declaration with wildcard dependencies, caching the dependencies in a .d file means that deleting a dependency will result in rebuilding. (Wildcard dependencies usually do not pick up on deletions)
$(DEPS)/%.d:
	$(MKDIR) $(@D)
	@echo "$*: $^" > $@
