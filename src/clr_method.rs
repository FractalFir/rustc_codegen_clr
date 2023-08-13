use crate::BaseIR;
use crate::IString;
use crate::FunctionSignature;
use crate::VariableType;
use rustc_middle::{
    mir::{
        interpret::ConstValue, AggregateKind, BinOp, ConstantKind, Operand, Rvalue, Statement,
        StatementKind, Terminator, TerminatorKind,
    },
    ty::{IntTy, TyKind},
};
#[derive(Debug)]
pub(crate) struct CLRMethod {
    ops: Vec<BaseIR>,
    locals: Vec<Option<VariableType>>,
    op_types:Vec<Option<VariableType>>,
    sig: FunctionSignature,
    name:IString,
}
enum LocalPlacement {
    Arg(u32),
    Var(u32),
}
impl CLRMethod {
    fn name(&self)->&str{&self.name}
    fn crosses_bb_border(&self,start:usize,end:usize)->bool{
        false
    }
    fn backtrace_op_type(&self,op_pos:usize)->Option<&VariableType>{
        let mut op_pos:isize = (op_pos as isize - 1);
        let mut depth = -1;
        while op_pos >= 0{
            depth += self.ops[op_pos as usize].stack_change();
            if(depth == 0){
                return self.op_types[op_pos as usize].as_ref();
            }
            op_pos -= 1;
        }
        None
    }
    // This function returns the type of a local variable if it is known
    pub(crate) fn local_type(&self,local:u32)->Option<&VariableType>{
        self.locals.get(local as usize)?.as_ref()
    }
    pub(crate) fn get_arg_type(&self,arg:u32)->&VariableType{
        &self.sig.inputs()[arg as usize]
    }
    fn set_trivial_types(&mut self){
        for index in 0..(self.op_types.len()){
            if let None = self.op_types[index]{
                self.op_types[index] = self.ops[index].get_trivial_type(self);
            }
        }
    }
    fn set_local_types(&mut self){
        for index in 0..(self.op_types.len()){
            if let BaseIR::STLoc(index) = self.ops[index as usize]{
                if let Some(var_type) = &self.op_types[index as usize]{
                    while self.locals.len() < index as usize + 1{self.locals.push(None)};
                    self.locals[index as usize] = Some(var_type.clone());
                }
            }
        }
    }
    fn backtrace_types(&mut self){
        for index in 0..(self.op_types.len()){
            if let None = self.op_types[index]{
                self.op_types[index] = self.backtrace_op_type(index).cloned();
            }
        }
    }
    pub(crate) fn typecheck(&mut self){
        while self.op_types.len() < self.ops.len(){
            self.op_types.push(None);      
        }
        for _ in 0..0x100{
            self.set_trivial_types();
            self.backtrace_types();
            self.set_local_types();
            if self.op_types.iter().all(|t| t.is_some()){break;}
        }
        println!("op_types:{:?}",self.op_types);
    }
    pub(crate) fn locals_init(&self)->IString{
        if self.locals.is_empty(){
            return "".into();     
        }
        let mut locals = String::new();
        let mut locals_iter = self.locals.iter().enumerate();
        match locals_iter.next(){
            Some((index,first))=>{
                locals.push_str(&format!("\n\t\t[{index}] {loc_type}",loc_type = first.as_ref().expect("UNRESOLVED LOCAL DURING IL EMISSION!").il_name()))
            }
            None=>(),
        }
        for (index,local) in locals_iter{
            locals.push_str(&format!(",\n\t\t[{index}] {loc_type}",loc_type = local.as_ref().expect("UNRESOLVED LOCAL DURING IL EMISSION!").il_name()))
        }
        format!("\t.locals init({locals}\n\t)").into()
    }
    pub(crate) fn into_il_ir(&self)->String{
        let output = self.sig.output().il_name();
        let mut arg_list = String::new();
        let mut arg_iter = self.sig.inputs.iter();
        match arg_iter.next(){
            Some(start)=>arg_list.push_str(&start.il_name()),
            None=>(),
        }
        for arg in arg_iter{
            arg_list.push(',');
            arg_list.push_str(&arg.il_name());
        }
        let mut ops_ir = String::new();
        for op in &self.ops{
            ops_ir.push_str(&op.clr_ir());
        }
        format!(".method public static {output} {name}({arg_list}){{\n{locals_init}\n{ops_ir}}}\n",name = self.name(),locals_init = self.locals_init())
    }
    fn argc(&self) -> u32 {
        self.sig.inputs().len() as u32
    }
    fn remove_sl(&mut self) -> usize {
        let mut opt_ops: Vec<BaseIR> = Vec::with_capacity(self.ops.len());
        let mut ops_peek = self.ops.iter().enumerate().peekable();
        while let Some((index,op)) = ops_peek.next() {
            match op {
                BaseIR::STLoc(local_id) => {
                    if let Some((next_index,BaseIR::LDLoc(other_id))) = ops_peek.peek() {
                        //Ops store and the load the same value, being effectively a NOP.
                        if local_id == other_id && !self.crosses_bb_border(index,*next_index) {
                            ops_peek.next();
                            continue;
                        }
                    }
                }
                _ => (),
            }
            opt_ops.push(op.clone());
        }
        self.ops = opt_ops;
        self.ops.len()
    }
    pub(crate) fn opt(&mut self) {
        const MAX_OPT_PASS: usize = 8;
        for _ in 0..MAX_OPT_PASS {
            let prev_length = self.ops.len();
            if !(self.remove_sl() < prev_length) {
                continue;
            }
        }
    }
    fn has_return(&self) -> bool {
        true
    }
    pub(crate) fn new(sig: FunctionSignature,name:&str) -> Self {
        Self {
            locals: Vec::new(),
            op_types: Vec::with_capacity(0x100),
            sig,
            name:name.into(),
            ops: Vec::with_capacity(0x100),
        }
    }
    fn var_live(&mut self, _local: u32) {
        //TODO: use variable lifetimes!
    }
    fn var_dead(&mut self, _local: u32) {
        //TODO: use variable lifetimes!
    }
    fn local_id_placement(&self, local: u32) -> LocalPlacement {
        // I assume local 0 is supposed to be the return value. TODO: check if this is always the case.
        if self.has_return() {
            if local == 0 {
                LocalPlacement::Var(0)
            } else if local - 1 < self.argc() {
                LocalPlacement::Arg(local - 1)
            } else {
                LocalPlacement::Var(local - self.argc())
            }
        } else {
            panic!("Can't handle void functions yet. Diagnose MIR to determine what happens to the return var(0)!");
        }
    }
    fn load(&mut self, local: u32) {
        self.ops.push(match self.local_id_placement(local) {
            LocalPlacement::Arg(arg_id) => BaseIR::LDArg(arg_id),
            LocalPlacement::Var(var_id) => BaseIR::LDLoc(var_id),
        })
    }
    fn store(&mut self, local: u32) {
        self.ops.push(match self.local_id_placement(local) {
            LocalPlacement::Arg(arg_id) => BaseIR::STArg(arg_id),
            LocalPlacement::Var(var_id) => BaseIR::STLoc(var_id),
        })
    }
    fn process_constant(&mut self, constant: ConstantKind) {
        self.ops.push(match constant {
            ConstantKind::Val(value, r#type) => match value {
                ConstValue::Scalar(scalar) => match r#type.kind() {
                    TyKind::Int(IntTy::I32) => BaseIR::LDConstI32(
                        scalar
                            .to_i32()
                            .expect("Type is i32, but odes not fit in i32."),
                    ),
                    _ => todo!("Unhandled type kind {:?}", r#type.kind()),
                },
                _ => todo!("Unhanled constant value {value:?}"),
            },
            _ => todo!("Unhanled constant {constant:?}"),
        });
    }
    // Makes so the top of the stack is the value of RValue
    fn process_operand(&mut self, operand: &Operand) {
        match operand {
            Operand::Copy(place) => self.load(place.local.into()),
            //TODO:Do moves need to be treated any diffrently forom copies in the context of CLR?
            Operand::Move(place) => self.load(place.local.into()),
            Operand::Constant(const_val) => {
                self.process_constant(const_val.literal);
            }
            _ => todo!("Unhanled operand {operand:?}"),
        }
    }
    // Makes so the top of the stack is the value of RValue
    fn process_rvalue(&mut self, rvalue: &Rvalue) {
        match rvalue {
            Rvalue::Use(operand) => self.process_operand(operand),
            Rvalue::BinaryOp(binop, operands) => {
                let (a, b): (_, _) = (&operands.0, &operands.1);
                self.process_operand(a);
                self.process_operand(b);
                self.ops.push(match binop {
                    BinOp::Add => BaseIR::Add,
                    BinOp::Mul => BaseIR::Mul,
                    BinOp::Shl => BaseIR::Shl,
                    _ => todo!("Unknown binop:{binop:?}"),
                });
            }
            Rvalue::Aggregate(kind, operands) => {
                match kind.as_ref() {
                    AggregateKind::Adt(def_id, variant_idx, subst_ref, utai, fidx) => {
                        //let (def_id,variant_idx,subst_ref,utai,fidx) = *adt;
                        panic!("def_id:{def_id:?},variant_idx:{variant_idx:?},subst_ref:{subst_ref:?},utai:{utai:?},fidx:{fidx:?}");
                    }
                    _ => todo!(
                        "Can't yet handle the aggregate of kind {kind:?} and operands:{operands:?}"
                    ),
                }
            }
            _ => todo!("Can't yet handle a rvalue of type {rvalue:?}"),
        }
    }
    pub(crate) fn add_statement(&mut self, statement: &Statement) {
        println!("statement:{statement:?}");
        match &statement.kind {
            StatementKind::Assign(asign_box) => {
                let (place, rvalue) = (asign_box.0, &asign_box.1);
                self.process_rvalue(rvalue);
                self.store(place.local.into());
                //panic!("place:{place:?},rvalue:{rvalue:?}");
            }
            StatementKind::StorageLive(local) => {
                self.var_live((*local).into());
            }
            StatementKind::StorageDead(local) => {
                self.var_dead((*local).into());
            }
            _ => todo!("Unhanded statement:{:?}", statement.kind),
        }
    }
    pub(crate) fn add_terminator(&mut self, terminator: &Terminator) {
        match terminator.kind {
            TerminatorKind::Return => {
                if self.has_return() {
                    self.load(0);
                    self.ops.push(BaseIR::Return);
                } else {
                    todo!("Can't yet return from a void method!");
                }
            }
            _ => todo!("Unknown terminator type {terminator:?}"),
        }
    }
}
