debug : 
	cargo build
	cp ./target/debug/libbfmat.a ./src
	matlab ./matlab/build.m

release : 
	cargo build --release
	cp ./target/release/libbfmat.a ./src
	matlab ./matlab/build.m
