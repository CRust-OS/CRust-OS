crust.gz: crust
	gzip -f -9 -c crust > crust.gz

crust: elfnote.o
	ld -o crust -T crust.lds elfnote.o

elfnote.o: elfnote.S
	gcc -c elfnote.S

clean:
	rm *.o
	rm crust
	rm crust.gz
