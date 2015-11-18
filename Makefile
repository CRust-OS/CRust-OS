.PHONY: clean create start debug

DOMAIN_NAME=Crust-OS
PORT=9999

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

#Conceptually a part of the "start" goal, but we need to create the domain before the $(eval DOMAIN_ID=...) gets expanded.
create_domain: bin/crust
	xl create -p crust.cfg 'name="$(DOMAIN_NAME)"'

start: bin/crust create_domain
	$(eval DOMAIN_ID=$(shell xl domid $(DOMAIN_NAME)))
	gdbsx -a $(DOMAIN_ID) 64 $(PORT) > /dev/null &
	xl console $(DOMAIN_ID)
	-xl destroy $(DOMAIN_ID)

debug:
	gdb -ex "target remote localhost:$(PORT)"
