crust.gz: crust
	gzip -f -9 -c crust > crust.gz

#crust: elfnote.o init_console.o crust.lds
	#ld -o crust -T crust.lds elfnote.o

crust: elfnote.o crust.lds librustlib.a
	ld -o crust -T crust.lds elfnote.o librustlib.a

elfnote.o: elfnote.S
	gcc -gstabs -c elfnote.S

#init_console.o: init_console.c
	#gcc -I include -c init_console.c

#librust_test.a: rust_test.rs
	#rustc -Z no-landing-pads rust_test.rs rlibc.rs

#librlibc.a: rlibc.rs
	#rustc -Z no-landing-pads --crate-type staticlib rlibc.rs

librustlib.a: rust/rlibc.rs rust/rust_test.rs rust/rustlib.rs
	rustc -g -Z no-landing-pads rust/rustlib.rs

clean:
	-rm *.o
	-rm librustlib.a
	-rm crust
	-rm crust.gz
