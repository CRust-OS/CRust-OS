include build/Utils.mk

# I tried using libvirt instead of xl but I'm getting "error: libxenlight state driver is not active"
XL = sudo xl

DOM_ID = $$($(XL) domid $(DOMAIN_NAME) 2> /dev/null)

.PHONY: dom_create
dom_create: $(TARGET)/crust crust.cfg
	$(XL) create -p crust.cfg 'name="$(DOMAIN_NAME)"' 'kernel="$(TARGET)/crust"'

.PHONY: dom_destroy
dom_destroy: var_DOMAIN_NAME
	$(XL) destroy $(DOM_ID)

.PHONY: dom_start
dom_start: dom_create
	$(XL) unpause $(DOM_ID)

.PHONY: dom_console
dom_console: dom_create
	(sleep 0.1; $(XL) unpause $(DOM_ID)) &
	echo -e '\e[32mStarting console - use C-] to exit\e[0m'; $(XL) console $(DOM_ID)

clean: $(if $(DOM_ID),dom_destroy)

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

clean: $(if $(GDBSX_PROC),gdbsx_stop)
