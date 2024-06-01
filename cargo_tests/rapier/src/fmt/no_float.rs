use crate::fmt::{Debug, Formatter, Result};

macro_rules! floating {
    ($ty:ident) => {

        impl Debug for $ty {
            #[inline]
            fn fmt(&self, _fmt: &mut Formatter<'_>) -> Result {
                panic!("floating point support is turned off");
            }
        }
    };
}

floating! { f16 }
floating! { f32 }
floating! { f64 }
floating! { f128 }