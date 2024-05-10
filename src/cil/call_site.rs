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
    #[must_use]
    pub fn mstring_to_ptr() -> Self {
        CallSite::new_extern(
            DotnetTypeRef::marshal(),
            "StringToCoTaskMemUTF8".into(),
            FnSig::new(&[DotnetTypeRef::string_type().into()], &Type::ISize),
            true,
        )
    }
    #[must_use]
    pub fn alloc() -> Self {
        CallSite::new_extern(
            DotnetTypeRef::native_mem(),
            "AlignedAlloc".into(),
            FnSig::new(&[Type::USize, Type::USize], &Type::Ptr(Type::Void.into())),
            true,
        )
    }
    #[must_use]
    pub fn realloc() -> Self {
        CallSite::new(
            Some(DotnetTypeRef::native_mem()),
            "AlignedRealloc".into(),
            FnSig::new(
                &[Type::Ptr(Type::Void.into()), Type::USize, Type::USize],
                &Type::Ptr(Type::Void.into()),
            ),
            true,
        )
    }
    /// Retruns a call site reffering to void* Unsafe.AsPtr<element>(ref element)
    #[must_use]
    pub fn ref_as_ptr(element: Type) -> Self {
        let unsafe_services = DotnetTypeRef::compiler_services_unsafe();
        let mut as_pointer = CallSite::new_extern(
            unsafe_services,
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
    #[must_use]
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
    #[must_use]
    pub fn new_extern(
        class: DotnetTypeRef,
        name: IString,
        signature: FnSig,
        is_static: bool,
    ) -> Self {
        debug_assert!(class.asm().is_some());
        Self {
            class: Some(class),
            name,
            signature,
            is_static,
            generics: vec![],
        }
    }
    #[must_use]
    pub fn builtin(name: IString, signature: FnSig, is_static: bool) -> Self {
        Self {
            class: None,
            name,
            signature,
            is_static,
            generics: vec![],
        }
    }
    #[must_use]
    pub fn generics(&self) -> &[Type] {
        &self.generics
    }
    pub fn set_generics(&mut self, generics: Vec<Type>) {
        self.generics = generics;
    }
    /// The same as [`Self::new`], but boxes the result.
    #[must_use]
    pub fn boxed(
        class: Option<DotnetTypeRef>,
        name: IString,
        signature: FnSig,
        is_static: bool,
    ) -> Box<Self> {
        Box::new(Self::new(class, name, signature, is_static))
    }
    /// Returns the signature of the function this call site targets.
    #[must_use]
    pub fn signature(&self) -> &FnSig {
        &self.signature
    }
    /// Returns the call site refering to the function malloc.
    #[must_use]
    pub fn malloc(ctx: TyCtxt) -> Self {
        Self::new(
            None,
            "malloc".into(),
            FnSig::new(&[Type::USize], &Type::Ptr(Type::c_void(ctx).into())),
            true,
        )
    }
    /// Returns the class the targeted method belongs to.
    #[must_use]
    pub fn class(&self) -> Option<&DotnetTypeRef> {
        self.class.as_ref()
    }
    /// Returns `true` if the method in question is static.
    #[must_use]
    pub fn is_static(&self) -> bool {
        self.is_static
    }
    /// Returns the name of the targteted method.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns true if a call is equivalent to a No-Op. Used to handle `black_box`.
    #[must_use]
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
    #[must_use]
    pub fn inputs(&self) -> &[crate::r#type::Type] {
        self.signature.inputs()
    }
    /// Inputs, with the implicit `this` skipped if needed.
    #[must_use]
    pub fn explicit_inputs(&self) -> &[crate::r#type::Type] {
        if self.is_static || self.inputs().is_empty() {
            self.signature.inputs()
        } else {
            &self.signature.inputs()[1..]
        }
    }
}
