use crate::{base_ir::BaseIR, types::Type};

pub(crate) mod aggregate;
pub(crate) mod array;
pub(crate) mod arthmetics;
pub(crate) mod convert;
pub(crate) mod entrypoint;
pub(crate) mod place;
pub(crate) fn sizeof_ops(_tpe: &Type) -> Vec<BaseIR> {
    todo!("Can't yet calculate size of things!");
}
#[derive(Clone, Copy)]
pub(crate) enum Aligement {
    A1,
    A2,
    A4,
    ANative,
    A8,
    //Unknown means "Any aligement under 8". Because of that max(A<4,Unknonwn) = Unknown. Since 8 is the largest aligement, max(A8,Unknown) is A8.
    Unknown,
}
impl Aligement {
    fn max(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::A8, _) => Self::A8,
            (_, Self::A8) => Self::A8,
            (Self::A1, _) => *other,
            (_, Self::A1) => *self,
            (Self::A2, Self::A2) => Self::A2,
            (Self::A4, Self::A4 | Self::A2) => Self::A4,
            (Self::A2, Self::A4) => Self::A4,
            (Self::ANative, Self::ANative | Self::A4 | Self::A2) => Self::ANative,
            (Self::A4 | Self::A2, Self::ANative) => Self::ANative,
            (Self::Unknown, Self::ANative | Self::A4 | Self::A2) => Self::Unknown,
            (Self::ANative | Self::A4 | Self::A2, Self::Unknown) => Self::Unknown,
            (Self::Unknown, Self::Unknown) => Self::Unknown,
        }
    }
}
fn align_ops(algiement: Aligement) -> BaseIR {
    match algiement {
        Aligement::A1 => BaseIR::LDConstI32(1),
        Aligement::A2 => BaseIR::LDConstI32(2),
        Aligement::A4 => BaseIR::LDConstI32(4),
        Aligement::A8 => BaseIR::LDConstI32(8),
        Aligement::ANative => BaseIR::SizeOf(Box::new(Type::ISize)),
        // TODO: Maybe we should panic if aligement is unknown??
        Aligement::Unknown => BaseIR::LDConstI32(8),
    }
}
fn align_of_tpe(tpe: &Type) -> Aligement {
    match tpe {
        Type::I8 | Type::U8 | Type::Bool | Type::Void => Aligement::A1,
        Type::I16 | Type::U16 => Aligement::A2,
        Type::F32 | Type::I32 | Type::U32 => Aligement::A4,
        Type::F64 | Type::I64 | Type::U64 | Type::I128 | Type::U128 => Aligement::A8,
        Type::ISize
        | Type::USize
        | Type::Ref(_)
        | Type::Ptr(_)
        | Type::Slice(_)
        | Type::StrSlice => Aligement::ANative,
        Type::Tuple(inner) => inner
            .iter()
            .map(align_of_tpe)
            .fold(Aligement::A1, |a, b| a.max(&b)),
        Type::Array { element, .. } => align_of_tpe(element),
        Type::Struct { fields, .. } => fields
            .iter()
            .map(|filed| align_of_tpe(&filed.tpe))
            .fold(Aligement::A1, |a, b| a.max(&b)),
        Type::Enum { variants, .. } => {
            assert!(
                variants.len() < 256,
                "TODO: handle enum discirmintator changing enum aligement!"
            );
            variants
                .iter()
                .map(|variant| {
                    variant
                        .fields
                        .iter()
                        .map(|filed| align_of_tpe(&filed.tpe))
                        .fold(Aligement::A1, |a, b| a.max(&b))
                })
                .fold(Aligement::A1, |a, b| a.max(&b))
        }
        Type::GenericParam { .. } => Aligement::Unknown,
        Type::ExternType { .. } => panic!("Can't calculate algiement of extern type!"),
        Type::EnumVariant { .. } => panic!("Can't calculate aligement of enum varaint, since it is a partial type."),
    }
}
pub(crate) fn align_of(tpe: Type) -> BaseIR {
    align_ops(align_of_tpe(&tpe))
}
