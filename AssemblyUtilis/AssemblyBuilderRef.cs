namespace AssemblyUtilis;
using Mono.Cecil;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
[StructLayout(LayoutKind.Sequential)]
struct Version{
    public ushort major;
    public ushort minor;
    public ushort build;
    public ushort revision;
    public override string ToString() => $"Version{{major:{major},minor:{minor},build:{build},revision:{revision}}}";
}
class AssemblyDefinitionRef{
    [UnmanagedCallersOnly(EntryPoint = "new_assembly_def",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<AssemblyDefinition> NewAssemblyDefinition(RustString assemblyName,RustString moduleName,bool isLib,Version version){
        var system_version = new System.Version((int)version.major,(int)version.minor,(int)version.build,(int)version.revision);
        AssemblyNameDefinition aName = new AssemblyNameDefinition(assemblyName.ToString(),default(System.Version));
        var kind = ModuleKind.Console;
        if(isLib)kind = ModuleKind.Dll;
        AssemblyDefinition builder = AssemblyDefinition.CreateAssembly(aName,moduleName.ToString(),kind);
        return new ManagedHandle<AssemblyDefinition>(builder);
    }
    [UnmanagedCallersOnly(EntryPoint = "free_assembly_def")]
    public static void FreeAssemblyDefinition(ManagedHandle<AssemblyDefinition> asmBuilder){
        asmBuilder.FreeHandle();
    }
    [UnmanagedCallersOnly(EntryPoint = "serialize_assembly")]
    public static void SerializeAssembly(ManagedHandle<AssemblyDefinition> asm, RustString filePath){
        asm.GetRef().Write(filePath.ToString());
    }
    [UnmanagedCallersOnly(EntryPoint = "add_typedef")]
    public static void AddTypedef(ManagedHandle<AssemblyDefinition> asm, ManagedHandle<TypeDefinition> type){
        asm.GetRef().MainModule.Types.Add(type.GetRef());
    }
}

