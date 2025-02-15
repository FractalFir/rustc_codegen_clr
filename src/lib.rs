#![feature(rustc_private)]
#![feature(let_chains)]
#![feature(f16, alloc_error_hook)]
#![warn(clippy::pedantic)]
// Used for handling some configs. Will be refactored later.
#![allow(clippy::assertions_on_constants)]
// The complexity is managable for now.
#![allow(clippy::too_many_lines)]
// Not a big issue.
#![allow(clippy::module_name_repetitions)]
// docs are WIP
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::module_inception
)]
//#![warn(missing_docs)]
//#![warn(clippy::missing_docs_in_private_items)]

//#![deny(dead_code)]

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
//! One notable exeception to this rule is the [`crate::type::tycache::TyCache`] - a structure used for caching type translations. Since it needs to perform some expensive work(eg. find `core::ptr::metadata::PtrComponents`)
//! upfront, reusing the `TyCache` for a whole codegen unit is needed. Thus, it is passed by a mutable reference. `TyCache` can be easily reset after a panic, ensuirng panic recovery is safe.
//!
//! ## Faithful to MIR
//!
//! The project first translates MIR to CIL in a very precise, but inefficient fasion. This is a deliberate choice - it recduces the chance of bugs, and enables easy checking of the resulting CIL.
//!
//! Since any given  MIR statement will always result in the same ops, and the ops from each statement are kept separate, any misformed piece of CIL byecode can be easily traced back to a
//! particular MIR statement.
//!
//! This way, it is far less likely that a piece of code will be miscompiled. It also helps with debuging, and allows us to achieve a very high-level translation of MIR.
//!
//! This intermediate, inefficent CIL can be optimized using the functions within the [`crate::opt`] module. Those optimzations are allowed to do things like reorder statements, remove/add locals, etc.
//! So, when debuging issues, it is recomeded the additional optimzations be turned off by seting the enviroment varaible `OPTIMIZE_CIL` to 0.
//!
//! ## Internal IR
//!
//! The project-internal IR(CIL trees) is defined in the module [`crate::cil_tree`]. Additional CIL-related data structures, such as call targets and field descriptors can be found in [`crate::cil`].
//! [`crate::cil_tree`] will also contain a brief overview of the CIL represenation used by the project.
//!
//! ## Type represenation
//!
//! All type-related data structures are defined in the module [`crate::type`]
//!
//! ## MIR handling
//!
//! Each MIR element is handled by a function defined in a module with the corresponding name. For example, MIR statements are handled by the function [`crate::statement::handle_statement`].
//!
//! # Where the compilation starts
//!
//! Almost everyting in this file is related to things specific to the rust compiler - reciving MIR from rustc, loading/saving intermediate data,
//! linking the final executable.
//! The compilation process really begins in [`crate::assembly::add_item`] - this is where an item - static, function, or inline assembly - gets turned into
//! its .NET representation. The [`crate::assembly::add_fn`] uses [`cilly::asm::Assembly::add_typedef`] to add all types needed by a method to the
//! assembly. `add_fn` gets the function name, signature, local varaiables and MIR. It uses `handle_statement` and `handle_terminator` turn MIR statements
//! and block terminators into CIL ops.
// TODO: Extend project desctiption.

// References to internal rustc crates.
extern crate rustc_abi;

extern crate rustc_codegen_ssa;
extern crate rustc_const_eval;
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
/// Used for handling OOM in compiler
mod alloc_erorr_hook;

// Modules
/// Code handling the creation of aggreate values (Arrays, enums,structs,tuples,etc.)
mod aggregate;
/// Representation of a .NET assembly
pub mod assembly;
/// Moudle containing defintion of basic blocks and method operating on them.
pub mod basic_block;
/// Code handling binary operations
mod binop;
/// Implementation of key external functions(eg. libc) necesary for propely running a Rust executable
pub mod builtin;
mod call_info;
/// Code hansling rust `as` casts.
mod casts;
/// Method compilation context
mod fn_ctx;
pub mod native_pastrough;

/// A representation of C# IL op.
pub mod cil;
/// Runtime errors and utlity functions/macros related to them
mod codegen_error;
/// Test harnesses.
pub mod compile_test;
/// Code handling loading constant values in CIL.
mod constant;

/// Implementation of compiletime features neccessary for interop.
mod comptime;
/// Signature of a function (inputs)->output
pub mod function_sig;
/// Interop type handling.
mod interop;

/// A representation of a .NET method
pub mod method;
/// Handles a MIR operand.
mod operand;

