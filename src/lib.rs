#![feature(rustc_private)]
extern crate rustc_codegen_ssa;
extern crate rustc_middle;
extern crate rustc_metadata;
extern crate rustc_span;
extern crate rustc_session;
extern crate rustc_data_structures;
extern crate rustc_driver;

use rustc_codegen_ssa::CrateInfo;
use std::any::Any;
use rustc_codegen_ssa::{traits::CodegenBackend,CodegenResults};
use rustc_middle::{ty::{TyCtxt,Ty,ParamEnv,Instance,PolyFnSig},dep_graph::{WorkProductId,WorkProduct},mir::{Statement,StatementKind,Rvalue,mono::MonoItem,Operand}};
use rustc_metadata::EncodedMetadata;
use rustc_span::ErrorGuaranteed;
use std::hash::BuildHasherDefault;
use rustc_session::{config::OutputFilenames,Session};
use rustc_data_structures::fx::FxIndexMap;
#[derive(Debug)]
enum CLROp{
    STArg(u32),
    LDArg(u32),
    STLoc(u32),
    LDLoc(u32),
}
struct MyBackend;
struct CustomCodegen{}
#[derive(Debug)]
struct CLRMethod{
    ops:Vec<CLROp>,
    argc:u32,
}
impl CLRMethod{
    fn argc(&self)->u32{self.argc}
    fn new(argc:u32)->Self{
        Self{argc,ops:Vec::with_capacity(0xF)}
    }
    fn load(&mut self,var:u32){
        if var < self.argc(){ 
            self.ops.push(CLROp::LDArg(var));
        }
        else{
            println!("argc:{}, var:{var}",self.argc);
            self.ops.push(CLROp::LDLoc(var - self.argc()));
        }
    }
    // Makes so the top of the stack is the value of RValue
    fn process_operand(&mut self, operand:&Operand){
        match operand{
            Operand::Copy(place) => self.load(place.local.into()),
            _=>todo!("Unhanled operand {operand:?}"),
        }
    }
    // Makes so the top of the stack is the value of RValue
    fn process_rvalue(&mut self,rvalue:&Rvalue){
        match rvalue{
            Rvalue::Use(operand)=>self.process_operand(operand),
            _=>todo!("Can't yet handle a rvalue of type {rvalue:?}"),
        }
    }
    fn store(&mut self,var:u32){
        if var < self.argc(){
            self.ops.push(CLROp::STArg(var));
        }
        else{
            self.ops.push(CLROp::STLoc(var - self.argc()));
        }
    }
    fn add_statement(&mut self,statement:&Statement){
        match &statement.kind{
            StatementKind::Assign(asign_box)=>{
                let (place,rvalue) = (asign_box.0,&asign_box.1);
                self.process_rvalue(rvalue);
                self.store(place.local.into());
                //panic!("place:{place:?},rvalue:{rvalue:?}");
            }
            _=>todo!("Unhanded statement:{:?}",statement.kind),       
        }
    }
}
impl CustomCodegen{
    fn add_fn<'tcx>(&mut self,instance:Instance<'tcx>,tcx: TyCtxt<'tcx>){
        // TODO: figure out: What should it be???
        let param_env = ParamEnv::empty();
        
        let def_id = instance.def_id(); 
        let mir = tcx.optimized_mir(def_id);
        let blocks = &(*mir.basic_blocks);
        let sig = instance.ty(tcx,param_env).fn_sig(tcx);
        let mut clr_method = CLRMethod::new(sig.inputs().no_bound_vars().expect("Encountered a not fully morphed function signature. This is not supported yet.").len() as u32);
        for block_data in blocks{
            for statement in &block_data.statements{
                clr_method.add_statement(statement);
            }
            
        }
        println!("clr_method:{clr_method:?}");
        println!("instance:{instance:?}\n");
    }
    fn add_item<'tcx>(&mut self,item:MonoItem<'tcx>,tcx: TyCtxt<'tcx>){
        println!("adding item:{}",item.symbol_name(tcx));
        
        match item{
            MonoItem::Fn(instance)=>self.add_fn(instance,tcx),
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
