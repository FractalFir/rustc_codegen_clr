#![feature(rustc_private)]
extern crate rustc_codegen_ssa;
extern crate rustc_middle;
extern crate rustc_metadata;
extern crate rustc_span;
extern crate rustc_session;
extern crate rustc_data_structures;
extern crate rustc_driver;

use rustc_middle::ty::PolyFnSig;
use rustc_middle::mir::mono::MonoItem;
use rustc_codegen_ssa::CrateInfo;
use std::any::Any;
use rustc_codegen_ssa::{traits::CodegenBackend,CodegenResults};
use rustc_middle::{ty::TyCtxt,dep_graph::WorkProduct,dep_graph::WorkProductId};
use rustc_middle::ty::ParamEnv;
use rustc_metadata::EncodedMetadata;
use rustc_span::ErrorGuaranteed;
use std::hash::BuildHasherDefault;
use rustc_session::{config::OutputFilenames,Session};
use rustc_data_structures::fx::FxIndexMap;

struct MyBackend;
struct CustomCodegen{}
impl CustomCodegen{
    fn add_fn<'tcx>(&mut self,fn_sig:PolyFnSig<'tcx>){
        let inputs = fn_sig.inputs();
        println!("fn_sig:{fn_sig:?} inputs:{inputs:?}");
    }
    fn add_item<'tcx>(&mut self,item:MonoItem<'tcx>,tcx: TyCtxt<'tcx>){
        println!("adding item:{}",item.symbol_name(tcx));
        // TODO: figure out: What should it be???
        let param_env = ParamEnv::empty();
        match item{
            MonoItem::Fn(instance)=>{
                let ty = instance.ty(tcx,param_env);
                let fn_sig = ty.fn_sig(tcx);
                self.add_fn(fn_sig);
            },
            _=>todo!("Unsupported item:\"{item:?}\"!"),
        }
    }
}
impl CodegenBackend for MyBackend {
   fn locale_resource(&self) -> &'static str { "" }

    fn codegen_crate<'a, 'tcx>(
        &self,
        tcx: TyCtxt<'tcx>,
        metadata: EncodedMetadata,
        _need_metadata_module: bool,
    ) -> Box<dyn Any> {
        let (defid_set,cgus) = tcx.collect_and_partition_mono_items(()); 
        for cgu in cgus{
            let mut codegen = CustomCodegen{};
            println!("codegen {} has {} items.",cgu.name(),cgu.items().len());
            for (item,data) in cgu.items(){
                codegen.add_item(*item,tcx);
            }
            
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
        use rustc_session::{config::{CrateType, OutFileName}, output::out_filename};
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
