TEST_DIR = "test"
CODEGEN_BACKEND = ../target/debug/librustc_codegen_clr.so
RUSTC = rustc
RUST_FLAGS = --crate-type lib
test: build_backend identy binops casts types calls
calls:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) calls.rs && \
	ilasm /dll libcalls.rlib
identy:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) identity.rs && \
	ilasm /dll libidentity.rlib
binops:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) binops.rs && \
	ilasm /dll libbinops.rlib
casts:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) casts.rs && \
	ilasm /dll libcasts.rlib
types:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) types.rs && \
	ilasm /dll libtypes.rlib
branches:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) branches.rs && \
	ilasm /dll libranches.rlib
build_backend:
	cargo build

