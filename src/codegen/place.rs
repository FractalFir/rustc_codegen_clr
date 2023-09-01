use crate::{base_ir::BaseIR, clr_method::LocalPlacement, statement::CodegenCtx, types::Type};
use rustc_middle::{
    mir::{Place as RPlace, PlaceElem},
    ty::TyCtxt,
};

pub(crate) fn place_getter_ops<'a>(place: &RPlace<'a>, ctx: &CodegenCtx<'a, '_>) -> Vec<BaseIR> {
    let place = Place::from(place, ctx);
    place_get_ops(&place)
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
fn place_get_ops(place: &Place) -> Vec<BaseIR> {
    let mut ops: Vec<_> = place
        .body
        .iter()
        .map(|proj| proj.body_ops())
        .flatten()
        .collect();
    ops.extend(place.head.get_ops());
    ops
}
// Translating Rust's palce to CLR ops is a not trivial task. For this reason, any place is split into 2 parts: "body" and "head".
// "Body" is the part that calculates the adress of the variable in question. It is handled in the same way for every operation done with a place.
// For any head, there is always a certain set of ops that will be consitenlty emmited.
// "Head" is the last element of the projection. This is the part that gets the value, addres or sets the varaible. What ops will it create? Depends on what you want to do with it.
struct Place {
    body: Box<[ProjectionElement]>,
    head: Box<ProjectionElement>,
}
impl Place {
    fn from<'a>(src: &RPlace<'a>, ctx: &CodegenCtx<'a, '_>) -> Self {
        let local = ctx.local_placement(src.local.as_u32());
        let mut projection: Vec<_> = src
            .projection
            .iter()
            .map(|e| ProjectionElement::from(&e, ctx))
            .collect();
        Self::new(local, projection)
    }
    fn new(local: LocalPlacement, projection: Vec<ProjectionElement>) -> Self {
        let local = ProjectionElement::Local(local);
        let mut body: Vec<_> = projection.into();
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
    Local(LocalPlacement),
    Deref,
    Field { index: u32, owner: Type },
    Index { local: LocalPlacement },
    ConstIndex { offset: u64, min_length: u64 },
    Subslice { from: u64, to: u64 },
}
impl ProjectionElement {
    fn from<'a>(value: &PlaceElem<'a>, ctx: &CodegenCtx<'a, '_>) -> Self {
        match value {
            PlaceElem::Deref => ProjectionElement::Deref,
            PlaceElem::Field(index, owner) => {
                let owner = Type::from_ty(owner, ctx.tyctx());
                Self::Field {
                    index: index.as_u32(),
                    owner,
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
    fn body_ops(&self) -> Vec<BaseIR> {
        match self {
            Self::Local(local) => vec![local_get(local)],
            _ => todo!("Unhandled projection element type:{self:?}"),
        }
    }
    /// Returns the ops necesary to get the value behind a [`ProjectionElement`] that is within `head` of a [`Place`].
    fn get_ops(&self) -> Vec<BaseIR> {
        match self {
            Self::Local(local) => vec![local_get(local)],
            _ => todo!("Unhandled projection element type:{self:?}"),
        }
    }
}
#[test]
fn trivial_get() {
    for i in 0..1000 {
        let place = Place::new(LocalPlacement::Arg(i), vec![]);
        let ops = place_get_ops(&place);
        assert_eq!(ops, [BaseIR::LDArg(i)])
    }
    for i in 0..1000 {
        let place = Place::new(LocalPlacement::Var(i), vec![]);
        let ops = place_get_ops(&place);
        assert_eq!(ops, [BaseIR::LDLoc(i)])
    }
}
/*
#[test]
fn field_get(){
    //let vec3 = Type::Struct { name: "vec3", fields: [Filed].into() }
    for i in 0..1000{
        let place = Place::new(LocalPlacement::Arg(i),vec![]);
        let ops = place_get_ops(&place);
        assert_eq!(ops,[BaseIR::LDArg(i)])
    }
    for i in 0..1000{
        let place = Place::new(LocalPlacement::Var(i),vec![]);
        let ops = place_get_ops(&place);
        assert_eq!(ops,[BaseIR::LDLoc(i)])
    }
}*/
