struct Allocation{
    allocator:[CILOp;4],
    initializer:Box<[CILOp]>,
}
struct Allocations{
    HashMap<u32,Allocation>,
}
impl Allocations{
    fn finalize(&self,asm:&Assembly)->Method{
        let mut method = Method::new();
        for (id,allocation) in self.allocations{
            
        }
    }
}
