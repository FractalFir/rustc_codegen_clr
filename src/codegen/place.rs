use crate::{base_ir::{BaseIR, FiledDescriptor}, clr_method::LocalPlacement, statement::CodegenCtx, types::Type};
use rustc_middle::mir::{Place as RPlace, PlaceElem};

pub(crate) fn place_getter_ops<'a>(place: &RPlace<'a>, ctx: &CodegenCtx<'a, '_>) -> Vec<BaseIR> {
    let place = Place::from(place, ctx);
    place_get_ops(&place)
}
pub(crate) fn place_setter_ops<'a>(
    place: &RPlace<'a>,
    ctx: &CodegenCtx<'a, '_>,
    value_calc: Vec<BaseIR>,
) -> Vec<BaseIR> {
    let place = Place::from(place, ctx);
    place_set_ops(&place, value_calc)
}
fn local_get(local: &LocalPlacement) -> BaseIR {
    match local {
        LocalPlacement::Arg(arg_num) => BaseIR::LDArg(*arg_num),
        LocalPlacement::Var(arg_num) => BaseIR::LDLoc(*arg_num),
    }
}
fn local_set(local: &LocalPlacement) -> BaseIR {
    match local {
        LocalPlacement::Arg(arg_num) => BaseIR::STArg(*arg_num),
        LocalPlacement::Var(arg_num) => BaseIR::STLoc(*arg_num),
    }
}
fn local_adress(local: &LocalPlacement) -> BaseIR {
    match local {
        LocalPlacement::Arg(arg_num) => BaseIR::LDArgA(*arg_num),
        LocalPlacement::Var(arg_num) => BaseIR::LDLocA(*arg_num),
    }
}
pub(crate) fn place_get_ops(place: &Place) -> Vec<BaseIR> {
    let mut ops: Vec<_> = Vec::with_capacity(place.body.len() * 2);
    // Since first element must be a local, first element does not use the type. It then changes the type to value other than [`Void`]. This makes catching bugs easier - if the
    // first element is not `Local`, this will instanlty panic.
    let mut tpe = Type::Void;
    for segement in &*place.body {
        let (sops, stpe) = segement.body_ops(&tpe);
        ops.extend(sops);
        tpe = stpe;
    }
    ops.extend(place.head.get_ops(tpe));
    ops
}
pub(crate) fn place_adress_ops(place: &Place) -> Vec<BaseIR> {
    let mut ops: Vec<_> = Vec::with_capacity(place.body.len() * 2);
    // Since first element must be a local, first element does not use the type. It then changes the type to value other than [`Void`]. This makes catching bugs easier - if the
    // first element is not `Local`, this will instanlty panic.
    let mut tpe = Type::Void;
    for segement in &*place.body {
        let (sops, stpe) = segement.body_ops(&tpe);
        ops.extend(sops);
        tpe = stpe;
    }
    ops.extend(place.head.adress_ops(tpe));
    ops
}
pub(crate) fn place_set_ops(place: &Place, value_calc: Vec<BaseIR>) -> Vec<BaseIR> {
    let mut ops: Vec<_> = Vec::with_capacity(place.body.len() * 2);
    // Since first element must be a local, first element does not use the type. It then changes the type to value other than [`Void`]. This makes catching bugs easier - if the
    // first element is not `Local`, this will instanlty panic.
    let mut tpe = Type::Void;
    for segement in &*place.body {
        let (sops, stpe) = segement.body_ops(&tpe);
        ops.extend(sops);
        tpe = stpe;
    }
    ops.extend(value_calc);
    ops.extend(place.head.set_ops(tpe));
    ops
}
// Translating Rust's palce to CLR ops is a not trivial task. For this reason, any place is split into 2 parts: "body" and "head".
// "Body" is the part that calculates the adress of the variable in question. It is handled in the same way for every operation done with a place.
// For any head, there is always a certain set of ops that will be consitenlty emmited.
// "Head" is the last element of the projection. This is the part that gets the value, addres or sets the varaible. What ops will it create? Depends on what you want to do with it.
pub(crate) struct Place {
    body: Box<[ProjectionElement]>,
    head: Box<ProjectionElement>,
}
impl Place {
    pub(crate) fn from<'a>(src: &RPlace<'a>, ctx: &CodegenCtx<'a, '_>) -> Self {
        let local = (
            ctx.local_placement(src.local.as_u32()),
            ctx.get_local_type(src.local.as_u32()),
        );
        let projection: Vec<_> = src
            .projection
            .iter()
            .map(|e| ProjectionElement::from(&e, ctx))
            .collect();
        Self::new(local, projection)
    }
    fn new(local: (LocalPlacement, &Type), projection: Vec<ProjectionElement>) -> Self {
        let local = ProjectionElement::Local {
            local: local.0,
            tpe: local.1.clone(),
        };
        let mut body: Vec<_> = projection;
        body.insert(0, local);
        //Since we have inserted into body, it must have at least 1 element in it.
        let head = body.pop().expect(
            "Impossible condition reached: could not pop from a vector with at least 1 element!",
        );
        Self {
            head: Box::new(head),
            body: body.into(),
        }
    }
}
#[derive(Debug, Clone)]
enum ProjectionElement {
    Local { local: LocalPlacement, tpe: Type },
    Deref,
    Field { index: u32, /*field_type: Type*/ },
    Index { local: LocalPlacement },
    ConstIndex { offset: u64, min_length: u64 },
    Subslice { from: u64, to: u64 },
}
impl ProjectionElement {
    fn from<'a>(value: &PlaceElem<'a>, ctx: &CodegenCtx<'a, '_>) -> Self {
        match value {
            PlaceElem::Deref => ProjectionElement::Deref,
            PlaceElem::Field(index, field_type) => {
                //let field_type = Type::from_ty(field_type, ctx.tyctx());
                Self::Field {
                    index: index.as_u32(),
                    //field_type,
                }
            }
            PlaceElem::Index(local) => Self::Index {
                local: ctx.local_placement(local.as_u32()),
            },
            PlaceElem::ConstantIndex {
                offset,
                min_length,
                from_end,
            } => {
                assert!(
                    !from_end,
                    "Can only handle constant indexing from the start, not from the end."
                );
                Self::ConstIndex {
                    offset: *offset,
                    min_length: *min_length,
                }
            }
            PlaceElem::Subslice { from, to, from_end } => {
                assert!(
                    !from_end,
                    "Can only handle slicing from the start, not from the end."
                );
                Self::Subslice {
                    from: *from,
                    to: *to,
                }
            }
            PlaceElem::OpaqueCast(_) => todo!("Can't handle casts in place!"),
            PlaceElem::Downcast(_, _) => todo!("Can't get the variant of an enum!"),
        }
    }
    /// Returns the ops necesary to handle a [`ProjectionElement`] that is within `body` of a [`Place`].
    fn body_ops(&self, tpe: &Type) -> (Vec<BaseIR>, Type) {
        match self {
            Self::Local { local, tpe } => (vec![local_get(local)], tpe.clone()),
            Self::Field{index} =>{
                let variant = 0;
                let field = tpe.field(variant, *index);
                match field.tpe{
                    Type::Struct { .. }=>(vec![BaseIR::LDFieldAdress(Box::new(FiledDescriptor{ owner:tpe.clone(), variant, field_index: *index }))],field.tpe.clone()),
                    _=>(vec![BaseIR::LDField(Box::new(FiledDescriptor{ owner:tpe.clone(), variant, field_index: *index }))],field.tpe.clone())
                }
            }
            Self::Deref => {
                let pointed = tpe
                    .pointed_type()
                    .expect("Tried to deref a non-pointer type!");
                let ops = body_deref_ops(pointed);
                (ops, pointed.clone())
            }
            //Self::Index{ }
            _ => todo!("Unhandled projection element type:{self:?}"),
        }
    }
    /// Returns the ops necesary to get the value behind a [`ProjectionElement`] that is within `head` of a [`Place`].
    fn get_ops(&self, tpe: Type) -> Vec<BaseIR> {
        match self {
            Self::Local { local, .. } => vec![local_get(local)],
            Self::Deref => {
                let pointed = tpe
                    .pointed_type()
                    .expect("Tried to deref a non-pointer type!");

                getter_deref_ops(pointed)
            }
            Self::Field{index} =>{
                let variant = 0;
                let field = tpe.field(variant, *index);
                vec![BaseIR::LDField(Box::new(FiledDescriptor{ owner:tpe.clone(), variant, field_index: *index }))]
            }
            _ => todo!("Unhandled projection element type:{self:?}"),
        }
    }
    /// Returns the ops necesary to set the value behind a [`ProjectionElement`] that is within `head` of a [`Place`].
    fn set_ops(&self, tpe: Type) -> Vec<BaseIR> {
        match self {
            Self::Local { local, .. } => vec![local_set(local)],
            Self::Deref => {
                let pointed = tpe
                    .pointed_type()
                    .expect("Tried to deref a non-pointer type!");

                setter_deref_ops(pointed)
            }
            Self::Field{index} =>{
                let variant = 0;
                let field = tpe.field(variant, *index);
                vec![BaseIR::STField(Box::new(FiledDescriptor{ owner:tpe.clone(), variant, field_index: *index }))]
            }
            _ => todo!("Unhandled projection element type:{self:?}"),
        }
    }
    /// Returns the ops necesary to get the value behind a [`ProjectionElement`] that is within `head` of a [`Place`].
    fn adress_ops(&self, tpe: Type) -> Vec<BaseIR> {
        match self {
            Self::Local { local, .. } => vec![local_adress(local)],
            Self::Deref => vec![BaseIR::Nop],
            Self::Field{index} =>{
                let variant = 0;
                let field = tpe.field(variant, *index);
                vec![BaseIR::LDFieldAdress(Box::new(FiledDescriptor{ owner:tpe.clone(), variant, field_index: *index }))]
            }
            _ => todo!("Unhandled projection element type:{self:?}"),
        }
    }
}
fn body_deref_ops(pointed: &Type) -> Vec<BaseIR> {
    match pointed {
        Type::I8 | Type::U8 => vec![BaseIR::LDIndIn(1)],
        Type::I16 | Type::U16 => vec![BaseIR::LDIndIn(2)],
        Type::I32 | Type::U32 => vec![BaseIR::LDIndIn(4)],
        Type::I64 | Type::U64 => vec![BaseIR::LDIndIn(8)],
        Type::Ref(_) | Type::Ptr(_) | Type::USize | Type::ISize => vec![BaseIR::LDIndI],
        Type::F32 => vec![BaseIR::LDIndR4],
        Type::F64 => vec![BaseIR::LDIndR8],
        // In body, struct types should never be derferenced.
        Type::Struct { .. } | Type::Array{..} => vec![BaseIR::Nop],
        _ => todo!("unsuported adress derf: {pointed:?}"),
    }
}
fn getter_deref_ops(pointed: &Type) -> Vec<BaseIR> {
    match pointed{
        Type::Struct { .. } | Type::Array{..} => vec![BaseIR::LDObj(pointed.clone())],
        _=>body_deref_ops(pointed)
    }
    
}
fn setter_deref_ops(pointed: &Type) -> Vec<BaseIR> {
    match pointed {
        Type::I8 | Type::U8 => vec![BaseIR::STIndIn(1)],
        Type::I16 | Type::U16 => vec![BaseIR::STIndIn(2)],
        Type::I32 | Type::U32 => vec![BaseIR::STIndIn(4)],
        Type::I64 | Type::U64 => vec![BaseIR::STIndIn(8)],
        Type::Ref(_) | Type::Ptr(_) | Type::USize | Type::ISize => vec![BaseIR::STIndI],
        Type::F32 => vec![BaseIR::STIndR4],
        Type::F64 => vec![BaseIR::STIndR8],
        Type::Struct{..} | Type::Array{..} => vec![BaseIR::STObj(pointed.clone())],
        _ => todo!("unsuported set derf: {pointed:?}"),
    }
}
#[test]
fn trivial_get() {
    for i in 0..1000 {
        let place = Place::new((LocalPlacement::Arg(i), &Type::I32), vec![]);
        let ops = place_get_ops(&place);
        assert_eq!(ops, [BaseIR::LDArg(i)]);
    }
    for i in 0..1000 {
        let place = Place::new((LocalPlacement::Var(i), &Type::I32), vec![]);
        let ops = place_get_ops(&place);
        assert_eq!(ops, [BaseIR::LDLoc(i)]);
    }
}
#[test]
fn deref() {
    //Deref I32
    let place = Place::new(
        (LocalPlacement::Arg(0), &Type::Ref(Box::new(Type::I32))),
        vec![ProjectionElement::Deref],
    );
    let ops = place_get_ops(&place);
    assert_eq!(ops, [BaseIR::LDArg(0), BaseIR::LDIndIn(4)]);
    //Deref I64
    let place = Place::new(
        (LocalPlacement::Arg(0), &Type::Ref(Box::new(Type::I64))),
        vec![ProjectionElement::Deref],
    );
    let ops = place_get_ops(&place);
    assert_eq!(ops, [BaseIR::LDArg(0), BaseIR::LDIndIn(8)]);
    //Deref U8
    let place = Place::new(
        (LocalPlacement::Arg(0), &Type::Ref(Box::new(Type::U8))),
        vec![ProjectionElement::Deref],
    );
    let ops = place_get_ops(&place);
    assert_eq!(ops, [BaseIR::LDArg(0), BaseIR::LDIndIn(1)]);
    //Deref Usize
    let place = Place::new(
        (LocalPlacement::Arg(0), &Type::Ref(Box::new(Type::USize))),
        vec![ProjectionElement::Deref],
    );
    let ops = place_get_ops(&place);
    assert_eq!(ops, [BaseIR::LDArg(0), BaseIR::LDIndI]);
    //Deref Ref()
    let place = Place::new(
        (
            LocalPlacement::Arg(0),
            &Type::Ref(Box::new(Type::Ref(Box::new(Type::USize)))),
        ),
        vec![ProjectionElement::Deref],
    );
    let ops = place_get_ops(&place);
    assert_eq!(ops, [BaseIR::LDArg(0), BaseIR::LDIndI]);
    //Deref 2 layers
    let place = Place::new(
        (
            LocalPlacement::Arg(0),
            &Type::Ref(Box::new(Type::Ref(Box::new(Type::U8)))),
        ),
        vec![ProjectionElement::Deref, ProjectionElement::Deref],
    );
    let ops = place_get_ops(&place);
    assert_eq!(ops, [BaseIR::LDArg(0), BaseIR::LDIndI, BaseIR::LDIndIn(1)]);
    //Deref F32
    let place = Place::new(
        (LocalPlacement::Arg(0), &Type::Ref(Box::new(Type::F32))),
        vec![ProjectionElement::Deref],
    );
    let ops = place_get_ops(&place);
    assert_eq!(ops, [BaseIR::LDArg(0), BaseIR::LDIndR4]);
    //Deref F64
    let place = Place::new(
        (LocalPlacement::Arg(0), &Type::Ref(Box::new(Type::F64))),
        vec![ProjectionElement::Deref],
    );
    let ops = place_get_ops(&place);
    assert_eq!(ops, [BaseIR::LDArg(0), BaseIR::LDIndR8]);
    let place = Place::new(
        (LocalPlacement::Arg(0), &Type::Ref(Box::new(Type::F32))),
        vec![ProjectionElement::Deref],
    );
    let ops = place_set_ops(&place, vec![BaseIR::LDConstF32(0.123)]);
    assert_eq!(
        ops,
        [BaseIR::LDArg(0), BaseIR::LDConstF32(0.123), BaseIR::STIndR4]
    );
}
