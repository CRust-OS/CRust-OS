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
