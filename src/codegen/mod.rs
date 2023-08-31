use crate::{types::Type, base_ir::BaseIR};

pub(crate) mod convert;
pub(crate) fn sizeof_ops(tpe:&Type)->Vec<BaseIR>{
    todo!("Can't yet calculate size of things!");
}
pub(crate) fn deref_ops(tpe:&Type)->Vec<BaseIR>{
    todo!("Can't yet get the deref ops of types!");
}