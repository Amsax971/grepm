install:
	cargo build
	sudo cp ./target/debug/grepm /usr/bin/

uninstall:
	sudo rm -rf /usr/bin/grepm