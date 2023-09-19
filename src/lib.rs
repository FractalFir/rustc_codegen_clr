#![feature(rustc_private)]
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
use rustc_middle::ty::{Binder, BoundVariableKind};
fn skip_binder_if_no_generic_types<T>(binder: Binder<T>) -> Option<T> {
    if binder
        .bound_vars()
        .iter()
        .any(|bound_var_kind| matches!(bound_var_kind, BoundVariableKind::Ty(_)))
    {
        None
    } else {
        Some(binder.skip_binder())
    }
}
use rustc_codegen_ssa::{
    traits::CodegenBackend, CodegenResults, CompiledModule, CrateInfo, ModuleKind,
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
use std::any::Any;
pub type IString = Box<str>;
mod access_modifier;
mod assembly;
mod assembly_exporter;
mod binop;
mod casts;
mod cil_op;
mod codegen_error;
mod compile_test;
mod constant;
mod entrypoint;
mod function_sig;
mod method;
mod operand;
mod place;
mod rvalue;
mod statement;
mod stdlib;
mod terminator;
mod r#type;
mod type_def;
mod unop;
mod utilis;
use assembly::Assembly;
struct MyBackend;
pub(crate) const ALWAYS_INIT_STRUCTS: bool = false;
pub(crate) const ALWAYS_INIT_LOCALS: bool = false;

impl CodegenBackend for MyBackend {
    fn locale_resource(&self) -> &'static str {
        ""
    }
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

        if let Some((entrypoint, kind)) = tcx.entry_fn(()) {
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
            let sig = function_sig::FnSig::from_poly_sig(&entrypoint_fn, tcx)
                .expect("Could not get the signature of the entrypoint.");
            let symbol = tcx.symbol_name(entrypoint);
            let symbol = format!("{symbol:?}");
            let cs = cil_op::CallSite::new(None, symbol.into(), sig, true);
            codegen.set_entrypoint(cs);
        }

        let name: IString = cgus.iter().next().unwrap().name().to_string().into();
        Box::new((
            name,
            codegen,
            metadata,
            CrateInfo::new(tcx, "clr".to_string()),
        ))
    }

    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn Any>,
        _sess: &Session,
        outputs: &OutputFilenames,
    ) -> Result<(CodegenResults, FxIndexMap<WorkProductId, WorkProduct>), ErrorGuaranteed> {
        use std::io::Write;
        let (asm_name, asm, metadata, crate_info) = *ongoing_codegen
            .downcast::<(IString, Assembly, EncodedMetadata, CrateInfo)>()
            .expect("in join_codegen: ongoing_codegen is not an Assembly");
        let asm_name = "";
        let serialized_asm_path = outputs.temp_path(OutputType::Object, Some(asm_name));
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

    fn link(
        &self,
        sess: &Session,
        codegen_results: CodegenResults,
        outputs: &OutputFilenames,
    ) -> Result<(), ErrorGuaranteed> {
        use rustc_session::{
            config::{CrateType, OutFileName},
            output::out_filename,
        };

        let crate_name = codegen_results.crate_info.local_crate_name;
        let mut final_assembly = Assembly::empty();
        for module in codegen_results.modules {
            use std::io::Read;

            let asm_path = module.object.expect("ERROR: object file path is missing!");
            let mut asm_file =
                std::fs::File::open(asm_path).expect("ERROR:Could not open the assembly file!");
            let mut asm_bytes = Vec::with_capacity(0x100);
            asm_file
                .read_to_end(&mut asm_bytes)
                .expect("ERROR:Could not load the assembly file!");
            let assembly = postcard::from_bytes(&asm_bytes)
                .expect("ERROR:Could not decode the assembly file!");
            final_assembly = final_assembly.join(assembly);
        }
        crate::stdlib::insert_libc(&mut final_assembly);
        println!(
            "PERPARING TO EMMIT FINAL CRATE! CRATE COUNT: {}",
            sess.opts.crate_types.len()
        );
        use crate::assembly_exporter::AssemblyExporter;
        let path = out_filename(sess, CrateType::Rlib, outputs, crate_name);

        let path = match path {
            OutFileName::Real(ref path) => path.to_owned(),
            OutFileName::Stdout => panic!("Compiling to stdout is not supported!"),
        };
        std::fs::create_dir_all(path.parent().expect("Could not get the target directory"))
            .expect("Could not create the target directory!");
        crate::assembly_exporter::ilasm_exporter::ILASMExporter::export_assembly(
            &final_assembly,
            &path,
        )
        .expect("Could not create the final asm!");
        Ok(())
    }
}

#[no_mangle]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(MyBackend)
}
