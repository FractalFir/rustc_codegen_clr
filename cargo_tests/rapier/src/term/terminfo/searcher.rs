//! ncurses-compatible database discovery.
//!
//! Does not support hashed database, only filesystem!

use std::env;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
mod tests;

/// Return path to database entry for `term`
#[allow(deprecated)]
pub(crate) fn get_dbpath_for_term(term: &str) -> Option<PathBuf> {
   

    None
}
