.PHONY: clean domain_running xl_create xl_console xl_destroy gdb gdbsx

SHELL=/bin/bash

DOMAIN_NAME=Crust-OS
GDBSX_PORT=9999

bin/crust.gz: bin/crust
	gzip -f -9 -c $^ > $@

bin/crust: crust.lds target/debug/libcrust.a src/arch/*.S
	mkdir -p `dirname $@`
	gcc -nostdlib -o bin/crust -T crust.lds src/arch/*.S target/debug/libcrust.a

target/%/libcrust.a: Cargo.toml Cargo.lock src/*.rs
	cargo rustc -- -Z no-landing-pads

clean:
	-rm -rf target
	-rm -rf bin

# Not an actual goal, just useful as a dependency
DOMAIN_ID=$(shell xl domid $(DOMAIN_NAME) 2> /dev/null)
ifeq ($(DOMAIN_ID),)
domain_running: xl_create
	$(eval DOMAIN_ID=$(shell xl domid $(DOMAIN_NAME) 2> /dev/null))
else
domain_running:
endif

xl_create: bin/crust
	xl create -p crust.cfg 'name="$(DOMAIN_NAME)"'

xl_console: domain_running
	@echo Starting console - use C-] to exit
	xl console $(DOMAIN_ID)
	xl destroy $(DOMAIN_ID)

xl_destroy:
ifdef ($(DOMAIN_ID),)
	xl destroy $(DOMAIN_ID)
else
	$(error $(DOMAIN_NAME) is not running)
endif

gdbsx: domain_running
	gdbsx -a $(DOMAIN_ID) 64 $(GDBSX_PORT) > /dev/null

gdb:
	rust-gdb -ex "target remote localhost:$(GDBSX_PORT)"
