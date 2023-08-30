TEST_DIR = "test"
CODEGEN_BACKEND = ../target/debug/librustc_codegen_clr.so
RUSTC = rustc
DEBUGER = #echo n > gdb -ex='set confirm on' -ex=run -ex=quit --args 
RUST_FLAGS =
# this function compiles a particular test lib
compile_lib = cd test && rustc -O --emit=mir --crate-type=lib $(1).rs && $(DEBUGER) rustc $(RUST_FLAGS) --crate-type lib -O -Z codegen-backend=$(CODEGEN_BACKEND) $(1).rs -o libs/$(1).rlib

test: setup build_backend identity branches binops casts types calls references structs libc nbody hello
setup:
	cd test && mkdir -p libs
build_backend:
	cargo build
calls:
	$(call compile_lib,calls)
libc:
	$(call compile_lib,libc)
identity:
	$(call compile_lib,identity)
binops:
	$(call compile_lib,binops)
references:
	$(call compile_lib,references)
nbody:
	$(call compile_lib,nbody)
casts:
	$(call compile_lib,casts)
types:
	$(call compile_lib,types)
branches:
	$(call compile_lib,branches)
structs:
	$(call compile_lib,structs)
hello:
	cd test && $(DEBUGER) rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) hello.rs && \
	ilasm hello && \
	rustc -O --emit=mir hello.rs

