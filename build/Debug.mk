# I tried using libvirt instead of xl but I'm getting "error: libxenlight state driver is not active"
XL = sudo xl
GDB = rust-gdb

DOM_ID = $(shell echo $$($(XL) domid $(DOMAIN_NAME) 2> /dev/null))

.PHONY: dom_create
dom_create: $(TARGET)/crust crust.cfg var_DOMAIN_NAME
	$(if $(DOM_ID),,$(XL) create -p crust.cfg 'name="$(DOMAIN_NAME)"' 'kernel="$(TARGET)/crust"')

.PHONY: dom_destroy
dom_destroy:
	$(if $(DOMAIN_NAME), $(if $(DOM_ID),$(XL) destroy $(DOM_ID)))

clean: dom_destroy

.PHONY: dom_start
dom_start: dom_create
	$(XL) unpause $(DOM_ID)

.PHONY: dom_console
dom_console: dom_create
	(sleep 0.1; $(XL) unpause $(DOM_ID)) &
	$(ECHO) -e '\e[32mStarting console - use C-] to exit\e[0m'; $(XL) console $(DOM_ID)


GDBSX_PROC = $(shell pgrep --list-full 'gdbsx' | grep '$(GDBSX_PORT)$$')
GDBSX_PID = $(firstword $(GDBSX_PROC))
GDBSX_PROC_PORT = $(lastword $(GDBSX_PROC))

.PHONY: gdbsx_start
gdbsx_start: dom_create var_DOMAIN_NAME var_GDBSX_PORT
	$(if $(GDBSX_PROC),,(sudo gdbsx -a $(DOM_ID) 64 $(GDBSX_PORT) > /dev/null &))

.PHONY: gdbsx_stop
gdbsx_stop:
	$(if $(GDBSX_PROC), sudo kill $(GDBSX_PID))

clean: gdbsx_stop

.PHONY: gdb
gdb: gdbsx_start
	$(GDB) -ex "target remote localhost:$(GDBSX_PROC_PORT)"
