#![feature(rustc_private)]
extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_index;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;

use rustc_codegen_ssa::{
    traits::CodegenBackend, CodegenResults, CompiledModule, CrateInfo, ModuleKind,
};
use rustc_data_structures::fx::FxIndexMap;
use rustc_metadata::EncodedMetadata;
use rustc_middle::{
    dep_graph::{WorkProduct, WorkProductId},
    ty::{PolyFnSig, TyCtxt},
};

use rustc_session::{
    config::{OutputFilenames, OutputType},
    Session,
};
use rustc_span::ErrorGuaranteed;
use serde::{Deserialize, Serialize};
use std::any::Any;

mod clr_method;
use clr_method::*;
mod assembly;
use assembly::*;
mod base_ir;
use base_ir::BaseIR;
mod variable;
use variable::*;
mod assigment_target;
mod projection;
mod statement;
pub type IString = Box<str>;

struct MyBackend;
pub(crate) const ALWAYS_INIT_STRUCTS:bool = false;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct FunctionSignature {
    inputs: Box<[VariableType]>,
    output: VariableType,
}
impl FunctionSignature {
    pub(crate) fn inputs(&self) -> &[VariableType] {
        &self.inputs
    }
    pub(crate) fn output(&self) -> &VariableType {
        &self.output
    }
    pub(crate) fn new(inputs: &[VariableType], output: &VariableType) -> Self {
        Self {
            inputs: inputs.into(),
            output: output.clone(),
        }
    }
    pub(crate) fn from_poly_sig(sig: PolyFnSig,tyctx:TyCtxt) -> Option<Self> {
        let inputs = sig
            .inputs()
            // `skip_binder` is `a riskiy thing` TODO: Figure out to which kind of issues it may lead!
            .skip_binder()
            //.no_bound_vars()?
            .iter()
            .map(|v| VariableType::from_ty(*v,tyctx))
            .collect();
        let output = VariableType::from_ty(
            sig.output()
                // `skip_binder` is `a riskiy thing` TODO: Figure out to which kind of issues it may lead!
                .skip_binder(), //.no_bound_vars()?
                tyctx
        );
        Some(Self { inputs, output })
    }
}

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
        let mut codegen = Assembly::new(&cgus.iter().next().unwrap().name().to_string());
        for cgu in cgus {
            //println!("codegen {} has {} items.", cgu.name(), cgu.items().len());
            for (item, _data) in cgu.items() {
                codegen.add_item(*item, tcx);
            }
        }
        //println!("CLR IL:\n```\n{ir}\n```", ir = codegen.into_il_ir());

        Box::new((codegen, metadata, CrateInfo::new(tcx, "clr".to_string())))
    }

    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn Any>,
        _sess: &Session,
        outputs: &OutputFilenames,
    ) -> Result<(CodegenResults, FxIndexMap<WorkProductId, WorkProduct>), ErrorGuaranteed> {
        use std::io::Write;
        let (asm, metadata, crate_info) = *ongoing_codegen
            .downcast::<(Assembly, EncodedMetadata, CrateInfo)>()
            .expect("in join_codegen: ongoing_codegen is not an Assembly");

        let serialized_asm_path = outputs.temp_path(OutputType::Object, Some(asm.name()));
        //std::fs::create_dir_all(&serialized_asm_path).expect("Could not create the directory temporary files are supposed to be in.");
        let mut asm_out = std::fs::File::create(&serialized_asm_path)
            .expect("Could not create the temporary files necessary for building the assembly!");
        asm_out
            .write_all(
                &postcard::to_stdvec(&asm).expect("Could not serialize the tmp assembly file!"),
            )
            .expect("Could not save the tmp assembly file!");
        let modules = vec![CompiledModule {
            name: asm.name().to_owned(),
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
        use std::io::Write;
        let crate_name = codegen_results.crate_info.local_crate_name;
        let mut final_assembly =
            Assembly::new(&codegen_results.crate_info.local_crate_name.to_string());
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
            final_assembly.link(assembly);
        }
        for &crate_type in sess.opts.crate_types.iter() {
            if crate_type != CrateType::Rlib {
                sess.fatal(format!("Crate type is {:?}", crate_type));
            }
            let output_name = out_filename(sess, crate_type, outputs, crate_name);
            match output_name {
                OutFileName::Real(ref path) => {
                    let mut out_file = std::fs::File::create(path).unwrap();
                    let asm_il = final_assembly.into_il_ir();
                    write!(out_file, "{}", asm_il).unwrap();
                }
                OutFileName::Stdout => {
                    let mut stdout = std::io::stdout();
                    write!(stdout, "This has been \"compiled\" successfully.").unwrap();
                }
            }
        }
        Ok(())
    }
}

#[no_mangle]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(MyBackend)
}
