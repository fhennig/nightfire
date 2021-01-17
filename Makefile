docker:
	docker build -t nightfire-build-armhf:latest .

bin:
	cross build --release --target armv7-unknown-linux-gnueabihf
