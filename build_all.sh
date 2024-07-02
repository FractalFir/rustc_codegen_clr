#!/bin/sh
cd cilly && cargo build --all --release && cargo build --all && cd .. && cargo build && cargo build --release 
