all: 
	cargo build
	./target/debug/jalgo.exe ./examples/rule110.jalgo ./rule110 c
