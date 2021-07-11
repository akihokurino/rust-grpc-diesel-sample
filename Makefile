MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

build:
	cargo build

run-local:
	cargo run

run-grpcui:
	grpcui -plaintext -port 4000 localhost:3000