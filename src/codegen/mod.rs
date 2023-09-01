use crate::{base_ir::BaseIR, types::Type};

pub(crate) mod convert;
pub(crate) mod place;
pub(crate) fn sizeof_ops(tpe: &Type) -> Vec<BaseIR> {
    todo!("Can't yet calculate size of things!");
}
pub(crate) fn deref_ops(tpe: &Type) -> Vec<BaseIR> {
    todo!("Can't yet get the deref ops of types!");
}
