use crate::{
    v2::{Assembly, ClassRef, ClassRefIdx, Int},
    FnSig, Type,
};
use serde::{Deserialize, Serialize};

use crate::IString;

/// Represenation of a target of a call.
#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Debug)]
pub struct CallSite {
    class: Option<ClassRefIdx>,
    name: IString,
    signature: FnSig,
    is_static: bool,
    generics: Vec<Type>,
}

impl CallSite {
    #[must_use]
    pub fn mstring_to_ptr(asm: &mut Assembly) -> Self {
        Self::new_extern(
            ClassRef::marshal(asm),
            "StringToCoTaskMemUTF8".into(),
            FnSig::new([Type::PlatformString], Type::Int(Int::ISize)),
            true,
        )
    }
    #[must_use]
    pub fn aligned_alloc(asm: &mut Assembly) -> Self {
        Self::new_extern(
            ClassRef::native_mem(asm),
            "AlignedAlloc".into(),
            FnSig::new(
                [Type::Int(Int::USize), Type::Int(Int::USize)],
                asm.nptr(Type::Void),
            ),
            true,
        )
    }
    #[must_use]
    pub fn alloc(asm: &mut Assembly) -> Self {
        Self::new_extern(
            ClassRef::marshal(asm),
            "AllocHGlobal".into(),
            FnSig::new([Type::Int(Int::I32)], Type::Int(Int::ISize)),
            true,
        )
    }
    #[must_use]
    pub fn realloc(asm: &mut Assembly) -> Self {
        Self::new(
            Some(ClassRef::native_mem(asm)),
            "AlignedRealloc".into(),
            FnSig::new(
                [
                    asm.nptr(Type::Void),
                    Type::Int(Int::USize),
                    Type::Int(Int::USize),
                ],
                asm.nptr(Type::Void),
            ),
            true,
        )
    }
    /// Constructs a new call site targeting method `name`, with signature `signature` and bleonging to class `class`. If `class` is [`None`], then the `<Module>` class
    /// is assumed.
    #[must_use]
    pub fn new(
        class: Option<ClassRefIdx>,
        name: IString,
        signature: FnSig,
        is_static: bool,
    ) -> Self {
        if *name == *".ctor" {
            assert!(!is_static);
        }
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
        class: ClassRefIdx,
        name: IString,
        signature: FnSig,
        is_static: bool,
    ) -> Self {
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
        class: Option<ClassRefIdx>,
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
    pub const fn class(&self) -> Option<ClassRefIdx> {
        self.class
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
    pub fn inputs(&self) -> &[Type] {
        self.signature.inputs()
    }
    /// Inputs, with the implicit `this` skipped if needed.
    #[must_use]
    pub fn explicit_inputs(&self) -> &[Type] {
        if self.is_static || self.inputs().is_empty() {
            self.signature.inputs()
        } else {
            &self.signature.inputs()[1..]
        }
    }
}
