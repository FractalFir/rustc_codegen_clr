TEST_DIR = "test"
CODEGEN_BACKEND = ../target/debug/librustc_codegen_clr.so
RUSTC = rustc
RUST_FLAGS = --crate-type lib
test: build_backend identy binops casts types
identy:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) identity.rs
binops:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) binops.rs
casts:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) casts.rs
types:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) types.rs
branches:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) branches.rs
build_backend:
	cargo build

