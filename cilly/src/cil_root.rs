use crate::bimap::Interned;
use crate::cilnode::IsPure;
use crate::cilnode::MethodKind;
use crate::typecheck::TypeCheckError;
use crate::v2::method::LocalDef;
use crate::FieldDesc;
use crate::{
    call,
    cil_node::{CallOpArgs, V1Node},
    AsmString, IString,
};
use crate::{Assembly, ClassRef, FnSig, MethodRef, StaticFieldDesc, Type};
use serde::{Deserialize, Serialize};
#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub enum V1Root {
    STLoc {
        local: u32,
        tree: V1Node,
    },
    BTrue {
        target: u32,
        sub_target: u32,
        cond: V1Node,
    },
    BFalse {
        target: u32,
        sub_target: u32,
        cond: V1Node,
    },
    BEq {
        target: u32,
        sub_target: u32,
        a: Box<V1Node>,
        b: Box<V1Node>,
    },
    BLt {
        target: u32,
        sub_target: u32,
        a: Box<V1Node>,
        b: Box<V1Node>,
    },
    BLtUn {
        target: u32,
        sub_target: u32,
        a: Box<V1Node>,
        b: Box<V1Node>,
    },
    BGt {
        target: u32,
        sub_target: u32,
        a: Box<V1Node>,
        b: Box<V1Node>,
    },
    BGtUn {
        target: u32,
        sub_target: u32,
        a: Box<V1Node>,
        b: Box<V1Node>,
    },
    BLe {
        target: u32,
        sub_target: u32,
        a: Box<V1Node>,
        b: Box<V1Node>,
    },
    BGe {
        target: u32,
        sub_target: u32,
        a: Box<V1Node>,
        b: Box<V1Node>,
    },
    BNe {
        target: u32,
        sub_target: u32,
        a: Box<V1Node>,
        b: Box<V1Node>,
    },
    GoTo {
        target: u32,
        sub_target: u32,
    },

    Call {
        site: Interned<MethodRef>,
        args: Box<[V1Node]>,
    },
    SetField {
        addr: Box<V1Node>,
        value: Box<V1Node>,
        desc: Interned<FieldDesc>,
    },

    CpBlk {
        dst: Box<V1Node>,
        src: Box<V1Node>,
        len: Box<V1Node>,
    },
    STIndI8(V1Node, V1Node),
    STIndI16(V1Node, V1Node),
    STIndI32(V1Node, V1Node),
    STIndI64(V1Node, V1Node),
    STIndISize(V1Node, V1Node),
    // addr, val, points_to
    STIndPtr(V1Node, V1Node, Box<Type>),
    STIndF64(V1Node, V1Node),
    STIndF32(V1Node, V1Node),
    STObj {
        tpe: Box<Type>,
        addr_calc: Box<V1Node>,
        value_calc: Box<V1Node>,
    },
    STArg {
        arg: u32,
        tree: V1Node,
    },
    Break,
    Nop,
    InitBlk {
        dst: Box<V1Node>,
        val: Box<V1Node>,
        count: Box<V1Node>,
    },
    CallVirt {
        site: Interned<MethodRef>,
        args: Box<[V1Node]>,
    },
    Ret {
        tree: V1Node,
    },
    Pop {
        tree: V1Node,
    },
    VoidRet,
    Throw(V1Node),
    ReThrow,
    CallI {
        sig: Box<FnSig>,
        fn_ptr: Box<V1Node>,
        args: Box<[V1Node]>,
    },
    JumpingPad {
        source: u32,
        target: u32,
    },
    SetStaticField {
        descr: Box<StaticFieldDesc>,
        value: V1Node,
    },
    SourceFileInfo(SFI),
    OptimizedSourceFileInfo(std::ops::Range<u64>, std::ops::Range<u64>, AsmString),
    /// Marks the inner pointer operation as volatile.
    Volatile(Box<Self>),
    InitObj(V1Node, Interned<Type>),
    V2(Interned<crate::v2::CILRoot>),
}
pub type SFI = Box<(std::ops::Range<u64>, std::ops::Range<u64>, IString)>;
impl V1Root {
    pub fn try_typecheck(
        &self,
        asm: &mut Assembly,
        fn_sig: Interned<FnSig>,
        locals: &[LocalDef],
    ) -> Result<(), TypeCheckError> {
        let Self::V2(root) = self else { return Ok(()) };
        asm[*root].clone().typecheck(fn_sig, locals, asm)
    }
    #[must_use]
    pub fn throw(msg: &str, asm: &mut Assembly) -> Self {
        let class = ClassRef::exception(asm);

        let name = asm.alloc_string(".ctor");
        let signature = asm.sig([class.into(), Type::PlatformString], Type::Void);
        Self::Throw(V1Node::NewObj(Box::new(CallOpArgs {
            site: asm.alloc_methodref(MethodRef::new(
                class,
                name,
                signature,
                MethodKind::Constructor,
                vec![].into(),
            )),
            args: [V1Node::LdStr(msg.into())].into(),
            is_pure: IsPure::NOT,
        })))
    }
    #[must_use]
    pub fn debug(msg: &str, asm: &mut Assembly) -> Self {
        let class = ClassRef::console(asm);

        let name = asm.alloc_string("WriteLine");
        let signature = asm.sig([Type::PlatformString], Type::Void);
        let message = tiny_message(msg, asm);
        Self::Call {
            site: asm.alloc_methodref(MethodRef::new(
                class,
                name,
                signature,
                MethodKind::Static,
                vec![].into(),
            )),
            args: [message].into(),
        }
    }
    pub fn targets(&self, targets: &mut Vec<(u32, u32)>) {
        match self {
            Self::BTrue {
                target, sub_target, ..
            }
            | Self::BFalse {
                target, sub_target, ..
            }
            | Self::BEq {
                target, sub_target, ..
            }
            | Self::BNe {
                target, sub_target, ..
            }
            | Self::GoTo { target, sub_target } => {
                targets.push((*target, *sub_target));
            }
            _ => (),
        }
    }
    pub fn fix_for_exception_handler(&mut self, id: u32) {
        match self {
            Self::BTrue {
                target, sub_target, ..
            }
            | Self::BFalse {
                target, sub_target, ..
            }
            | Self::BEq {
                target, sub_target, ..
            }
            | Self::BNe {
                target, sub_target, ..
            }
            | Self::BLt {
                target, sub_target, ..
            }
            | Self::BLtUn {
                target, sub_target, ..
            }
            | Self::BLe {
                target, sub_target, ..
            }
            | Self::BGt {
                target, sub_target, ..
            }
            | Self::BGtUn {
                target, sub_target, ..
            }
            | Self::BGe {
                target, sub_target, ..
            }
            | Self::GoTo { target, sub_target } => {
                assert_eq!(
                    *sub_target, 0,
                    "An exception handler can't contain inner exception handler!"
                );
                *sub_target = *target;
                *target = id;
            }
            _ => (),
        }
    }

