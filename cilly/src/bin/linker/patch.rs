pub fn patch_all(asm: &mut cilly::asm::Assembly) {
    let _ = asm;
    //println!("Applying patches to the resulting assembly...");
    //hijack_arg_init(asm);
}
/*
public class C {
    static unsafe void really_init(char** argv,int argc){

    }
    public unsafe void M() {
        string[] args = Environment.GetCommandLineArgs();
        string exec_path = System.Reflection.Assembly.GetEntryAssembly().Location;
        int argc = args.Length + 1;
        char** argv = (char**)System.Runtime.InteropServices.NativeMemory.AlignedAlloc((nuint)(argc*sizeof(char*)),8);
        for(int idx = 0; idx < args.Length; idx++){
            argv[idx] = (char*)System.Runtime.InteropServices.Marshal.StringToCoTaskMemUTF8(args[idx]);
        }
        really_init(argv,argc);
    }
}
*/
