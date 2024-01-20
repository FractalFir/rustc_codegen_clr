use rustc_target::abi::Layout;
use rustc_target::abi::FieldIdx;
use rustc_target::abi::FieldsShape;
use rustc_target::abi::Variants;
use rustc_middle::ty::TyCtxt;
use rustc_target::abi::Size;
use crate::r#type::Type;
pub(crate) enum FieldOffsetIterator{
    Explicit{
        offsets:Box<[u32]>,
        index:usize,
    },
    NoOffset{
        count:u64,
    }
}
impl Iterator for FieldOffsetIterator{
    type Item = u32;
    fn next(&mut self)->Option<u32>{
        match self{
            Self::Explicit{offsets,index}=>{
                let next = offsets.get(*index);
                *index += 1;
                next.copied()
            }
            Self::NoOffset{count}=>{
                if *count > 0 {
                    *count -= 1;
                    Some(0)
                }
                else{
                    None
                }
            }
        }
       
    }
}
impl FieldOffsetIterator{
    pub fn fields(parent:&Layout)->FieldOffsetIterator{
        match parent.fields(){
            FieldsShape::Arbitrary{offsets,memory_index}=>{
                let offsets:Box<[_]> = memory_index.iter().map(|index|offsets[FieldIdx::from_u32(*index)].bytes() as u32).collect();
                FieldOffsetIterator::Explicit{offsets,index:0}
            },
            FieldsShape::Union(count)=>FieldOffsetIterator::NoOffset{count:Into::<usize>::into(*count) as u64},
            _=>todo!()
        }
    }
}
/// Takes layout of an enum as input, and returns the type of its tag(Void if no tag) and the size of the tag(0 if no tag).
pub fn enum_tag_info<'tyctx>(r#enum:&Layout<'tyctx>,tyctx:TyCtxt<'tyctx>)->(Type,u32){
    match r#enum.variants(){
        Variants::Single{..}=>(Type::Void,0),
        Variants::Multiple{tag,..}=>(scalr_to_type(*tag),tag.size(&tyctx).bytes() as u32),
    }
}
fn scalr_to_type(scalar:rustc_target::abi::Scalar)->Type{
    let primitive = match scalar {rustc_target::abi::Scalar::Union{value}=>value,rustc_target::abi::Scalar::Initialized{value,..}=>value};
    primitive_to_type(primitive)
}
fn primitive_to_type(primitive:rustc_target::abi::Primitive)->Type{
    use rustc_target::abi::Primitive;
    use  rustc_target::abi::Integer;
    match primitive{
        Primitive::Int(int,sign)=>match (int,sign){
            (Integer::I8,true)=>Type::I8,
            (Integer::I16,true)=>Type::I16,
            (Integer::I32,true)=>Type::I32,
            (Integer::I64,true)=>Type::I64,
            (Integer::I128,true)=>Type::I128,
            (Integer::I8,false)=>Type::U8,
            (Integer::I16,false)=>Type::U16,
            (Integer::I32,false)=>Type::U32,
            (Integer::I64,false)=>Type::U64,
            (Integer::I128,false)=>Type::U128,
        }
        Primitive::F32=>Type::F32,
        Primitive::F64=>Type::F64,
        Primitive::Pointer(_)=>Type::Ptr(Type::Void.into()),
    }
}