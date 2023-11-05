#![feature(rustc_private)]
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
extern crate rustc_target;

// Debug config

/// Tells the codegen to insert comments containing the MIR statemtens after each one of them.
const INSERT_MIR_DEBUG_COMMENTS: bool = false;
const PRINT_LOCAL_TYPES: bool = false;
const PRINT_TY_CONVERTION: bool = false;
/// Tells the codegen to optmize the emiited CIL.
const OPTIMIZE_CIL: bool = (!INSERT_MIR_DEBUG_COMMENTS) && (true);

/// Try turining on in cause of issues. If it fixes them, then their root cause is UB(eg. use of uninitailized memory).
pub const ALWAYS_INIT_LOCALS:bool = false;
pub const ABORT_ON_ERROR: bool = false;

// Modules

/// Specifies if a method/type is private or public.
pub mod access_modifier;
/// Code handling the creation of aggreate values (Arrays, enums,structs,tuples,etc.)
mod aggregate;

/// Representation of a .NET assembly
pub mod assembly;
/// Module containg ILASM-based exporter and code shared between all IL exporter.
pub mod assembly_exporter;
mod basic_block;
/// Code handling binary operations
mod binop;

/// Code hansling rust `as` casts.
mod casts;
mod checked_binop;
/// A representation of C# IL op.
pub mod cil_op;
/// Runtime errors and utlity functions/macros related to them
mod codegen_error;
/// Test harnesses.
mod compile_test;
/// Code handling loading constant values in CIL.
mod constant;
/// Code detecting and inserting wrappers around entrypoints.
mod entrypoint;
/// Signature of a function (inputs)->output
pub mod function_sig;
/// Implementation of some libc functions in CIL assembly. Will likely be removed and mostly replaced by functions implmented using mycorrhize.
pub mod libc;
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
/// A representation of a primitve type or a reference.
mod r#type;
/// Contains a reperesentation of a non-primitve .NET type(class,struct)
mod type_def;
/// Implementations of unary operations.
mod unop;
/// Contains small helper functions(debug assertions, functions used to get field names, etc), which are frequently used, but are not specific to a part of the coodegen.
mod utilis;
// rustc functions used here.
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
        let (_defid_set, cgus) = tcx.collect_and_partition_mono_items(());

        let mut codegen = Assembly::empty();
        for cgu in cgus {
            //println!("codegen {} has {} items.", cgu.name(), cgu.items().len());
            for (item, _data) in cgu.items() {
                codegen
                    .add_item(*item, tcx)
                    .expect("Could not add function");
            }
        }

        if let Some((entrypoint, _kind)) = tcx.entry_fn(()) {
            let penv = rustc_middle::ty::ParamEnv::empty();
            let entrypoint = rustc_middle::ty::Instance::resolve(
                tcx,
                penv,
                entrypoint,
                rustc_middle::ty::List::empty(),
            )
            .expect("Could not resolve entrypoint!")
            .expect("Could not resolve entrypoint!");
            let entrypoint_fn = entrypoint.ty(tcx, penv).fn_sig(tcx);
            let sig = function_sig::FnSig::from_poly_sig(&entrypoint_fn, tcx, &entrypoint)
                .expect("Could not get the signature of the entrypoint.");
            let symbol = tcx.symbol_name(entrypoint);
            let symbol = format!("{symbol:?}");
            let cs = cil_op::CallSite::new(None, symbol.into(), sig, true);
            codegen.set_entrypoint(cs);
        }
        codegen.opt();
        let name: IString = cgus.iter().next().unwrap().name().to_string().into();
        Box::new((
            name,
            codegen,
            metadata,
            CrateInfo::new(tcx, "clr".to_string()),
        ))
    }
    /// Saves an in-memory assemably to codegen specific IR in a .bc file.
    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn Any>,
        _sess: &Session,
        outputs: &OutputFilenames,
    ) -> Result<(CodegenResults, FxIndexMap<WorkProductId, WorkProduct>), ErrorGuaranteed> {
        use std::io::Write;
        let (_asm_name, asm, metadata, crate_info) = *ongoing_codegen
            .downcast::<(IString, Assembly, EncodedMetadata, CrateInfo)>()
            .expect("in join_codegen: ongoing_codegen is not an Assembly");
        let asm_name = "";
        let serialized_asm_path = outputs.temp_path(OutputType::Bitcode, Some(asm_name));
        //std::fs::create_dir_all(&serialized_asm_path).expect("Could not create the directory temporary files are supposed to be in.");
        let mut asm_out = std::fs::File::create(&serialized_asm_path)
            .expect("Could not create the temporary files necessary for building the assembly!");
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
        Ok((codegen_results, FxIndexMap::default()))
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
    fn new_archive_builder<'a>(&self, sess: &'a Session) -> Box<dyn ArchiveBuilder<'a> + 'a> {
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
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(MyBackend)
}
