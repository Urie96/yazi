.PHONY: install
install: 
	cargo build --release
	cp ./target/release/yazi ~/.cargo/bin/yazi