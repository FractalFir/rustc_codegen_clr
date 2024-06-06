#![feature(fmt_internals,sync_unsafe_cell,numfmt,core_intrinsics,flt2dec,no_sanitize,extern_types,specialization,maybe_uninit_uninit_array,maybe_uninit_slice,never_type,exposed_provenance)]
#![feature(adt_const_params,test)]
use std::fmt::Debug;

//mod fmt;
mod term;
use std::fs::File;
use std::hint;
use std::io::{self, Write, Read};
use std::net::TcpStream;

use term::terminfo::searcher::get_dbpath_for_term;
fn main() {
    let mut homedir = std::hint::black_box(std::path::PathBuf::from("/home/myusername"));
    homedir.push(".terminfo");
    std::hint::black_box(&homedir);
}
