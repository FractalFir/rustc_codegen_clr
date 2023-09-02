TEST_DIR = "test"
CODEGEN_BACKEND = ../target/debug/librustc_codegen_clr.so
RUSTC = rustc
DEBUGER = #echo n > gdb -ex='set confirm on' -ex=run -ex=quit --args 
RUST_FLAGS =
# this function compiles a particular test lib
compile_lib = cd test && rustc -O --emit=mir --crate-type=lib $(1).rs && $(DEBUGER) rustc $(RUST_FLAGS) --crate-type lib -O -Z codegen-backend=$(CODEGEN_BACKEND) $(1).rs -o libs/$(1).rlib

test: hello
hello:
	cd test && $(DEBUGER) rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) hello.rs && \
	ilasm hello && \
	rustc -O --emit=mir hello.rs

