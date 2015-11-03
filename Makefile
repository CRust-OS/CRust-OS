crust.gz: crust
	gzip -f -9 -c crust > crust.gz

#crust: elfnote.o init_console.o crust.lds
	#ld -o crust -T crust.lds elfnote.o

crust: elfnote.o crust.lds librust_test.a
	ld -o crust -T crust.lds elfnote.o librust_test.a

elfnote.o: elfnote.S
	gcc -c elfnote.S

#init_console.o: init_console.c
	#gcc -I include -c init_console.c

librust_test.a: rust_test.rs
	rustc -Z no-landing-pads rust_test.rs

clean:
	rm *.o
	rm crust
	rm crust.gz
	rm rust_test.a
