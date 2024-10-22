use super::{Assembly, MethodDef};

struct MethodBuilder<'asm> {
    asm: &'asm mut Assembly,
    def: MethodDef,
}
impl<'asm> MethodBuilder<'asm> {}
