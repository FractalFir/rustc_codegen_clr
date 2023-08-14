#![feature(rustc_private)]
extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_index;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use rustc_codegen_ssa::{CompiledModule,ModuleKind,CrateInfo,traits::CodegenBackend, CodegenResults};
use rustc_data_structures::fx::FxIndexMap;
use rustc_metadata::EncodedMetadata;
use rustc_middle::{
    dep_graph::{WorkProduct, WorkProductId},
    ty::{FloatTy, IntTy, Ty, TyCtxt, TyKind, UintTy,PolyFnSig},
    mir::Mutability,
};

use rustc_session::{config::{OutputFilenames,OutputType}, Session};
use rustc_span::ErrorGuaranteed;
use std::any::Any;
use serde::{Serialize,Deserialize};

mod clr_method;
use clr_method::*;
mod assembly;
use assembly::*;
mod base_ir;
use base_ir::BaseIR;
pub type IString = Box<str>;

struct MyBackend;
#[derive(Serialize,Deserialize,Clone, Debug)]
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
#[derive(Clone,Debug,Serialize,Deserialize)]
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
        let mut codegen = Assembly::new(&cgus.iter().next().unwrap().name().to_string());
        for cgu in cgus {
            //println!("codegen {} has {} items.", cgu.name(), cgu.items().len());
            for (item, _data) in cgu.items() {
                codegen.add_item(*item, tcx);
            }
            
        }
        //println!("CLR IL:\n```\n{ir}\n```", ir = codegen.into_il_ir());
      
        Box::new((codegen,metadata,CrateInfo::new(tcx, "clr".to_string())))
    }

    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn Any>,
        _sess: &Session,
        outputs: &OutputFilenames,
    ) -> Result<(CodegenResults, FxIndexMap<WorkProductId, WorkProduct>), ErrorGuaranteed> {
        use std::io::Write;
        let (asm,metadata,crate_info) = *ongoing_codegen
            .downcast::<(Assembly,EncodedMetadata,CrateInfo)>()
            .expect("in join_codegen: ongoing_codegen is not an Assembly");
        
        let serialized_asm_path = outputs.temp_path(OutputType::Object,Some(asm.name()));
        //std::fs::create_dir_all(&serialized_asm_path).expect("Could not create the directory temporary files are supposed to be in.");
        let mut asm_out = std::fs::File::create(&serialized_asm_path).expect("Could not create the temporary files necessary for building the assembly!");
        asm_out.write_all(&postcard::to_stdvec(&asm).expect("Could not serialize the tmp assembly file!")).expect("Could not save the tmp assembly file!");
        let mut modules =  vec![CompiledModule{name:asm.name().to_owned(),kind:ModuleKind::Regular,object:Some(serialized_asm_path.into()),bytecode:None,dwarf_object:None }];
        let codegen_results = CodegenResults {
            modules,
            allocator_module: None,
            metadata_module: None,
            metadata,
            crate_info
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
        let mut final_assembly = Assembly::new(&codegen_results.crate_info.local_crate_name.to_string());
        for module in codegen_results.modules{
            use std::io::Read;
            
            let asm_path = module.object.expect("ERROR: object file path is missing!");
            let mut asm_file = std::fs::File::open(asm_path).expect("ERROR:Could not open the assembly file!");
            let mut asm_bytes = Vec::with_capacity(0x100);
            asm_file.read_to_end(&mut asm_bytes).expect("ERROR:Could not load the assembly file!");
            let assembly = postcard::from_bytes(&asm_bytes).expect("ERROR:Could not decode the assembly file!");
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
                    write!(out_file, "{}",asm_il).unwrap();
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
