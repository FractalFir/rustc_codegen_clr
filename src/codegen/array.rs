use crate::{
    base_ir::{BaseIR, CallSite},
    types::Type,
};

fn array_get(array_type: Type, mut index: Vec<BaseIR>) -> Vec<BaseIR> {
    let signature = crate::FunctionSignature::new(
        &[Type::ISize],
        array_type
            .element_type()
            .expect("Tried to index non-array type!"),
    );
    index.extend([BaseIR::Call(Box::new(CallSite {
        owner: Some(array_type),
        name: "get_Item".into(),
        signature,
        is_static: false,
    }))]);
    index
}
pub(crate) fn array_set(array_type: Type, mut index: Vec<BaseIR>) -> Vec<BaseIR> {
    let signature = crate::FunctionSignature::new(
        &[
            Type::ISize,
            array_type
                .element_type()
                .expect("Tried to index non-array type!")
                .clone(),
        ],
        &Type::Void,
    );
    index.extend([BaseIR::Call(Box::new(CallSite {
        owner: Some(array_type),
        name: "set_Item".into(),
        signature,
        is_static: false,
    }))]);
    index
}
