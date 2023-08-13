#![feature(rustc_private)]
extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use rustc_codegen_ssa::CrateInfo;
use rustc_codegen_ssa::{traits::CodegenBackend, CodegenResults};
use rustc_metadata::EncodedMetadata;
use rustc_middle::{
    dep_graph::{WorkProduct, WorkProductId},
    ty::{Ty,TyCtxt,TyKind,IntTy,UintTy},
};
use rustc_span::ErrorGuaranteed;
use std::any::Any;
use rustc_middle::ty::PolyFnSig;
use rustc_data_structures::fx::FxIndexMap;
use rustc_session::{config::OutputFilenames, Session};

mod clr_method;
use clr_method::*;
mod assembly;
use assembly::*;
mod base_ir;
use base_ir::BaseIR;
pub type IString = Box<str>;

struct MyBackend;
#[derive(Clone,Debug)]
enum VariableType{
    I8,I16,I32,I64,I128,ISize,
    U8,U16,U32,U64,U128,USize,
    Bool,
}
#[derive(Debug)]
struct FunctionSignature{
    inputs:Box<[VariableType]>,
    output:VariableType,
}
impl FunctionSignature{
    pub(crate) fn inputs(&self)->&[VariableType]{
        &self.inputs
    }
    pub(crate) fn output(&self)->&VariableType{
        &self.output
    }
    pub(crate) fn from_poly_sig(sig:PolyFnSig)->Option<Self>{
        let inputs = sig.inputs().no_bound_vars()?.iter().map(|v|VariableType::from_ty(*v)).collect();
        let output = VariableType::from_ty(sig.output().no_bound_vars()?);
        Some(Self{inputs,output})
    }
}
impl VariableType{
    fn from_ty(ty:Ty)->Self{
        match ty.kind() {
            TyKind::Int(IntTy::I8) => VariableType::I8,
            TyKind::Int(IntTy::I16) => VariableType::I16,
            TyKind::Int(IntTy::I32) => VariableType::I32,
            TyKind::Int(IntTy::I64) => VariableType::I64,
            TyKind::Int(IntTy::I128) => VariableType::I128,
            TyKind::Int(IntTy::Isize) => VariableType::ISize,
            TyKind::Uint(UintTy::U8) => VariableType::U8,
            TyKind::Uint(UintTy::U16) => VariableType::U16,
            TyKind::Uint(UintTy::U32) => VariableType::U32,
            TyKind::Uint(UintTy::U64) => VariableType::U64,
            TyKind::Uint(UintTy::U128) => VariableType::U128,
            TyKind::Uint(UintTy::Usize) => VariableType::USize,
            TyKind::Bool=>VariableType::Bool,
            _ => todo!("Unhandled type kind {:?}", ty.kind()),
        }
    }
    pub(crate) fn il_name(&self)->IString{
        match self{
            Self::I8=>"int8",
            Self::I16=>"int16",
            Self::I32=>"int32",
            Self::I64=>"int64",
            Self::I128=>"[System.Runtime]System.Int128",
            Self::ISize => "native int",
            Self::U8=>"uint8",
            Self::U16=>"uint16",
            Self::U32=>"uint32",
            Self::U64=>"uint64",
            Self::U128=>"[System.Runtime]System.UInt128",
            Self::USize =>"native uint",
            Self::Bool=>"bool",
        }.into()
    }
}
impl CodegenBackend for MyBackend {
    fn locale_resource(&self) -> &'static str {
        ""
    }
    fn codegen_crate<'a, 'tcx>(
        &self,
        tcx: TyCtxt<'tcx>,
        metadata: EncodedMetadata,
        _need_metadata_module: bool,
    ) -> Box<dyn Any> {
        let (_defid_set, cgus) = tcx.collect_and_partition_mono_items(());
        for cgu in cgus {
            let mut codegen = Assembly::new(&format!("{}", cgu.name()));
            println!("codegen {} has {} items.", cgu.name(), cgu.items().len());
            for (item, _data) in cgu.items() {
                codegen.add_item(*item, tcx);
            }
            println!("CLR IL:\n```\n{ir}\n```", ir = codegen.into_il_ir());
        }
        Box::new(CodegenResults {
            modules: vec![],
            allocator_module: None,
            metadata_module: None,
            metadata,
            crate_info: CrateInfo::new(tcx, "fake_target_cpu".to_string()),
        })
    }

    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn Any>,
        _sess: &Session,
        _outputs: &OutputFilenames,
    ) -> Result<(CodegenResults, FxIndexMap<WorkProductId, WorkProduct>), ErrorGuaranteed> {
        let codegen_results = ongoing_codegen
            .downcast::<CodegenResults>()
            .expect("in join_codegen: ongoing_codegen is not a CodegenResults");
        Ok((*codegen_results, FxIndexMap::default()))
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
        for &crate_type in sess.opts.crate_types.iter() {
            if crate_type != CrateType::Rlib {
                sess.fatal(format!("Crate type is {:?}", crate_type));
            }
            let output_name = out_filename(sess, crate_type, &outputs, crate_name);
            match output_name {
                OutFileName::Real(ref path) => {
                    let mut out_file = ::std::fs::File::create(path).unwrap();
                    write!(out_file, "This has been \"compiled\" successfully.").unwrap();
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
