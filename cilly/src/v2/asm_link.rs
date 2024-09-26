use super::{
    asm::{CCTOR, TCCTOR, USER_INIT},
    Assembly, BasicBlock, CILNode, CILRoot, ClassDef, ClassDefIdx, ClassRef, ClassRefIdx,
    FieldDesc, FnSig, MethodDef, MethodDefIdx, MethodRef, StaticFieldDesc, Type,
};
impl Assembly {
    pub(crate) fn translate_type(&mut self, source: &Self, tpe: Type) -> Type {
        match tpe {
            Type::Ptr(inner) => {
                let inner = self.translate_type(source, source[inner]);
                self.nptr(inner)
            }
            Type::Ref(inner) => {
                let inner = self.translate_type(source, source[inner]);
                self.nref(inner)
            }
            Type::Int(_)
            | Type::Float(_)
            | Type::PlatformString
            | Type::PlatformChar
            | Type::Bool
            | Type::Void
            | Type::PlatformObject
            | Type::PlatformGeneric(_, _) => tpe,
            Type::ClassRef(class_ref) => {
                Type::ClassRef(self.translate_class_ref(source, class_ref))
            }
            Type::PlatformArray { elem, dims } => {
                let elem = self.translate_type(source, source[elem]);
                let elem = self.alloc_type(elem);
                Type::PlatformArray { elem, dims }
            }
            Type::FnPtr(sig) => {
                let sig = self.translate_sig(source, &source[sig]);
                Type::FnPtr(self.alloc_sig(sig))
            }
        }
    }
    pub(crate) fn translate_class_ref(
        &mut self,
        source: &Assembly,
        class_ref: ClassRefIdx,
    ) -> ClassRefIdx {
        let cref = source.class_ref(class_ref);

        let name = self.alloc_string(&source[cref.name()]);

        let asm = cref
            .asm()
            .map(|asm_name| self.alloc_string(&source[asm_name]));
        let generics = cref
            .generics()
            .iter()
            .map(|tpe| self.translate_type(source, *tpe))
            .collect();
        self.alloc_class_ref(ClassRef::new(name, asm, cref.is_valuetype(), generics))
    }
    pub(crate) fn translate_sig(&mut self, source: &Assembly, sig: &FnSig) -> FnSig {
        FnSig::new(
            sig.inputs()
                .iter()
                .map(|tpe| self.translate_type(source, *tpe))
                .collect(),
            self.translate_type(source, *sig.output()),
        )
    }
    pub(crate) fn translate_field(&mut self, source: &Assembly, field: FieldDesc) -> FieldDesc {
        let name = self.alloc_string(source[field.name()].as_ref());
        let owner = self.translate_class_ref(source, field.owner());
        let tpe = self.translate_type(source, field.tpe());
        FieldDesc::new(owner, name, tpe)
    }
    pub(crate) fn translate_static_field(
        &mut self,
        source: &Assembly,
        field: StaticFieldDesc,
    ) -> StaticFieldDesc {
        let name = self.alloc_string(source[field.name()].as_ref());
        let owner = self.translate_class_ref(source, field.owner());
        let tpe = self.translate_type(source, field.tpe());
        StaticFieldDesc::new(owner, name, tpe)
    }
    pub(crate) fn translate_method_ref(
        &mut self,
        source: &Assembly,
        method_ref: &MethodRef,
    ) -> MethodRef {
        let class = self.translate_class_ref(source, method_ref.class());
        let name = self.alloc_string(source[method_ref.name()].as_ref());
        let sig = self.translate_sig(source, &source[method_ref.sig()]);
        let sig = self.alloc_sig(sig);
        let generics = method_ref
            .generics()
            .iter()
            .map(|tpe| self.translate_type(source, *tpe))
            .collect();
        MethodRef::new(class, name, sig, method_ref.kind(), generics)
    }
    // The complexity of this function is unavoidable.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn translate_node(&mut self, source: &Assembly, node: CILNode) -> CILNode {
        match &node {
            CILNode::LdLoc(_) | CILNode::LdLocA(_) | CILNode::LdArg(_) | CILNode::LdArgA(_) => node,
            CILNode::Const(cst) => match cst.as_ref() {
                super::Const::PlatformString(pstr) => CILNode::Const(Box::new(
                    super::Const::PlatformString(self.alloc_string(source[*pstr].as_ref())),
                )),
                _ => node.clone(),
            },
            CILNode::BinOp(a, b, op) => {
                let a = self.translate_node(source, source.get_node(*a).clone());
                let b = self.translate_node(source, source.get_node(*b).clone());
                CILNode::BinOp(self.alloc_node(a), self.alloc_node(b), *op)
            }
            CILNode::UnOp(a, op) => {
                let a = self.translate_node(source, source.get_node(*a).clone());
                CILNode::UnOp(self.alloc_node(a), op.clone())
            }
            CILNode::Call(call_arg) => {
                let (mref, args) = call_arg.as_ref();
                let method_ref = self.translate_method_ref(source, &source[*mref]);
                let mref = self.alloc_methodref(method_ref);
                let args = args
                    .iter()
                    .map(|arg| {
                        let arg = self.translate_node(source, source.get_node(*arg).clone());
                        self.alloc_node(arg)
                    })
                    .collect();
                CILNode::Call(Box::new((mref, args)))
            }
            CILNode::IntCast {
                input,
                target,
                extend,
            } => {
                let input = self.translate_node(source, source.get_node(*input).clone());
                let input = self.alloc_node(input);
                CILNode::IntCast {
                    input,
                    target: *target,
                    extend: *extend,
                }
            }
            CILNode::FloatCast {
                input,
                target,
                is_signed,
            } => {
                let input = self.translate_node(source, source.get_node(*input).clone());
                let input = self.alloc_node(input);
                CILNode::FloatCast {
                    input,
                    target: *target,
                    is_signed: *is_signed,
                }
            }
            CILNode::RefToPtr(input) => {
                let input = self.translate_node(source, source.get_node(*input).clone());
                let input = self.alloc_node(input);
                CILNode::RefToPtr(input)
            }
            CILNode::PtrCast(input, cast_res) => {
                let input = self.translate_node(source, source.get_node(*input).clone());
                let input = self.alloc_node(input);
                let cast_res = match cast_res.as_ref() {
                    crate::v2::cilnode::PtrCastRes::Ptr(inner) => {
                        let inner = self.translate_type(source, source[*inner]);
                        crate::v2::cilnode::PtrCastRes::Ptr(self.alloc_type(inner))
                    }
                    crate::v2::cilnode::PtrCastRes::Ref(inner) => {
                        let inner = self.translate_type(source, source[*inner]);
                        crate::v2::cilnode::PtrCastRes::Ref(self.alloc_type(inner))
                    }
                    crate::v2::cilnode::PtrCastRes::FnPtr(sig) => {
                        let sig = self.translate_sig(source, &source[*sig]);
                        crate::v2::cilnode::PtrCastRes::FnPtr(self.alloc_sig(sig))
                    }
                    crate::v2::cilnode::PtrCastRes::USize
                    | crate::v2::cilnode::PtrCastRes::ISize => *cast_res.clone(),
                };
                CILNode::PtrCast(input, Box::new(cast_res))
            }
            CILNode::LdFieldAdress { addr, field } => {
                let field = self.translate_field(source, *source.get_field(*field));
                let field = self.alloc_field(field);
                let addr = self.translate_node(source, source.get_node(*addr).clone());
                let addr = self.alloc_node(addr);
                CILNode::LdFieldAdress { addr, field }
            }
            CILNode::LdField { addr, field } => {
                let field = self.translate_field(source, *source.get_field(*field));
                let field = self.alloc_field(field);
                let addr = self.translate_node(source, source.get_node(*addr).clone());
                let addr = self.alloc_node(addr);
                CILNode::LdField { addr, field }
            }
            CILNode::LdInd {
                addr,
                tpe,
                volitale,
            } => {
                let addr = self.translate_node(source, source.get_node(*addr).clone());
                let addr = self.alloc_node(addr);
                let tpe = self.translate_type(source, source[*tpe]);
                let tpe = self.alloc_type(tpe);
                CILNode::LdInd {
                    addr,
                    tpe,
                    volitale: *volitale,
                }
            }
            CILNode::SizeOf(tpe) => {
                let tpe = self.translate_type(source, source[*tpe]);
                let tpe = self.alloc_type(tpe);
                CILNode::SizeOf(tpe)
            }
            CILNode::GetException => CILNode::GetException,
            CILNode::IsInst(object, tpe) => {
                let object = self.translate_node(source, source.get_node(*object).clone());
                let object = self.alloc_node(object);
                let tpe = self.translate_type(source, source[*tpe]);
                let tpe = self.alloc_type(tpe);
                CILNode::IsInst(object, tpe)
            }
            CILNode::CheckedCast(object, tpe) => {
                let object = self.translate_node(source, source.get_node(*object).clone());
                let object = self.alloc_node(object);
                let tpe = self.translate_type(source, source[*tpe]);
                let tpe = self.alloc_type(tpe);
                CILNode::CheckedCast(object, tpe)
            }
            CILNode::CallI(args) => {
                let (fnptr, sig, args) = args.as_ref();
                let fnptr = self.translate_node(source, source.get_node(*fnptr).clone());
                let fnptr = self.alloc_node(fnptr);
                let sig = self.translate_sig(source, &source[*sig]);
                let sig = self.alloc_sig(sig);
                let args = args
                    .iter()
                    .map(|arg| {
                        let arg = self.translate_node(source, source.get_node(*arg).clone());
                        self.alloc_node(arg)
                    })
                    .collect();
                CILNode::CallI(Box::new((fnptr, sig, args)))
            }
            CILNode::LocAlloc { size } => {
                let size = self.translate_node(source, source.get_node(*size).clone());
                let size = self.alloc_node(size);
                CILNode::LocAlloc { size }
            }
            CILNode::LdStaticField(sfld) => {
                let sfld = self.translate_static_field(source, *source.get_static_field(*sfld));
                let sfld = self.alloc_sfld(sfld);
                CILNode::LdStaticField(sfld)
            }
            CILNode::LdStaticFieldAdress(sfld) => {
                let sfld = self.translate_static_field(source, *source.get_static_field(*sfld));
                let sfld = self.alloc_sfld(sfld);
                CILNode::LdStaticFieldAdress(sfld)
            }
            CILNode::LdFtn(mref) => {
                let method_ref = self.translate_method_ref(source, &source[*mref]);
                let mref = self.alloc_methodref(method_ref);
                CILNode::LdFtn(mref)
            }
            CILNode::LdTypeToken(tpe) => {
                let tpe = self.translate_type(source, source[*tpe]);
                let tpe = self.alloc_type(tpe);
                CILNode::LdTypeToken(tpe)
            }
            CILNode::LdLen(len) => {
                let len = self.translate_node(source, source.get_node(*len).clone());
                let len = self.alloc_node(len);
                CILNode::LdLen(len)
            }
            CILNode::LocAllocAlgined { tpe, align } => {
                let tpe = self.translate_type(source, source[*tpe]);
                let tpe = self.alloc_type(tpe);
                CILNode::LocAllocAlgined { tpe, align: *align }
            }
            CILNode::LdElelemRef { array, index } => {
                let array = self.translate_node(source, source.get_node(*array).clone());
                let array = self.alloc_node(array);
                let index = self.translate_node(source, source.get_node(*index).clone());
                let index = self.alloc_node(index);
                CILNode::LdElelemRef { array, index }
            }
            CILNode::UnboxAny { object, tpe } => {
                let object = self.translate_node(source, source.get_node(*object).clone());
                let object = self.alloc_node(object);
                let tpe = self.translate_type(source, source[*tpe]);
                let tpe = self.alloc_type(tpe);
                CILNode::UnboxAny { object, tpe }
            }
        }
    }
    // The complexity of this function is unavoidable.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn translate_root(&mut self, source: &Assembly, root: CILRoot) -> CILRoot {
        match root {
            CILRoot::Unreachable(str) => {
                let str = self.alloc_string(&source[str]);
                CILRoot::Unreachable(str)
            }
            CILRoot::StLoc(loc, node) => {
                let node = self.translate_node(source, source.get_node(node).clone());
                let node = self.alloc_node(node);
                CILRoot::StLoc(loc, node)
            }
            CILRoot::StArg(loc, node) => {
                let node = self.translate_node(source, source.get_node(node).clone());
                let node = self.alloc_node(node);
                CILRoot::StArg(loc, node)
            }
            CILRoot::Ret(node) => {
                let node = self.translate_node(source, source.get_node(node).clone());
                let node = self.alloc_node(node);
                CILRoot::Ret(node)
            }
            CILRoot::Pop(node) => {
                let node = self.translate_node(source, source.get_node(node).clone());
                let node = self.alloc_node(node);
                CILRoot::Pop(node)
            }
            CILRoot::Throw(node) => {
                let node = self.translate_node(source, source.get_node(node).clone());
                let node = self.alloc_node(node);
                CILRoot::Throw(node)
            }
            CILRoot::Branch(branch) => {
                let (target, sub_target, cond) = branch.as_ref();
                let cond = cond.as_ref().map(|cond| match cond {
                    super::cilroot::BranchCond::True(cond) => {
                        let cond = self.translate_node(source, source.get_node(*cond).clone());
                        let cond = self.alloc_node(cond);
                        super::cilroot::BranchCond::True(cond)
                    }
                    super::cilroot::BranchCond::False(cond) => {
                        let cond = self.translate_node(source, source.get_node(*cond).clone());
                        let cond = self.alloc_node(cond);
                        super::cilroot::BranchCond::False(cond)
                    }
                    super::cilroot::BranchCond::Eq(a, b) => {
                        let a = self.translate_node(source, source.get_node(*a).clone());
                        let a = self.alloc_node(a);
                        let b = self.translate_node(source, source.get_node(*b).clone());
                        let b = self.alloc_node(b);
                        super::cilroot::BranchCond::Eq(a, b)
                    }
                    super::cilroot::BranchCond::Ne(a, b) => {
                        let a = self.translate_node(source, source.get_node(*a).clone());
                        let a = self.alloc_node(a);
                        let b = self.translate_node(source, source.get_node(*b).clone());
                        let b = self.alloc_node(b);
                        super::cilroot::BranchCond::Ne(a, b)
                    }
                    super::cilroot::BranchCond::Lt(a, b, cmp_kind) => {
                        let a = self.translate_node(source, source.get_node(*a).clone());
                        let a = self.alloc_node(a);
                        let b = self.translate_node(source, source.get_node(*b).clone());
                        let b = self.alloc_node(b);
                        super::cilroot::BranchCond::Lt(a, b, cmp_kind.clone())
                    }
                    super::cilroot::BranchCond::Gt(a, b, cmp_kind) => {
                        let a = self.translate_node(source, source.get_node(*a).clone());
                        let a = self.alloc_node(a);
                        let b = self.translate_node(source, source.get_node(*b).clone());
                        let b = self.alloc_node(b);
                        super::cilroot::BranchCond::Gt(a, b, cmp_kind.clone())
                    }
                    super::cilroot::BranchCond::Le(a, b, cmp_kind) => {
                        let a = self.translate_node(source, source.get_node(*a).clone());
                        let a = self.alloc_node(a);
                        let b = self.translate_node(source, source.get_node(*b).clone());
                        let b = self.alloc_node(b);
                        super::cilroot::BranchCond::Le(a, b, cmp_kind.clone())
                    }
                    super::cilroot::BranchCond::Ge(a, b, cmp_kind) => {
                        let a = self.translate_node(source, source.get_node(*a).clone());
                        let a = self.alloc_node(a);
                        let b = self.translate_node(source, source.get_node(*b).clone());
                        let b = self.alloc_node(b);
                        super::cilroot::BranchCond::Ge(a, b, cmp_kind.clone())
                    }
                });
                CILRoot::Branch(Box::new((*target, *sub_target, cond)))
            }
            CILRoot::VoidRet | CILRoot::Break | CILRoot::Nop | CILRoot::ReThrow => root,
            CILRoot::SourceFileInfo {
                line_start,
                line_len,
                col_start,
                col_len,
                file,
            } => {
                let file = self.alloc_string(source[file].as_ref());
                CILRoot::SourceFileInfo {
                    line_start,
                    line_len,
                    col_start,
                    col_len,
                    file,
                }
            }
            CILRoot::SetField(info) => {
                let (field, addr, val) = info.as_ref();
                let field = self.translate_field(source, *source.get_field(*field));
                let field = self.alloc_field(field);
                let addr = self.translate_node(source, source.get_node(*addr).clone());
                let addr = self.alloc_node(addr);
                let val = self.translate_node(source, source.get_node(*val).clone());
                let val = self.alloc_node(val);
                CILRoot::SetField(Box::new((field, addr, val)))
            }
            CILRoot::Call(call_arg) => {
                let (mref, args) = call_arg.as_ref();
                let method_ref = self.translate_method_ref(source, &source[*mref]);
                let mref = self.alloc_methodref(method_ref);
                let args = args
                    .iter()
                    .map(|arg| {
                        let arg = self.translate_node(source, source.get_node(*arg).clone());
                        self.alloc_node(arg)
                    })
                    .collect();
                CILRoot::Call(Box::new((mref, args)))
            }
            CILRoot::StInd(info) => {
                let (addr, val, tpe, volitile) = info.as_ref();
                let addr = self.translate_node(source, source.get_node(*addr).clone());
                let addr = self.alloc_node(addr);
                let val = self.translate_node(source, source.get_node(*val).clone());
                let val = self.alloc_node(val);
                let tpe = self.translate_type(source, *tpe);
                CILRoot::StInd(Box::new((addr, val, tpe, *volitile)))
            }
            CILRoot::CpObj { src, dst, tpe } => {
                let src = self.translate_node(source, source.get_node(src).clone());
                let src = self.alloc_node(src);
                let dst = self.translate_node(source, source.get_node(dst).clone());
                let dst = self.alloc_node(dst);
                let tpe = self.translate_type(source, source[tpe]);
                CILRoot::CpObj {
                    src,
                    dst,
                    tpe: self.alloc_type(tpe),
                }
            }
            CILRoot::InitBlk(info) => {
                let (dst, val, count) = info.as_ref();
                let dst = self.translate_node(source, source.get_node(*dst).clone());
                let dst = self.alloc_node(dst);
                let val = self.translate_node(source, source.get_node(*val).clone());
                let val = self.alloc_node(val);
                let count = self.translate_node(source, source.get_node(*count).clone());
                let count = self.alloc_node(count);
                CILRoot::InitBlk(Box::new((dst, val, count)))
            }
            CILRoot::CpBlk(info) => {
                let (dst, src, len) = info.as_ref();
                let dst = self.translate_node(source, source.get_node(*dst).clone());
                let dst = self.alloc_node(dst);
                let src = self.translate_node(source, source.get_node(*src).clone());
                let src = self.alloc_node(src);
                let len = self.translate_node(source, source.get_node(*len).clone());
                let len = self.alloc_node(len);
                CILRoot::CpBlk(Box::new((dst, src, len)))
            }
            CILRoot::CallI(args) => {
                let (fnptr, sig, args) = args.as_ref();
                let fnptr = self.translate_node(source, source.get_node(*fnptr).clone());
                let fnptr = self.alloc_node(fnptr);
                let sig = self.translate_sig(source, &source[*sig]);
                let sig = self.alloc_sig(sig);
                let args = args
                    .iter()
                    .map(|arg| {
                        let arg = self.translate_node(source, source.get_node(*arg).clone());
                        self.alloc_node(arg)
                    })
                    .collect();
                CILRoot::CallI(Box::new((fnptr, sig, args)))
            }
            CILRoot::ExitSpecialRegion { target, source } => {
                CILRoot::ExitSpecialRegion { target, source }
            }
            CILRoot::SetStaticField { field, val } => {
                let val = self.translate_node(source, source.get_node(val).clone());
                let val = self.alloc_node(val);
                let field = self.translate_static_field(source, *source.get_static_field(field));
                let field = self.alloc_sfld(field);
                CILRoot::SetStaticField { field, val }
            }
        }
    }
    pub(crate) fn translate_block(&mut self, source: &Assembly, block: &BasicBlock) -> BasicBlock {
        let roots = block
            .roots()
            .iter()
            .map(|root| {
                let root = self.translate_root(source, source.get_root(*root).clone());
                self.alloc_root(root)
            })
            .collect();
        let handler = block.handler().map(|blocks| {
            blocks
                .iter()
                .map(|block| self.translate_block(source, block))
                .collect()
        });
        BasicBlock::new(roots, block.block_id(), handler)
    }
    pub(crate) fn translate_method_def(&mut self, source: &Assembly, def: &MethodDef) -> MethodDef {
        let class = self.translate_class_ref(source, *def.class());

        // OK, becuase our caller translates the parrent of this class too.
        let class = ClassDefIdx(class);
        let name = self.alloc_string(source[def.name()].as_ref());
        let sig = self.translate_sig(source, &source[def.sig()]);
        let sig = self.alloc_sig(sig);
        let method_impl = match def.implementation() {
            super::MethodImpl::MethodBody { blocks, locals } => {
                let blocks = blocks
                    .iter()
                    .map(|block| self.translate_block(source, block))
                    .collect();
                let locals = locals
                    .iter()
                    .map(|(name, tpe)| {
                        let tpe = self.translate_type(source, source[*tpe]);
                        (
                            name.map(|name| self.alloc_string(source[name].as_ref())),
                            self.alloc_type(tpe),
                        )
                    })
                    .collect();
                super::MethodImpl::MethodBody { blocks, locals }
            }
            super::MethodImpl::Extern {
                lib,
                preserve_errno,
            } => {
                let lib = self.alloc_string(source[*lib].as_ref());
                super::MethodImpl::Extern {
                    lib,
                    preserve_errno: *preserve_errno,
                }
            }
            super::MethodImpl::AliasFor(mref) => {
                let method_ref = self.translate_method_ref(source, &source[*mref]);
                let mref = self.alloc_methodref(method_ref);
                super::MethodImpl::AliasFor(mref)
            }
            super::MethodImpl::Missing => super::MethodImpl::Missing,
        };
        let arg_names = def
            .arg_names()
            .iter()
            .map(|arg| arg.map(|arg| self.alloc_string(source[arg].as_ref())))
            .collect();
        MethodDef::new(
            *def.access(),
            class,
            name,
            sig,
            def.kind(),
            method_impl,
            arg_names,
        )
    }
    pub(crate) fn translate_class_def(&mut self, source: &Assembly, def: &ClassDef) -> ClassDef {
        let name = self.alloc_string(source[def.name()].as_ref());
        let extends = def
            .extends()
            .map(|cref| self.translate_class_ref(source, cref));
        let fields = def
            .fields()
            .iter()
            .map(|(tpe, name, offset)| {
                let tpe = self.translate_type(source, *tpe);
                let name = self.alloc_string(source[*name].as_ref());
                (tpe, name, *offset)
            })
            .collect();
        let static_fields = def
            .static_fields()
            .iter()
            .map(|(tpe, name, thread_local)| {
                let tpe = self.translate_type(source, *tpe);
                let name = self.alloc_string(source[*name].as_ref());
                (tpe, name, *thread_local)
            })
            .collect();
        let translated = ClassDef::new(
            name,
            def.is_valuetype(),
            def.generics(),
            extends,
            fields,
            static_fields,
            *def.access(),
            def.explict_size(),
        );
        let class_ref = self.alloc_class_ref(translated.ref_to());
        let (defs_mut, _) = self.class_defs_mut_strings();
        match defs_mut.entry(ClassDefIdx(class_ref)) {
            std::collections::hash_map::Entry::Occupied(mut occupied) => {
                occupied.get_mut().merge_defs(translated.clone());
            }
            std::collections::hash_map::Entry::Vacant(vacant) => {
                vacant.insert(translated.clone());
            }
        }

        def
            .methods()
            .iter()
            .for_each(|mdef| {
                let mut method_definition = self.translate_method_def(source, source.method_def(*mdef));
                let method_ref = self.alloc_methodref(method_definition.ref_to());
                // 1st Take the orignal method, if it exists(we need this to be able to mutate methods)
                let original = self.method_defs().get(&MethodDefIdx(method_ref));
                let method_definition = match original {
                    Some(original) => {
                        assert_eq!(method_definition.name(), original.name());
                        // Check if this method has a special name, and needs merging.
                        let name = &self[method_definition.name()];
                        if SPECIAL_METHOD_NAMES.iter().any(|val| **val == *name) {
                            // Not special, proly does not need merging, so we can check if it matches and go on our merry way.
                            assert_eq!(method_definition.access(), original.access());
                            assert_eq!(method_definition.class(), original.class());
                            assert_eq!(method_definition.sig(), original.sig());
                            assert_eq!(method_definition.kind(), original.kind());
                            method_definition
                                .implementation_mut()
                                .merge_cctor_impls(original.implementation(), self);
                            method_definition
                        } else {
                            // Not special, proly does not need merging, so we can check if it matches and go on our merry way.
                            assert_eq!(method_definition.access(), original.access());
                            assert_eq!(method_definition.class(), original.class());
                            assert_eq!(method_definition.sig(), original.sig());
                            assert_eq!(method_definition.kind(), original.kind());
                            // EXPENSIVE, consider making this check debug only.
                            let (left_val,right_val) = (&(method_definition.implementation()), &(original.implementation()));
                            if!(*left_val== *right_val){
                                eprintln!("WARNING: linking methods with diveriging implmenentations. This is usualy a sign of a bug. {left_val:?} {right_val:?}");
                            };
                            method_definition
                        }
                    }
                    None => method_definition,
                };
                self.new_method(method_definition);
            })
            ;
        translated
    }
}
const SPECIAL_METHOD_NAMES: &[&str] = &[CCTOR, TCCTOR, USER_INIT];
