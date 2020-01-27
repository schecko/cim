.PHONY: build
SOURCE=source ~/.cargo/env
SHELL=/bin/bash

run:
	$(SOURCE) && cargo run

r: run

build:
	$(SOURCE) && cargo build

clean:
	$(SOURCE) && cargo clean
