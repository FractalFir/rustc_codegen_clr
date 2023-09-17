use serde::{Deserialize, Serialize};

use crate::{r#type::Type, IString};
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct TypeDef {
    inner_types: Vec<Self>,
    fields: Vec<(IString,Type)>,
    gargc:u32,
}
