include build/Utils.mk

# I tried using libvirt instead of xl but I'm getting "error: libxenlight state driver is not active"
XL = sudo xl
JQ = jq --raw-output --compact-output

_DOM = $(XL) list -l | $(JQ) 'map(select(.config.c_info.name == "$(DOMAIN_NAME)")) | .[0] // empty'
DOM = $(shell $(_DOM))

_DOM_ID = $(_DOM) | $(JQ) '.domid'
DOM_ID = $(shell $(_DOM_ID))

DOM_RUNNING = $(DOM)

.PHONY: dom_create
dom_create: $(TARGET)/crust crust.cfg var_DOMAIN_NAME
	$(if $(DOM_RUNNING),,$(XL) create -p crust.cfg 'name="$(DOMAIN_NAME)"' 'kernel="$(TARGET)/crust"')

.PHONY: dom_destroy
dom_destroy: var_DOMAIN_NAME
	$(if $(DOM_RUNNING),$(XL) destroy $(DOM_ID))

.PHONY: dom_start
dom_start: dom_create
	$(XL) unpause $(DOM_ID)

.PHONY: dom_console
dom_console:
	@echo Starting console - use C-] to exit
	$(if $(DOM_RUNNING),,$(XL) create -c crust.cfg 'name="$(DOMAIN_NAME)"' 'kernel="$(TARGET)/crust"')

.PHONY: dom_%
dom_%: var_DOMAIN_NAME
	$(XL) % $(DOM_ID)

clean: $(if $(DOMAIN_NAME),dom_destroy)

GDBSX_PROC = $(shell pgrep --list-full 'gdbsx' | grep "gdbsx -a $(DOM_ID)")
GDBSX_PID = $(firstword $(GDBSX_PROC))
GDBSX_PROC_PORT = $(lastword $(GDBSX_PROC))

.PHONY: gdbsx_start
gdbsx_start: dom_create var_DOMAIN_NAME
	$(if $(GDBSX_PROC),,$(if $(GDBSX_PORT),(sudo gdbsx -a $(DOM_ID) 64 $(GDBSX_PORT) &),$(error Usage: make GDBSX_PORT=... [OPTIONS] [TARGET])))

.PHONY: gdbsx_stop
gdbsx_stop:
	$(if $(GDBSX_PROC),kill $(GDBSX_PID))

.PHONY: gdb
gdb: gdbsx_start
	gdb -ex "target remote localhost:$(GDBSX_PROC_PORT)"

clean: $(if $(DOM_ID),gdbsx_stop)
