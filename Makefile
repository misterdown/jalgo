all: 
	./jalgo.exe ./examples/rule110.jalgo ./rule110.nasm c
	nasm -f win64 rule110.nasm -o rule110.o
	ld -o rule110.exe rule110.o -lkernel32 -lmsvcrt
