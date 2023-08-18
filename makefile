TEST_DIR = "test"
CODEGEN_BACKEND = ../target/debug/librustc_codegen_clr.so
RUSTC = rustc
RUST_FLAGS = --crate-type lib
test: build_backend identy binops casts types calls references structs
calls:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) calls.rs && \
	ilasm /dll libcalls.rlib && \
	rustc -O --emit=mir --crate-type=lib calls.rs
identy:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) identity.rs && \
	ilasm /dll libidentity.rlib
binops:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) binops.rs && \
	ilasm /dll libbinops.rlib && \
	rustc -O --emit=mir --crate-type=lib binops.rs
references:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) references.rs && \
	ilasm /dll libreferences.rlib && \
	rustc -O --emit=mir --crate-type=lib references.rs
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
structs:
	cd test && rustc $(RUST_FLAGS) -O -Z codegen-backend=$(CODEGEN_BACKEND) structs.rs && \
	ilasm /dll libstructs.rlib && \
	rustc -O --emit=mir --crate-type=lib structs.rs

