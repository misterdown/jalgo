all: 
	./jalgo.exe ./examples/rule110.jalgo ./rule110.asm c
	nasm -f win64 rule110.asm -o rule110.o
	ld -o rule110.exe rule110.o -lkernel32 -lmsvcrt
nj: 
	nasm -f win64 rule110.asm -o rule110.o -O0
	ld -o rule110.exe rule110.o -lkernel32 -lmsvcrt