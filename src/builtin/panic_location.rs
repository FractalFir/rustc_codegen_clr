// TODO: Can't yet register thread-local deconstructors.
add_method_from_trees!(
    push_panic_location,
    [Type::Int(Int::ISize),],
    Type::Void,
    vec![BasicBlock::new(vec![CILRoot::VoidRet.into()], 0, None)],
    vec![],
    vec![
        Some("dtor".into()),
        Some("obj".into()),
        Some("dso_handle".into())
    ]
);
/*
using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;

class ThreadLocalDemo
{
        static ThreadLocal<List<IntPtr>> panicLocationStack = new ThreadLocal<List<IntPtr>>(()=>new List<IntPtr>(64));
        static void Main()
        {

        }
        static void PushPanicLocation(IntPtr ptr){
            panicLocationStack.Value.Add(ptr);
        }
    static void PopPanicLocation(IntPtr ptr){
            List<IntPtr> ploc = panicLocationStack.Value;
            ploc.RemoveRange(ploc.IndexOf(ptr),ploc.Count);
        }
}

*/
