#[macro_export]
macro_rules! binop {
    // |closure| + |closure|
    (|$asm1:ident|$lhs:expr,|$asm2:ident|$rhs:expr,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            asm.biop(
                |$asm1: &mut $crate::asm::Assembly| $lhs.into_idx($asm1),
                |$asm2: &mut $crate::asm::Assembly| $rhs.into_idx($asm2),
                $op,
            )
        }
    };
    (|$asm1:ident|$lhs:block,|$asm2:ident|$rhs:expr,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            asm.biop(
                |$asm1: &mut $crate::asm::Assembly| $lhs.into_idx($asm1),
                |$asm2: &mut $crate::asm::Assembly| $rhs.into_idx($asm2),
                $op,
            )
        }
    };
    (|$asm1:ident|$lhs:expr,|$asm2:ident|$rhs:block,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            asm.biop(
                |$asm1: &mut $crate::asm::Assembly| $lhs.into_idx($asm1),
                |$asm2: &mut $crate::asm::Assembly| $rhs.into_idx($asm2),
                $op,
            )
        }
    };
    (|$asm1:ident|$lhs:block,|$asm2:ident|$rhs:block,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            asm.biop(
                |$asm1: &mut $crate::asm::Assembly| $lhs.into_idx($asm1),
                |$asm2: &mut $crate::asm::Assembly| $rhs.into_idx($asm2),
                $op,
            )
        }
    };
    // block + |closure|
    ($lhs:expr,|$asm2:ident|$rhs:expr,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            asm.biop(
                $lhs,
                |$asm2: &mut $crate::asm::Assembly| $rhs.into_idx($asm2),
                $op,
            )
        }
    };
    ($lhs:block,|$asm2:ident|$rhs:expr,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            asm.biop(
                $lhs,
                |$asm2: &mut $crate::asm::Assembly| $rhs.into_idx($asm2),
                $op,
            )
        }
    };
    ($lhs:expr,|$asm2:ident|$rhs:block,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            asm.biop(
                $lhs,
                |$asm2: &mut $crate::asm::Assembly| $rhs.into_idx($asm2),
                $op,
            )
        }
    };
    ($lhs:block,|$asm2:ident|$rhs:block,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            asm.biop(
                $lhs,
                |$asm2: &mut $crate::asm::Assembly| $rhs.into_idx($asm2),
                $op,
            )
        }
    };
    // block + |closure|
    (|$asm1:ident|$lhs:expr,$rhs:expr,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            asm.biop(
                |$asm1: &mut $crate::asm::Assembly| $lhs.into_idx($asm1),
                rhs,
                $op,
            )
        }
    };
    (|$asm1:ident|$lhs:expr,$rhs:block,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            asm.biop(
                |$asm1: &mut $crate::asm::Assembly| $lhs.into_idx($asm1),
                rhs,
                $op,
            )
        }
    };
    (|$asm1:ident|$lhs:block,$rhs:expr,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            asm.biop(
                |$asm1: &mut $crate::asm::Assembly| $lhs.into_idx($asm1),
                rhs,
                $op,
            )
        }
    };
    (|$asm1:ident|$lhs:block,$rhs:block,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            asm.biop(
                |$asm1: &mut $crate::asm::Assembly| $lhs.into_idx($asm1),
                rhs,
                $op,
            )
        }
    };
    // block + block
    ($lhs:expr,$rhs:expr,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| asm.biop($lhs, $rhs, $op)
    };
    ($lhs:block,$rhs:expr,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| asm.biop($lhs, $rhs, $op)
    };
    ($lhs:expr,$rhs:block,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| asm.biop($lhs, $rhs, $op)
    };
    ($lhs:block,$rhs:block,$op:expr) => {
        |asm: &mut $crate::asm::Assembly| asm.biop($lhs, $rhs, $op)
    };
}
#[macro_export]
macro_rules! gen_binop {
    ($name:ident,$op:expr) => {
        #[macro_export]
        macro_rules! $name {
                        // |closure| + |closure|
                        (|$asm1:ident|$lhs:expr,|$asm2:ident|$rhs:expr) => {{
                            use $crate::BinOp;
                            $crate:binop!(|$asm1| $lhs, |$asm2| $rhs, $op)}
                        };
                        (|$asm1:ident|$lhs:block,|$asm2:ident|$rhs:expr) => {{
                            use $crate::BinOp;
                            $crate:binop!(|$asm1| $lhs, |$asm2| $rhs, $op)}
                        };
                        (|$asm1:ident|$lhs:expr,|$asm2:ident|$rhs:block) => {{  use $crate::BinOp;
                            $crate:binop!(|$asm1| $lhs, |$asm2| $rhs, $op)}
                        };
                        (|$asm1:ident|$lhs:block,|$asm2:ident|$rhs:block) => {{ use $crate::BinOp;
                            $crate:binop!(|$asm1| $lhs, |$asm2| $rhs, $op) }
                        };
                        // block + |closure|
                        ($lhs:expr,|$asm2:ident|$rhs:expr) => {{use $crate::BinOp;
                            $crate:binop!($lhs, |$asm2| $rhs, $op)  }
                        };
                        (|$lhs:block,|$asm2:ident|$rhs:expr) => {{ use $crate::BinOp;
                            $crate:binop!($lhs, |$asm2| $rhs, $op) }
                        };
                        ($lhs:expr,|$asm2:ident|$rhs:block) => {{ use $crate::BinOp;
                            $crate:binop!($lhs, |$asm2| $rhs, $op)}
                        };
                        ($lhs:block,|$asm2:ident|$rhs:block) => {{ use $crate::BinOp;
                            $crate:binop!($lhs, |$asm2| $rhs, $op)}
                        };
                        // |closure| + block
                        (|$asm1:ident|$lhs:expr,$rhs:expr) => {{ use $crate::BinOp;
                            $crate:binop!(|$asm1| $lhs, $rhs, $op)}
                        };
                        (|$asm1:ident|$lhs:block,$rhs:expr) => {{ use $crate::BinOp;
                            $crate:binop!(|$asm1| $lhs, $rhs, $op)}
                        };
                        (|$asm1:ident|$lhs:expr,|$asm2:ident|$rhs:block) => {{ use $crate::BinOp;
                            $crate:binop!(|$asm1| $lhs, $rhs, $op)}
                        };
                        (|$asm1:ident|$lhs:block,|$asm2:ident|$rhs:block) => {{ use $crate::BinOp;
                            $crate:binop!(|$asm1| $lhs, $rhs, $op)}
                        }; // block + block
                        ($lhs:expr,$rhs:expr) => {{ use $crate::BinOp;
                            $crate::binop!({$lhs}, $rhs, $op)}
                        };
                        ($lhs:block,$rhs:expr) => {{ use $crate::BinOp;
                            $crate:binop!( $lhs, $rhs, $op)}
                        };
                        ($lhs:expr,|$asm2:ident|$rhs:block) => {{ use $crate::BinOp;
                            $crate:binop!( $lhs, $rhs, $op)}
                        };
                        ($lhs:block,|$asm2:ident|$rhs:block) => {{ use $crate::BinOp;
                            $crate:binop!( $lhs, $rhs, $op)}
                        };
                        }
    };
}
#[macro_export]
macro_rules! size_of {
    (()) => {
        compile_error!("Attempt to take the size of void type (), which is not allowed")
    };
    (usize) => {{
        use $crate::IntoAsmIndex;
        |asm: &mut $crate::asm::Assembly| {
            <$crate::CILNode as IntoAsmIndex<$crate::v2::Interned<$crate::v2::CILNode>>>::into_idx(
                $crate::CILNode::SizeOf(asm.alloc_type($crate::Type::Int($crate::Int::USize))),
                asm,
            )
        }
    }};
    (|$asm:ident|$val:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            <$crate::CILNode as IntoAsmIndex<$crate::Interned<CILNode>>>::into_idx(
                $crate::CILNode::SizeOf(|$asm| { $val }.into_idx(asm)),
            )
        }
    };
    (|$asm:ident|$val:block) => {
        |asm: &mut $crate::asm::Assembly| {
            <$crate::CILNode as IntoAsmIndex<$crate::Interned<CILNode>>>::into_idx(
                $crate::CILNode::SizeOf(|$asm| { $val }.into_idx(asm)),
            )
        }
    };
    ($val:expr) => {{
        use $crate::IntoAsmIndex;
        |asm: &mut $crate::asm::Assembly| {
            <$crate::CILNode as IntoAsmIndex<$crate::v2::Interned<$crate::v2::CILNode>>>::into_idx(
                $crate::CILNode::SizeOf($val.into_idx(asm)),
                asm,
            )
        }
    }};
    ($val:block) => {
        |asm: &mut $crate::asm::Assembly| {
            <$crate::CILNode as IntoAsmIndex<$crate::Interned<CILNode>>>::into_idx(
                $crate::CILNode::SizeOf($val.into_idx(asm)),
            )
        }
    };
}
gen_binop! {add,  BinOp::Add}
gen_binop! {sub,  BinOp::Sub}
gen_binop! {mul, BinOp::Mul}
#[macro_export]
macro_rules! zero_extend {
    (|$asm:ident|$val:expr,$ty:ty) => {{
        #[allow(unused_must_use)]
        {
            |asm: &mut $crate::asm::Assembly| {
                use $crate::IntoAsmIndex;

                    asm.int_cast(
                        |$asm| $val,
                        <$ty as $crate::IntoIntType>::int_type(),
                        $crate::cilnode::ExtendKind::ZeroExtend,
                    ),
                    asm,

            }
        }
    }};
    (|$asm:ident|$val:block,$ty:ty) => {{
        #[allow(unused_must_use)]
        {
            |asm: &mut $crate::asm::Assembly| {
                use $crate::IntoAsmIndex;
                <$crate::CILNode as IntoAsmIndex<$crate::Interned<CILNode>>>::into_idx(
                    asm.int_cast(
                        |$asm| $val,
                        <$ty as $crate::IntoIntType>::int_type(),
                        $crate::cilnode::ExtendKind::ZeroExtend,
                    ),
                    asm,
                )
            }
        }
    }};
    ($val:expr,$ty:ty) => {{
        #[allow(unused_must_use)]
        {
            |asm: &mut $crate::asm::Assembly| {


                    asm.int_cast(
                        $val,
                        <$ty as $crate::IntoIntType>::int_type(),
                        $crate::cilnode::ExtendKind::ZeroExtend,
                    )


            }
        }
    }};
    ($val:block,$ty:ty) => {{
        #[allow(unused_must_use)]
        {
            |asm: &mut $crate::asm::Assembly| {
                use $crate::IntoAsmIndex;
                <$crate::CILNode as IntoAsmIndex<$crate::Interned<CILNode>>>::into_idx(
                    asm.int_cast(
                        $val,
                        <$ty as $crate::IntoIntType>::int_type(),
                        $crate::cilnode::ExtendKind::ZeroExtend,
                    ),
                    asm,
                )
            }
        }
    }};
}
#[macro_export]
macro_rules! ptr_cast {
    ($val:expr,*$ptr:expr) => {
        |asm: &mut $crate::asm::Assembly| {
            use $crate::IntoAsmIndex;
            <$crate::CILNode as IntoAsmIndex<$crate::Interned<CILNode>>>::into_idx(
                asm.ptr_cast($val, $crate::cilnode::PtrCastRes::Ptr($ptr)),
                asm,
            )
        }
    };
}
#[macro_export]
macro_rules! ld_arg {
    ($literal:literal) => {
        CILNode::LdArg($literal)
    };
}
#[test]
fn macro_test() {
    let sum = add!(
        zero_extend!(size_of!(usize), usize),
        zero_extend!(size_of!(crate::Type::Int(crate::Int::U8)), usize)
    );
    let mut asm = super::Assembly::default();
    sum(&mut asm);
}
/*


gen_binop! {div,  crate::BinOp::Div}
gen_binop! {div_un,  crate::BinOp::Div}
gen_binop! {rem,  crate::BinOp::Rem}
gen_binop! {rem_un,  crate::BinOp::RemUn}
 */
