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
use rustc_middle::{ty::{TyCtxt,Ty,ParamEnv,Instance,PolyFnSig,TyKind,IntTy},dep_graph::{WorkProductId,WorkProduct},mir::{Statement,StatementKind,Terminator,TerminatorKind,Rvalue,BinOp,Operand,ConstantKind,AggregateKind,interpret::ConstValue,mono::MonoItem}};
use rustc_metadata::EncodedMetadata;
use rustc_span::ErrorGuaranteed;
use std::hash::BuildHasherDefault;
use rustc_session::{config::OutputFilenames,Session};
use rustc_data_structures::fx::FxIndexMap;
#[derive(Debug,Clone)]
enum BaseIR{
    LDConstI32(i32),
    STArg(u32),
    LDArg(u32),
    STLoc(u32),
    LDLoc(u32),
    Add,
    Mul,
    Shl,
    Return,
}
struct MyBackend;
struct CustomCodegen{}
#[derive(Debug)]
struct CLRMethod{
    ops:Vec<BaseIR>,
    argc:u32,
}
enum LocalPlacement{
    Arg(u32),
    Var(u32),
}
impl CLRMethod{
    fn argc(&self)->u32{self.argc}
    fn remove_sl(&mut self)->usize{
        let mut opt_ops:Vec<BaseIR> = Vec::with_capacity(self.ops.len());
        let mut ops_peek = self.ops.iter().peekable();
        while let Some(op) = ops_peek.next(){
            match op{
                BaseIR::STLoc(local_id)=>{
                    if let Some(BaseIR::LDLoc(other_id)) = ops_peek.peek(){
                        //Ops store and the load the same value, being effectively a NOP.
                        if local_id == other_id{
                            ops_peek.next();
                            continue;
                        }
                    }  
                }
                _=>(),
            }
            opt_ops.push(op.clone());
        }
        self.ops = opt_ops;
        self.ops.len()
    }
    fn opt(&mut self){
        const MAX_OPT_PASS:usize = 8;
        for _ in 0..MAX_OPT_PASS{
            let prev_length = self.ops.len();
            if !(self.remove_sl() < prev_length){continue;}
        }
    }
    fn has_return(&self)->bool{true}
    fn new(argc:u32)->Self{
        Self{argc,ops:Vec::with_capacity(0xF)}
    }
    fn var_live(&mut self,local:u32){
        //TODO: use variable lifetimes!
    }
    fn var_dead(&mut self,local:u32){
        //TODO: use variable lifetimes!
    }
    fn local_id_placement(&self,local:u32)->LocalPlacement{
        // I assume local 0 is supposed to be the return value. TODO: check if this is always the case.
        if self.has_return(){
             if local == 0{
                LocalPlacement::Var(0)
            } 
            else if local - 1 < self.argc(){
                LocalPlacement::Arg(local - 1)
            }
            else{
                LocalPlacement::Var(local - self.argc())
            }
        }
        else{
            panic!("Can't handle void functions yet. Diagnose MIR to determine what happens to the return var(0)!");
        }
       
    }
    fn load(&mut self,local:u32){
        self.ops.push(match self.local_id_placement(local){
            LocalPlacement::Arg(arg_id)=>BaseIR::LDArg(arg_id),
            LocalPlacement::Var(var_id)=>BaseIR::LDLoc(var_id),
        })
    }
    fn store(&mut self,local:u32){
        self.ops.push(match self.local_id_placement(local){
            LocalPlacement::Arg(arg_id)=>BaseIR::STArg(arg_id),
            LocalPlacement::Var(var_id)=>BaseIR::STLoc(var_id),
        })
    }
    fn process_constant(&mut self, constant:ConstantKind){
        self.ops.push(match constant{
            ConstantKind::Val(value,r#type)=> match value{
                ConstValue::Scalar(scalar)=>{
                    match r#type.kind(){
                        TyKind::Int(IntTy::I32)=>BaseIR::LDConstI32(scalar.to_i32().expect("Type is i32, but odes not fit in i32.")),
                        _=>todo!("Unhandled type kind {:?}",r#type.kind()),
                    }
                }
                _=>todo!("Unhanled constant value {value:?}"),
            }
            _=>todo!("Unhanled constant {constant:?}"),
        });
    }
    // Makes so the top of the stack is the value of RValue
    fn process_operand(&mut self, operand:&Operand){
        match operand{
            Operand::Copy(place) => self.load(place.local.into()),
            //TODO:Do moves need to be treated any diffrently forom copies in the context of CLR?
            Operand::Move(place) => self.load(place.local.into()),
            Operand::Constant(const_val)=>{
                self.process_constant(const_val.literal);
            },
            _=>todo!("Unhanled operand {operand:?}"),
        }
    }
    // Makes so the top of the stack is the value of RValue
    fn process_rvalue(&mut self,rvalue:&Rvalue){
        match rvalue{
            Rvalue::Use(operand)=>self.process_operand(operand),
            Rvalue::BinaryOp(binop,operands)=>{
                let (a,b):(_,_) = (&operands.0,&operands.1);
                self.process_operand(a);
                self.process_operand(b);
                self.ops.push(match binop{
                    BinOp::Add=>BaseIR::Add,
                    BinOp::Mul=>BaseIR::Mul,
                    BinOp::Shl=>BaseIR::Shl,
                    _=>todo!("Unknown binop:{binop:?}"),
                });
            }
            Rvalue::Aggregate(kind,operands)=>{
                match kind.as_ref(){
                    AggregateKind::Adt(def_id,variant_idx,subst_ref,utai,fidx)=>{
                        //let (def_id,variant_idx,subst_ref,utai,fidx) = *adt;
                        panic!("def_id:{def_id:?},variant_idx:{variant_idx:?},subst_ref:{subst_ref:?},utai:{utai:?},fidx:{fidx:?},\noperands:{operands}");
                    }
                    _=> todo!("Can't yet handle the aggregate of kind {kind:?} and operands:{operands:?}"),
                }
            }
            _=>todo!("Can't yet handle a rvalue of type {rvalue:?}"),
        }
    }
    fn add_statement(&mut self,statement:&Statement){
        println!("statement:{statement:?}");
        match &statement.kind{
            StatementKind::Assign(asign_box)=>{
                let (place,rvalue) = (asign_box.0,&asign_box.1);
                self.process_rvalue(rvalue);
                self.store(place.local.into());
                //panic!("place:{place:?},rvalue:{rvalue:?}");
            },
            StatementKind::StorageLive(local)=>{
                self.var_live((*local).into());
            }
            StatementKind::StorageDead(local)=>{
                self.var_dead((*local).into());
            }
            _=>todo!("Unhanded statement:{:?}",statement.kind),       
        }
    }
    fn add_terminator(&mut self,terminator:&Terminator){
        match terminator.kind{
            TerminatorKind::Return=>{
                if self.has_return(){
                    self.load(0);
                    self.ops.push(BaseIR::Return);
                }
                else{
                    todo!("Can't yet return from a void method!");
                } 
            }
            _=>todo!("Unknown terminator type {terminator:?}"),
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
            match &block_data.terminator{
                Some(term)=>clr_method.add_terminator(term),
                None=>(),
            }
        }
        clr_method.opt();
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
