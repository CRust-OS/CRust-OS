crust: elfnote.o
	ld -o crust -T crust.lds elfnote.o

elfnote.o: elfnote.S
	gcc -c elfnote.S
