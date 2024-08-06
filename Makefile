ROOT := $(dir $(abspath $(lastword %(MAKEFILE_LIST))))

.PHONY: run

init:
	cd $(ROOT)/driver && cargo run -- --init

run:
	cd $(ROOT)/driver && cargo run -- --utrace=~/islet/rmm --filter=islet_rmm --verbose

.PHONY: example

example:
	cd $(ROOT)/out && rm -rf *
	cd $(ROOT)/plugin && cargo run ../examples/unsafe-keyword.rs