/// Code handling getting/setting/adressing memory locations.
mod place;
/// Converts righthandside of a MIR statement into CIL ops.
mod rvalue;
/// Code dealing with truning an individual MIR statement into CIL ops.
pub mod statement;
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
mod unsize;
// rustc functions used here.
use crate::rustc_middle::dep_graph::DepContext;
use cilly::{
    v2::{cilnode::MethodKind, MethodRef},
    Assembly,
};
use fn_ctx::MethodCompileCtx;
use rustc_codegen_ssa::{
    back::archive::{ArArchiveBuilder, ArchiveBuilder, ArchiveBuilderBuilder},
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

use std::{any::Any, path::Path};
/// Immutable string - used to save a bit of memory on storage.
pub type IString = cilly::IString;
/// Immutable string - used to save a bit of memory on storage.
pub type AString = std::sync::Arc<Box<str>>;

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
        need_metadata_module: bool,
    ) -> Box<dyn Any> {
        {
            let cgus = tcx.collect_and_partition_mono_items(());

            let mut asm = Assembly::default();
            if need_metadata_module {
                use std::io::Write;
                let mut packed_metadata = rustc_metadata::METADATA_HEADER.to_vec();
                packed_metadata
                    .write_all(&(metadata.raw_data().len() as u64).to_le_bytes())
                    .unwrap();
                packed_metadata.extend(metadata.raw_data());
                asm.add_section(".rustc", packed_metadata);
            }
            let _ = cilly::utilis::get_environ(&mut asm);

            for cgu in cgus.codegen_units {
                //println!("codegen {} has {} items.", cgu.name(), cgu.items().len());
                for (item, _data) in cgu.items() {
                    assembly::add_item(&mut asm, *item, tcx).expect("Could not add function");
                }
            }

            if let Some((entrypoint, _kind)) = tcx.entry_fn(()) {
                let penv = rustc_middle::ty::TypingEnv::fully_monomorphized();
                let entrypoint = rustc_middle::ty::Instance::try_resolve(
                    tcx,
                    penv,
                    entrypoint,
                    rustc_middle::ty::List::empty(),
                )
                .expect("Could not resolve entrypoint!")
                .expect("Could not resolve entrypoint!");
                let mut ctx = MethodCompileCtx::new(tcx, None, entrypoint, &mut asm);
                let sig = function_sig::sig_from_instance_(entrypoint, &mut ctx)
                    .expect("Could not get the signature of the entrypoint.");
                let symbol = tcx.symbol_name(entrypoint);
                let symbol = format!("{symbol:?}");
                let cs = MethodRef::new(
                    *asm.main_module(),
                    asm.alloc_string(symbol),
                    asm.alloc_sig(sig),
                    MethodKind::Static,
                    vec![].into(),
                );

                cilly::entrypoint::wrapper(cs, &mut asm);
            }

            let ffi_compile_timer = tcx
                .profiler()
                .generic_activity("insert .NET FFI functions/types");
            //builtin::insert_ffi_functions(&mut asm, tcx);
            drop(ffi_compile_timer);
            let name: IString = cgus
                .codegen_units
                .iter()
                .next()
                .unwrap()
                .name()
                .to_string()
                .into();

            Box::new((name, asm, metadata, CrateInfo::new(tcx, "clr".to_string())))
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
            let mut v2 = cilly::v2::Assembly::from_v1(&asm);
            v2.opt(&mut v2.fuel_from_env());
            v2.typecheck();
            asm_out
                .write_all(
                    &postcard::to_stdvec(&v2).expect("Could not serialize the tmp assembly file!"),
                )
                .expect("Could not save the tmp assembly file!");
            let modules = vec![CompiledModule {
                name: asm_name.into(),
                kind: ModuleKind::Regular,
                object: Some(serialized_asm_path),
                bytecode: None,
                dwarf_object: None,
                llvm_ir: None,
                assembly: None,
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
    fn link(&self, sess: &Session, codegen_results: CodegenResults, outputs: &OutputFilenames) {
        use rustc_codegen_ssa::back::link::link_binary;
        link_binary(sess, &RlibArchiveBuilder, codegen_results, outputs);
    }
}
// Inspired by cranelifts glue code. Is responsible for turing the files produced by teh backend into
struct RlibArchiveBuilder;
impl ArchiveBuilderBuilder for RlibArchiveBuilder {
    fn new_archive_builder<'a>(&self, sess: &'a Session) -> Box<dyn ArchiveBuilder + 'a> {
        Box::new(ArArchiveBuilder::new(
            sess,
            &rustc_codegen_ssa::back::archive::DEFAULT_OBJECT_READER,
        ))
    }
    fn create_dll_import_lib(
        &self,
        _sess: &Session,
        _lib_name: &str,
        _dll_imports: std::vec::Vec<rustc_codegen_ssa::back::archive::ImportLibraryItem>,
        _tmpdir: &Path,
    ) {
        unimplemented!("creating dll imports is not supported");
    }
}
#[no_mangle]
/// Entrypoint of the codegen. This function starts the backend up, and returns a reference to it to rustc.
pub extern "Rust" fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    std::alloc::set_alloc_error_hook(alloc_erorr_hook::custom_alloc_error_hook);
    Box::new(MyBackend)
}
pub use cilly::{DATA_PTR, ENUM_TAG, METADATA};
