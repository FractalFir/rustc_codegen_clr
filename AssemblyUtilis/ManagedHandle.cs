namespace AssemblyUtilis;
using System.Runtime.InteropServices;
[StructLayout(LayoutKind.Sequential)]
public struct ManagedHandle<T>
where T : class
{
    int refID;
    //private ManagedHandle(int refID)=>this.refID = refID;
    static List<T> ts = new List<T>{null};
    static List<int> free = new List<int>();
    public ManagedHandle(T reference){
        lock(ts){
            if(free.Count > 1){
                var last = free[free.Count - 1];
                free.RemoveAt(free.Count - 1);
                ts[last] = reference;
                this.refID = last;
            }
            else{
                int lastIndex = ts.Count();
                ts.Add(reference);
                this.refID = lastIndex;
            }
        }
    }
    public void FreeHandle(){
        lock(ts){
            ts[refID] = null;
            if(refID == ts.Count - 1){
                ts.RemoveAt(refID);
            }
            else{
                free.Add(refID);
            }
        }
    }
    public T GetRef()=>ts[refID];
    public override string ToString()=>$"handle:{refID}";
}
