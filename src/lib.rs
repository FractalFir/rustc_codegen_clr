#![feature(rustc_private, box_patterns)]
// Used for handling some configs. Will be refactored later.
#![allow(clippy::assertions_on_constants)]
// Not a big issue.
#![allow(clippy::module_name_repetitions)]
// CIL uses the same ops to load signed and unsigned constants, so an usingned number must be first cast to a signed one in order to be stored in CIL.
#![allow(clippy::cast_possible_wrap)]
//#![deny(missing_docs)]
//#![warn(clippy::missing_docs_in_private_items)]
//! Rustc Codegen CLR - an experimental rustc backend compiling Rust for .NET. This project aims to bring the speed and memory efficency of Rust to .NET.
//!
//! # Explaing the project
//!  
//! This part of the documentation aims to help anyone interested in the project better understand the guiding principles behind it, and its architecture.
//! It is a bit more on the techincal side, but please feel free to ask me if anything is unclear.
//!
//! ## Guiding principles
//!
//! The project aims to keep it's codebase as simple as possible, at the cost of increased compile times. Compile times still should be on par with someting like
//! LLVM, due to the project not needing to perform as many complex optimzations.
//!
//! ### Functional
//!
//! The project heavily uses functional programing style. Each element of Rust MIR is handled by a simple pure function, which takes the MIR element,
//! and returns a single translated item, eg. a method, a list of CIL ops, or type definitions. All parameters passed to the functuion are immutable:
//! this heavily simplifies testing, and makes the backend far more predictable. It also makes recovering from errors very easy, since we don't have to deal
//! with potential changes to mutable structutes.
//! This does have its drawbacks(it makes allocating additional local variables harder than it needs to be), but its benefits outhgweight the issues it brings,
//! at least at this point in time.
//!
//! ## Faithful to MIR
//!
//! The project first translates MIR to CIL in a very precise, but inefficient fasion. This inefficent CIL is then optimized using the functions within the `opt` module.
//! This way, it is far less likely that a piece of code will be miscompiled. It also helps with debuging, and allows us to achieve a very high-level translation of MIR.
//!
//! # Where the compilation starts
//!
//! Almost everyting in this file is related to things specific to the rust compiler - reciving MIR from rustc, loading/saving intermediate data,
//! linking the final executable.
//! The compilation process really begins in [`crate::assembly::Assembly::add_item`] - this is where an item - static, function, or inline assembly - gets turned into
//! its .NET representation. The [`crate::assembly::Assembly::add_function`] uses [`crate::assembly::Assembly::add_type`] to add all types needed by a method to the
//! assembly. `add_function` gets the function name, signature, local varaiables and MIR. It uses `handle_statement` and `handle_terminator` turn MIR statements
//! and block terminators into CIL ops.
// TODO: Extend project desctibtion.

// References to internal rustc crates.
extern crate rustc_abi;
extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_symbol_mangling;
extern crate rustc_target;
extern crate rustc_ty_utils;
extern crate stable_mir;
pub mod native_pastrough;
// Modules
/// Specifies if a method/type is private or public.
pub mod access_modifier;
/// Code handling the creation of aggreate values (Arrays, enums,structs,tuples,etc.)
mod aggregate;
/// Representation of a .NET assembly
pub mod assembly;
/// Module containg ILASM-based exporter and code shared between all IL exporter.
pub mod assembly_exporter;
pub mod basic_block;
/// Code handling binary operations
mod binop;
/// Implementation of key external functions(eg. libc) necesary for propely running a Rust executable
pub mod builtin;
mod call_info;
/// Code hansling rust `as` casts.
mod casts;
mod checked_binop;
/// A representation of C# IL op.
pub mod cil;
pub mod cil_tree;
/// Runtime errors and utlity functions/macros related to them
mod codegen_error;
/// Test harnesses.
pub mod compile_test;
/// Code handling loading constant values in CIL.
mod constant;
/// Code detecting and inserting wrappers around entrypoints.
mod entrypoint;
/// Signature of a function (inputs)->output
pub mod function_sig;
/// Interop type handling.
mod interop;
mod verify;
//

/// A representation of a .NET method
pub mod method;
/// Handles a MIR operand.
mod operand;
/// Method-level CIL opitimizations
mod opt;
/// Code handling getting/setting/adressing memory locations.
mod place;
/// Converts righthandside of a MIR statement into CIL ops.
mod rvalue;
/// Code dealing with truning an individual MIR statement into CIL ops.
mod statement;
/// Converts a terminator of a basic block into CIL ops.
mod terminator;
/// Code handling transmutes.
mod transmute;
/// Code related to types.
pub mod r#type;

/// Implementations of unary operations.
mod unop;
/// Contains small helper functions(debug assertions, functions used to get field names, etc), which are frequently used, but are not specific to a part of the coodegen.
mod utilis;

pub mod config;
// rustc functions used here.
use crate::rustc_middle::dep_graph::DepContext;
use rustc_codegen_ssa::{
    back::archive::{
        get_native_object_symbols, ArArchiveBuilder, ArchiveBuilder, ArchiveBuilderBuilder,
    },
    traits::CodegenBackend,
    CodegenResults, CompiledModule, CrateInfo, ModuleKind,
};

use rustc_data_structures::fx::FxIndexMap;
use rustc_metadata::EncodedMetadata;
use rustc_middle::{
    dep_graph::{WorkProduct, WorkProductId},
    ty::TyCtxt,
};
use rustc_session::{
    config::{OutputFilenames, OutputType},
    Session,
};
use rustc_span::ErrorGuaranteed;

