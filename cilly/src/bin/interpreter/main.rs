#![feature(iterator_try_collect)]
use std::collections::HashMap;

use cilly::{
    asm::Assembly, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, method::Method,
    FnSig, IString, Type,
};
#[derive(Debug)]
enum Exception {
    MethodNotFound(CallSite),
    LocalOutOfRange { loc: usize, lcount: usize },
    AllocOffsetOutOfRange,
}
type AllocID = u32;
struct InterpreterState<'asm> {
    asm: &'asm Assembly,
    call_stack: Vec<(&'asm CallSite, usize, usize)>,
    locals: Vec<Box<[Value]>>,
    mem:HashMap<AllocID,Box<[u8]>>,
    last_alloc:AllocID,
}
#[derive(Clone, Debug)]
enum Value {
    Undef,
    StringArray(Box<[IString]>),
    String(IString),
    USize(usize),
    I32(i32),
    U32(u32),
    Ptr(AllocID,u32)
}

impl Value {
    fn as_usize(&self) -> Option<&usize> {
        if let Self::USize(v) = self {
            Some(v)
        } else {
            None
        }
    }
    fn as_ptr(&self) -> Option<(AllocID,u32)> {
        if let Self::Ptr(id,offset) = self {
            Some((*id,*offset))
        } else {
            None
        }
    }
}
fn eval_node<'asm>(
    node: &'asm CILNode,
    state: &mut InterpreterState<'asm>,
) -> Result<Value, Exception> {
    assert_eq!(state.locals.len(), state.call_stack.len());
    println!("Evaluating node {node:?}");
    match node {
        CILNode::SizeOf(tpe) =>match tpe.as_ref(){
            Type::USize | Type::ISize =>  Ok(Value::I32(8)),
            _=>todo!("Can't yet calc sizeof {tpe:?}"),
        }
        CILNode::LdcU32(val) => Ok(Value::U32(*val)),
        
        CILNode::LdcI32(val) => Ok(Value::I32(*val)),
        CILNode::Call { args, site } => {
            let mut args: Box<[_]> = args.iter().map(|arg| eval_node(arg, state)).try_collect()?;
            assert_eq!(state.locals.len(), state.call_stack.len());
            match state.run(site, &mut args) {
                Ok(val) => Ok(val),
                Err(Exception::MethodNotFound(_)) => state.try_call_extern(site, &mut args),
                Err(err) => Err(err),
            }
        }
        CILNode::Add(a,b) => {
            let a = eval_node(a, state)?;
            let b = eval_node(b, state)?;
            match (&a,&b) {
                (Value::I32(a),Value::I32(b))=>Ok(Value::I32(a.wrapping_add(*b))),
                (Value::Ptr(alloc,offset),Value::USize(b))=>Ok(Value::Ptr(*alloc,offset.checked_add(*b as u32).ok_or(Exception::AllocOffsetOutOfRange)?)),
               // Value::USize(val) => Ok(Value::I32(val as i32)),
                _ => todo!("Can't yet add {a:?} and {b:?}"),
            }
        }
        CILNode::LDElelemRef { arr, idx } => {
            let a = eval_node(arr, state)?;
            let b = eval_node(idx, state)?;
            match (&a,&b) {
                (Value::StringArray(arr),Value::I32(idx))=>Ok(Value::String(arr[*idx as usize].clone())),
               // Value::USize(val) => Ok(Value::I32(val as i32)),
                _ => todo!("Can't yet index into {a:?} wtih {b:?}"),
            }
        }
        CILNode::Mul(a,b) => {
            let a = eval_node(a, state)?;
            let b = eval_node(b, state)?;
            match (&a,&b) {
                (Value::I32(a),Value::I32(b))=>Ok(Value::I32(a.wrapping_mul(*b))),
               // Value::USize(val) => Ok(Value::I32(val as i32)),
                _ => todo!("Can't yet mul {a:?} and {b:?}"),
            }
        }
        CILNode::ConvI32(value) => {
            let value = eval_node(value, state)?;
            match value {
                Value::USize(val) => Ok(Value::I32(val as i32)),
                _ => todo!("Can't convert a value of type {value:?} to i32"),
            }
        }
        CILNode::ZeroExtendToUSize(value) => {
            let value = eval_node(value, state)?;
            match value {
                Value::I32(val) => Ok(Value::USize(val as u32 as usize)),
                Value::U32(val) => Ok(Value::USize(val as usize)),
                _ => todo!("Can't convert a value of type {value:?} to usize"),
            }
        }
        CILNode::LDLen { arr } => {
            let arr = eval_node(arr, state)?;
            match arr {
                Value::StringArray(arr) => Ok(Value::USize(arr.len())),
                _ => todo!("Can't get the length of a {arr:?}"),
            }
        }
        CILNode::LDLoc(loc) => state
            .locals
            .last()
            .unwrap()
            .get(*loc as usize)
            .cloned()
            .ok_or(Exception::LocalOutOfRange {
                loc: *loc as usize,
                lcount: state.locals.len(),
            }),
        _ => todo!("Can't handle node:{node:?}"),
    }
}
impl<'asm> InterpreterState<'asm> {
    pub fn alloc(&mut self,size:usize)->AllocID{
        let new_alloc = self.last_alloc;
        self.last_alloc += 1;
        self.mem.insert(new_alloc,(0..size).map(|_|0_u8).collect());
        new_alloc
    }
    pub fn try_call_extern(
        &mut self,
        call: &'asm CallSite,
        args: &mut Box<[Value]>,
    ) -> Result<Value, Exception> {
        assert_eq!(self.locals.len(), self.call_stack.len());
        let res = match (call.class(), call.name(), call.signature()) {
            (Some(_), "GetCommandLineArgs", _) => Ok(Value::StringArray(
                std::env::args().map(|arg| arg.into()).collect(),
            )),
            (Some(_), "AlignedAlloc", _) => Ok(Value::Ptr(
                self.alloc(*args[0].as_usize().unwrap()),0
            )),
            (Some(class), name, sig) => todo!("Can't yet call extern {call:?}"),
            _ => Err(Exception::MethodNotFound(call.clone())),
        };
        println!(
            "try_call_extern call:{call:?} res:{res:?} cs:{:?}",
            self.call_stack
        );
        res
    }
    pub fn run_cctor(&mut self) -> Result<Value, Exception> {
        match self.asm.cctor() {
            Some(_) => self.run(
                Box::<CallSite>::leak(Box::new(CallSite::builtin(
                    ".cctor".into(),
                    FnSig::new(&[], Type::Void),
                    true,
                ))),
                &mut vec![].into(),
            ),
            None => Ok(Value::Undef),
        }
    }
    pub fn method(&self, site: &'asm CallSite) -> Result<&'asm Method, Exception> {
        self.asm
            .functions()
            .get(site)
            .ok_or(Exception::MethodNotFound(site.clone()))
    }
    pub fn run(
        &mut self,
        call: &'asm CallSite,
        args: &mut Box<[Value]>,
    ) -> Result<Value, Exception> {
        assert_eq!(self.locals.len(), self.call_stack.len());

        self.locals
            .push(vec![Value::Undef; self.method(call)?.locals().len()].into());
        self.call_stack.push((call, 0, 0));
        loop {
            let (method, block, tree) = if let Some(method) = self.call_stack.last_mut() {
                method
            } else {
                break;
            };
            let method = self
                .asm
                .functions()
                .get(method)
                .ok_or::<Exception>(Exception::MethodNotFound(method.clone()))?;
            let block_ref = &method.blocks()[*block];
            let tree_ref = &block_ref.trees()[*tree];
            match tree_ref.root() {
                CILRoot::GoTo { target, sub_target } => {
                    assert_eq!(*sub_target, 0);
                    let (method, block, tree) = self.call_stack.last_mut().unwrap();
                    *block = *target as usize;
                    *tree = 0;
                    continue;
                }
                CILRoot::STLoc { local, tree } => {
                    //let tree = tree.clone();
                    assert_eq!(self.locals.len(), self.call_stack.len());
                    let val = eval_node(tree, self)?;
                    self.locals.last_mut().unwrap()[*local as usize] = val;
                }
                CILRoot::STIndISize(addr,val) => {
                    //let tree = tree.clone();
                    assert_eq!(self.locals.len(), self.call_stack.len());
                    let addr = eval_node(addr, self)?;
                    let val = eval_node(val, self)?;
                    self.get_memory_at_mut(addr)[..8].copy_from_slice(&val.as_usize().unwrap().to_le_bytes()[..])
                }
                _ => todo!("unhandled root {root:?}", root = tree_ref.root()),
            }
            let (_, _, tree) = self.call_stack.last_mut().unwrap();
            *tree += 1;
        }
        todo!()
    }
    fn new(asm: &'asm Assembly) -> Self {
        Self {
            asm,
            call_stack: vec![],
            locals: vec![],
            mem:HashMap::new(),
            last_alloc:0,
        }
    }
    
    fn get_memory_at_mut(&mut self, addr: Value) ->&mut [u8]{
        let (id,offset) = addr.as_ptr().unwrap();
        &mut self.mem.get_mut(&id).unwrap()[offset as usize ..]
    }
}
fn load_asm(mut file: impl std::io::Read) -> Assembly {
    let mut asm_bytes = Vec::with_capacity(0x100);
    file.read_to_end(&mut asm_bytes)
        .expect("ERROR: Could not load the assembly file!");
    let assembly =
        postcard::from_bytes(&asm_bytes).expect("ERROR:Could not decode the assembly file!");
    assembly
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let asm = load_asm(std::fs::File::open(path).unwrap());
    let mut interpreter = InterpreterState::new(&asm);
    interpreter.run_cctor().unwrap();
}
