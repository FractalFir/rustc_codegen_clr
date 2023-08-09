TEST_DIR = "test"
CODEGEN_BACKEND = target/debug/librustc_codegen_clr.so
RUSTC = rustc
RUST_FLAGS = --crate-type lib
test: build_backend compile_simple_file
compile_simple_file:
	rustc $(RUST_FLAGS) -Z codegen-backend=$(CODEGEN_BACKEND) test/identity.rs
build_backend:
	cargo build

