use serde::{Deserialize, Serialize};

use crate::{
    function_sig::FnSig,
    r#type::{DotnetTypeRef, Type},
    IString,
};
use rustc_middle::ty::TyCtxt;
/// Represenation of a target of a call.
#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Debug)]
pub struct CallSite {
    class: Option<DotnetTypeRef>,
    name: IString,
    signature: FnSig,
    is_static: bool,
    generics: Vec<Type>,
}
impl CallSite {
    /// Retruns a call site reffering to void* Unsafe.AsPtr<element>(ref element)
    pub fn ref_as_ptr(element: Type) -> Self {
        let unsafe_services = DotnetTypeRef::compiler_services_unsafe();
        let mut as_pointer = CallSite::new(
            unsafe_services.into(),
            "AsPointer".into(),
            FnSig::new(
                &[Type::ManagedReference(Type::CallGenericArg(0).into())],
                &Type::Ptr(Type::Void.into()),
            ),
            true,
        );
        as_pointer.set_generics(vec![element.clone()]);
        as_pointer
    }
    /// Constructs a new call site targeting method `name`, with signature `signature` and bleonging to class `class`. If `class` is [`None`], then the `<Module>` class
    /// is assumed.
    pub fn new(
        class: Option<DotnetTypeRef>,
        name: IString,
        signature: FnSig,
        is_static: bool,
    ) -> Self {
        Self {
            class,
            name,
            signature,
            is_static,
            generics: vec![],
        }
    }
    pub fn generics(&self) -> &[Type] {
        &self.generics
    }
    pub fn set_generics(&mut self, generics: Vec<Type>) {
        self.generics = generics;
    }
    /// The same as [`Self::new`], but boxes the result.
    pub fn boxed(
        class: Option<DotnetTypeRef>,
        name: IString,
        signature: FnSig,
        is_static: bool,
    ) -> Box<Self> {
        Box::new(Self::new(class, name, signature, is_static))
    }
    /// Returns the signature of the function this call site targets.
    pub fn signature(&self) -> &FnSig {
        &self.signature
    }
    /// Returns the call site refering to the function malloc.
    pub fn malloc(ctx: TyCtxt) -> Self {
        Self::new(
            None,
            "malloc".into(),
            FnSig::new(&[Type::USize], &Type::Ptr(Type::c_void(ctx).into())),
            true,
        )
    }
    /// Returns the class the targeted method belongs to.
    pub fn class(&self) -> Option<&DotnetTypeRef> {
        self.class.as_ref()
    }
    /// Returns `true` if the method in question is static.
    pub fn is_static(&self) -> bool {
        self.is_static
    }
    /// Returns the name of the targteted method.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns true if a call is equivalent to a No-Op. Used to handle black_box.
    pub fn is_nop(&self) -> bool {
        if !self.is_static() {
            return false;
        }
        if self.class().is_some() {
            return false;
        };
        if self.name.as_ref() != "black_box" && self.name.as_ref() != "assert_inhabited" {
            return false;
        };
        if self.signature.inputs().len() != 1 {
            return false;
        };
        if self.signature.inputs()[0] != *self.signature.output() {
            return false;
        };
        true
    }
    /// All inputs. Includes impilcit `this` argument for instance functions.
    pub fn inputs(&self) -> &[crate::r#type::Type] {
        self.signature.inputs()
    }
    /// Inputs, with the implicit `this` skipped if needed.
    pub fn explicit_inputs(&self) -> &[crate::r#type::Type] {
        if self.is_static || self.inputs().is_empty() {
            self.signature.inputs()
        } else {
            &self.signature.inputs()[1..]
        }
    }
}
