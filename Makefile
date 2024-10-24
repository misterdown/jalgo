all: 
	cargo build
	./target/debug/jalgo.exe ./examples/rule110.jalgo ./rule110 c
	./target/debug/jalgo.exe ./examples/HelloWorld.jalgo ./HelloWorld c
	./target/debug/jalgo.exe ./examples/HelloWorld.jalgo ./HelloWorld i