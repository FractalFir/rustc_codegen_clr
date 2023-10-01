#!/bin/bash  
BACKEND_PATH=`readlink -f ../../target/release/librustc_codegen_clr.so`
RUSTFLAGS="-Z codegen-backend=$BACKEND_PATH" cargo build --release