use std::{
    any::Any,
    path::{Path, PathBuf},
};
/// Immutable string - used to save a bit of memory on storage.
pub type IString = Box<str>;

use assembly::Assembly;
/// An instance of the codegen.
struct MyBackend;
impl CodegenBackend for MyBackend {
    fn locale_resource(&self) -> &'static str {
        ""
    }
    /// Compiles a crate, and returns its in-memory representaion as a .NET assembly.
    fn codegen_crate<'a>(
        &self,
        tcx: TyCtxt<'_>,
        metadata: EncodedMetadata,
        _need_metadata_module: bool,
    ) -> Box<dyn Any> {
        {
            let (_defid_set, cgus) = tcx.collect_and_partition_mono_items(());

            let mut codegen = Assembly::empty();
            let mut cache = crate::r#type::TyCache::empty();
            for cgu in cgus {
                //println!("codegen {} has {} items.", cgu.name(), cgu.items().len());
                for (item, data) in cgu.items() {
                    // Data will be needed in the future.
                    let _data = data;
                    codegen
                        .add_item(*item, tcx, &mut cache)
                        .expect("Could not add function");
                }
            }
            for type_def in cache.defs() {
                codegen.add_typedef(type_def.clone());
            }
            if let Some((entrypoint, _kind)) = tcx.entry_fn(()) {
                let penv = rustc_middle::ty::ParamEnv::reveal_all();
                let entrypoint = rustc_middle::ty::Instance::resolve(
                    tcx,
                    penv,
                    entrypoint,
                    rustc_middle::ty::List::empty(),
                )
                .expect("Could not resolve entrypoint!")
                .expect("Could not resolve entrypoint!");
                let sig = function_sig::FnSig::sig_from_instance_(entrypoint, tcx, &mut cache)
                    .expect("Could not get the signature of the entrypoint.");
                let symbol = tcx.symbol_name(entrypoint);
                let symbol = format!("{symbol:?}");
                let cs = cil::CallSite::new(None, symbol.into(), sig, true);
                codegen.set_entrypoint(cs);
            }
            codegen.opt();
            // Done twice for inlining!
            codegen.opt();
            let ffi_compile_timer = tcx
                .profiler()
                .generic_activity("insert .NET FFI functions/types");
            builtin::insert_ffi_functions(&mut codegen, tcx);
            drop(ffi_compile_timer);
            let name: IString = cgus.iter().next().unwrap().name().to_string().into();

            Box::new((
                name,
                codegen,
                metadata,
                CrateInfo::new(tcx, "clr".to_string()),
            ))
        }
    }
    /// Saves an in-memory assemably to codegen specific IR in a .bc file.
    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn Any>,
        _sess: &Session,
        outputs: &OutputFilenames,
    ) -> (CodegenResults, FxIndexMap<WorkProductId, WorkProduct>) {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            use std::io::Write;
            let (_asm_name, asm, metadata, crate_info) = *ongoing_codegen
                .downcast::<(IString, Assembly, EncodedMetadata, CrateInfo)>()
                .expect("in join_codegen: ongoing_codegen is not an Assembly");
            let asm_name = "";
            let serialized_asm_path = outputs.temp_path(OutputType::Bitcode, Some(asm_name));
            //std::fs::create_dir_all(&serialized_asm_path).expect("Could not create the directory temporary files are supposed to be in.");
            let mut asm_out = std::fs::File::create(&serialized_asm_path).expect(
                "Could not create the temporary files necessary for building the assembly!",
            );
            asm_out
                .write_all(
                    &postcard::to_stdvec(&asm).expect("Could not serialize the tmp assembly file!"),
                )
                .expect("Could not save the tmp assembly file!");
            let modules = vec![CompiledModule {
                name: asm_name.into(),
                kind: ModuleKind::Regular,
                object: Some(serialized_asm_path),
                bytecode: None,
                dwarf_object: None,
            }];
            let codegen_results = CodegenResults {
                modules,
                allocator_module: None,
                metadata_module: None,
                metadata,
                crate_info,
            };
            (codegen_results, FxIndexMap::default())
        }))
        .expect("Could not join_codegen")
    }
    /// Collects all the files emmited by the codegen for a specific crate, and turns them into a .rlib file containg the serialized assembly IR and metadata.
    fn link(
        &self,
        sess: &Session,
        codegen_results: CodegenResults,
        outputs: &OutputFilenames,
    ) -> Result<(), ErrorGuaranteed> {
        use rustc_codegen_ssa::back::link::link_binary;
        //panic!();
        link_binary(sess, &RlibArchiveBuilder, &codegen_results, outputs)
            .expect("Could not link the binary into a .rlib file!");
        Ok(())
    }
}
// Inspired by cranelifts glue code. Is responsible for turing the files produced by teh backend into
struct RlibArchiveBuilder;
impl ArchiveBuilderBuilder for RlibArchiveBuilder {
    fn new_archive_builder<'a>(&self, sess: &'a Session) -> Box<dyn ArchiveBuilder + 'a> {
        Box::new(ArArchiveBuilder::new(sess, get_native_object_symbols))
    }
    fn create_dll_import_lib(
        &self,
        _sess: &Session,
        _lib_name: &str,
        _dll_imports: &[rustc_session::cstore::DllImport],
        _tmpdir: &Path,
        _is_direct_dependency: bool,
    ) -> PathBuf {
        unimplemented!("creating dll imports is not supported");
    }
}
#[no_mangle]
/// Entrypoint of the codegen. This function starts the backend up, and returns a reference to it to rustc.
pub extern "Rust" fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(MyBackend)
}
