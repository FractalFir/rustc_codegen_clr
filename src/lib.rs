#![feature(rustc_private)]
extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_index;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use rustc_codegen_ssa::{CrateInfo,traits::CodegenBackend, CodegenResults};
use rustc_data_structures::fx::FxIndexMap;
use rustc_metadata::EncodedMetadata;
use rustc_middle::{
    dep_graph::{WorkProduct, WorkProductId},
    ty::{FloatTy, IntTy, Ty, TyCtxt, TyKind, UintTy,PolyFnSig},
    mir::Mutability,
};
use rustc_session::{config::OutputFilenames, Session};
use rustc_span::ErrorGuaranteed;
use std::any::Any;

mod clr_method;
use clr_method::*;
mod assembly;
use assembly::*;
mod base_ir;
use base_ir::BaseIR;
pub type IString = Box<str>;

struct MyBackend;
#[derive(Clone, Debug)]
enum VariableType {
    Void,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    F32,
    F64,
    Bool,
    Ref(Box<Self>),
    RefMut(Box<Self>),
}
#[derive(Debug)]
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
    pub(crate) fn from_poly_sig(sig: PolyFnSig) -> Option<Self> {
        let inputs = sig
            .inputs()
            // `skip_binder` is `a riskiy thing` TODO: Foigure out to which kind of issues it may lead!
            .skip_binder()
            //.no_bound_vars()?
            .iter()
            .map(|v| VariableType::from_ty(*v))
            .collect();
        let output = VariableType::from_ty(sig.output()
            // `skip_binder` is `a riskiy thing` TODO: Foigure out to which kind of issues it may lead!
            .skip_binder()
            //.no_bound_vars()?
        );
        Some(Self { inputs, output })
    }
}
impl VariableType {
    fn from_ty(ty: Ty) -> Self {
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
            TyKind::Float(FloatTy::F32) => VariableType::F32,
            TyKind::Float(FloatTy::F64) => VariableType::F64,
            TyKind::Bool => VariableType::Bool,
            TyKind::Char => todo!("Can't handle chars yet!"),
            TyKind::Foreign(ftype) => todo!("Can't handle foreign types yet!"),
            TyKind::Str => todo!("Can't handle string slices yet!"),
            TyKind::Array(element_type,length) => todo!("Can't handle arrays yet!"),
            TyKind::Slice(element_type) => todo!("Can't handle slices yet!"),
            TyKind::Adt(adt_def,subst) => todo!("Can't ADTs(structs,enums,unions) yet!"),
            TyKind::RawPtr(target_type) => todo!("Can't handle pointers yet!"),
            TyKind::FnPtr(sig) => todo!("Can't handle function pointers yet!"),
            TyKind::Ref(region,ref_type,mutability)=> {
                // There is no such concept as lifetimes in CLR
                let _ = region;
                match mutability{
                    Mutability::Mut =>  Self::RefMut(Box::new(Self::from_ty(*ref_type))),
                    Mutability::Not => Self::Ref(Box::new(Self::from_ty(*ref_type)))
                }
               
            },
            TyKind::Bound(debrujin_index, bound_ty)=>{
                todo!("Bound, debrujin_index:{debrujin_index:?}, bound_ty:{bound_ty:?}");
            },
            TyKind::Tuple(inner_types) => {
                if inner_types.len() == 0{
                    return Self::Void;
                }
                todo!("Can't handle tuples yet!");
            }
            _ => todo!("Unhandled type kind {:?}", ty.kind()),
        }
    }
    pub(crate) fn il_name(&self) -> IString {
        match self {
            Self::Void => "void".into(),
            Self::I8 => "int8".into(),
            Self::I16 => "int16".into(),
            Self::I32 => "int32".into(),
            Self::I64 => "int64".into(),
            Self::I128 => "[System.Runtime]System.Int128".into(),
            Self::ISize => "native int".into(),
            Self::U8 => "uint8".into(),
            Self::U16 => "uint16".into(),
            Self::U32 => "uint32".into(),
            Self::U64 => "uint64".into(),
            Self::U128 => "[System.Runtime]System.UInt128".into(),
            Self::USize => "native uint".into(),
            Self::F32 => "float32".into(),
            Self::F64 => "float64".into(),
            Self::Bool => "bool".into(),
            Self::Ref(inner) => format!("{inner}&",inner = inner.il_name()), 
            Self::RefMut(inner) => format!("{inner}&",inner = inner.il_name()), 
        }
        .into()
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
            let output_name = out_filename(sess, crate_type, outputs, crate_name);
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