    #[must_use]
    pub fn source_info(
        file: &str,
        line: std::ops::Range<u64>,
        column: std::ops::Range<u64>,
    ) -> Self {
        assert!(
            column.start < column.end,
            "PDB files must have columns that contain at least one element "
        );
        Self::SourceFileInfo(Box::new((line, column, file.to_owned().into())))
    }
    #[must_use]
    pub fn set_field(addr: V1Node, value: V1Node, desc: Interned<FieldDesc>) -> Self {
        Self::SetField {
            addr: Box::new(addr),
            value: Box::new(value),
            desc,
        }
    }
}
fn tiny_message(msg: &str, asm: &mut Assembly) -> V1Node {
    let pieces: Vec<_> = msg.split_inclusive(char::is_whitespace).collect();
    runtime_string(&pieces, asm)
}
fn runtime_string(pieces: &[&str], asm: &mut Assembly) -> V1Node {
    //
    match pieces.len() {
        0 => panic!("Incorrect piece count"),
        1 => V1Node::LdStr(pieces[0].to_owned().into()),
        2 => {
            let mref = MethodRef::new(
                ClassRef::string(asm),
                asm.alloc_string("Concat"),
                asm.sig(
                    [Type::PlatformString, Type::PlatformString],
                    Type::PlatformString,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                asm.alloc_methodref(mref),
                [
                    V1Node::LdStr(pieces[0].to_owned().into()),
                    V1Node::LdStr(pieces[1].to_owned().into())
                ]
            )
        }
        3 => {
            let mref = MethodRef::new(
                ClassRef::string(asm),
                asm.alloc_string("Concat"),
                asm.sig(
                    [
                        Type::PlatformString,
                        Type::PlatformString,
                        Type::PlatformString,
                    ],
                    Type::PlatformString,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                asm.alloc_methodref(mref),
                [
                    V1Node::LdStr(pieces[0].to_owned().into()),
                    V1Node::LdStr(pieces[1].to_owned().into()),
                    V1Node::LdStr(pieces[2].to_owned().into())
                ]
            )
        }
        4 => {
            let mref = MethodRef::new(
                ClassRef::string(asm),
                asm.alloc_string("Concat"),
                asm.sig(
                    [
                        Type::PlatformString,
                        Type::PlatformString,
                        Type::PlatformString,
                        Type::PlatformString,
                    ],
                    Type::PlatformString,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                asm.alloc_methodref(mref),
                [
                    V1Node::LdStr(pieces[0].to_owned().into()),
                    V1Node::LdStr(pieces[1].to_owned().into()),
                    V1Node::LdStr(pieces[2].to_owned().into()),
                    V1Node::LdStr(pieces[3].to_owned().into())
                ]
            )
        }
        _ => {
            let sub_part = pieces.len() / 4;
            let mref = MethodRef::new(
                ClassRef::string(asm),
                asm.alloc_string("Concat"),
                asm.sig(
                    [
                        Type::PlatformString,
                        Type::PlatformString,
                        Type::PlatformString,
                        Type::PlatformString,
                    ],
                    Type::PlatformString,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                asm.alloc_methodref(mref),
                [
                    runtime_string(&pieces[..sub_part], asm),
                    runtime_string(&pieces[sub_part..(sub_part * 2)], asm),
                    runtime_string(&pieces[(sub_part * 2)..(sub_part * 3)], asm),
                    runtime_string(&pieces[(sub_part * 3)..], asm)
                ]
            )
        }
    }
}
