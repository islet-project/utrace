ROOT := $(dir $(abspath $(lastword %(MAKEFILE_LIST))))
CRATE_PATH ?= ~/islet/plat/fvp
CRATE_NAME ?= "islet_rmm, fvp, vmsa, uart"

.PHONY: init
init:
	cd $(ROOT)/driver && cargo run --release -- --init

.PHONY: summary
summary:
	cd $(ROOT)/driver
	cargo run -- --utrace=$(CRATE_PATH)

.PHONY: unsafe-list
unsafe-list:
	cd $(ROOT)/driver
	cargo run -- --utrace=$(CRATE_PATH) --filter=$(CRATE_NAME) --verbose

.PHONY: call-trace
call-trace:
	cd $(ROOT)/driver
	cargo run -- --utrace=$(CRATE_PATH) --filter=$(CRATE_NAME) --call-trace

.PHONY: example
example:
	cd $(ROOT)/out && rm -rf *
	cd $(ROOT)/plugin && cargo run ../examples/unsafe-keyword.rs
