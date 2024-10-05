all: 
	./jalgo.exe ./some.jalgo ./result.nasm c
	nasm -f win64 result.nasm -o result.o
	ld -o result.exe result.o -lkernel32 -lmsvcrt