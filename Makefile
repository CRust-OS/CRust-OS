crust.gz: crust
	gzip -f -9 -c crust > crust.gz

crust: elfnote.o init_console.o crust.lds
	ld -o crust -T crust.lds elfnote.o

elfnote.o: elfnote.S
	gcc -c elfnote.S

init_console.o: init_console.c
	gcc -I include -c init_console.c

clean:
	rm *.o
	rm crust
	rm crust.gz
