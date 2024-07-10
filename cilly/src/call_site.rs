use crate::{DotnetTypeRef, FnSig, Type};
use serde::{Deserialize, Serialize};

use crate::IString;

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
    pub fn mcheck() -> Self {
        Self::builtin("mcheck".into(), FnSig::new(&[Type::ISize], Type::I32), true)
    }
    #[must_use]
    pub fn mcheck_check_all() -> Self {
        Self::builtin("mcheck_check_all".into(), FnSig::new(&[], Type::Void), true)
    }
    #[must_use]
    pub fn mstring_to_ptr() -> Self {
        Self::new_extern(
            DotnetTypeRef::marshal(),
            "StringToCoTaskMemUTF8".into(),
            FnSig::new(&[DotnetTypeRef::string_type().into()], Type::ISize),
            true,
        )
    }
    #[must_use]
    pub fn alloc() -> Self {
        Self::new_extern(
            DotnetTypeRef::native_mem(),
            "AlignedAlloc".into(),
            FnSig::new(&[Type::USize, Type::USize], Type::Ptr(Type::Void.into())),
            true,
        )
    }
    #[must_use]
    pub fn realloc() -> Self {
        Self::new(
            Some(DotnetTypeRef::native_mem()),
            "AlignedRealloc".into(),
            FnSig::new(
                &[Type::Ptr(Type::Void.into()), Type::USize, Type::USize],
                Type::Ptr(Type::Void.into()),
            ),
            true,
        )
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
    pub const fn signature(&self) -> &FnSig {
        &self.signature
    }

    /// Returns the class the targeted method belongs to.
    #[must_use]
    pub const fn class(&self) -> Option<&DotnetTypeRef> {
        self.class.as_ref()
    }
    /// Returns `true` if the method in question is static.
    #[must_use]
    pub const fn is_static(&self) -> bool {
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
        if AsRef::<str>::as_ref(&self.name) != "black_box"
            && AsRef::<str>::as_ref(&self.name) != "assert_inhabited"
        {
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
