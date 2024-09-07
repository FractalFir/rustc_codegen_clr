namespace AssemblyUtilis;
using Mono.Cecil;
using Mono.Cecil.Rocks;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
public class ClassDef
{
    [UnmanagedCallersOnly(EntryPoint = "new_type_def",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<ClassDefinition> NewClassDef(RustString nameSpace,RustString name,TypeAttributes attributes){
        var typeDef = new ClassDefinition(nameSpace.ToString(),name.ToString(),attributes);
        return new ManagedHandle<ClassDefinition>(typeDef);
    }
    [UnmanagedCallersOnly(EntryPoint = "new_field_def",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<FieldDefinition> NewFieldDef(RustString fieldName,FieldAttributes attributes,ManagedHandle<TypeReference> fieldType,bool hasOffset,uint offset){
        var fieldDef = new FieldDefinition(fieldName.ToString(),attributes,fieldType.GetRef());
        if(hasOffset){
            fieldDef.Offset = (int)offset; 
        }
        return new ManagedHandle<FieldDefinition>(fieldDef);
    }
    [UnmanagedCallersOnly(EntryPoint = "add_field_def",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static void AddFieldDef(ManagedHandle<ClassDefinition> typeDef,ManagedHandle<FieldDefinition> fieldDef){
        typeDef.GetRef().Fields.Add(fieldDef.GetRef());
    }
    [UnmanagedCallersOnly(EntryPoint = "new_type_ref",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<TypeReference> NewTypeRef(RustString nameSpace,RustString name,bool isValueType){
        var tref = new TypeReference(nameSpace.ToString(),name.ToString(), null, null);
        tref.IsValueType = isValueType;
        return new ManagedHandle<TypeReference>(tref);
    }
    [UnmanagedCallersOnly(EntryPoint = "add_inner_type",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static void AddInnerType(ManagedHandle<ClassDefinition> typeDef,ManagedHandle<ClassDefinition> inner){
        typeDef.GetRef().NestedTypes.Add(inner.GetRef());
    }
    // Void
    [UnmanagedCallersOnly(EntryPoint = "void",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<TypeReference> Void(ManagedHandle<AssemblyDefinition> asm)=>new ManagedHandle<TypeReference>(asm.GetRef().MainModule.ImportReference(typeof(void)));
    // Bool 
    [UnmanagedCallersOnly(EntryPoint = "bool",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<TypeReference> Bool(ManagedHandle<AssemblyDefinition> asm)=>new ManagedHandle<TypeReference>(asm.GetRef().MainModule.ImportReference(typeof(bool)));
    // int8 
    [UnmanagedCallersOnly(EntryPoint = "int8",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<TypeReference> Int8(ManagedHandle<AssemblyDefinition> asm)=>new ManagedHandle<TypeReference>(asm.GetRef().MainModule.ImportReference(typeof(sbyte)));
    // int16 
    [UnmanagedCallersOnly(EntryPoint = "int16",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<TypeReference> Int16(ManagedHandle<AssemblyDefinition> asm)=>new ManagedHandle<TypeReference>(asm.GetRef().MainModule.ImportReference(typeof(short)));
    // int32 
    [UnmanagedCallersOnly(EntryPoint = "int32",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<TypeReference> Int32(ManagedHandle<AssemblyDefinition> asm)=>new ManagedHandle<TypeReference>(asm.GetRef().MainModule.ImportReference(typeof(int)));
    
    // uint8 
    [UnmanagedCallersOnly(EntryPoint = "uint8",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<TypeReference> UInt8(ManagedHandle<AssemblyDefinition> asm)=>new ManagedHandle<TypeReference>(asm.GetRef().MainModule.ImportReference(typeof(byte)));
    // uint16 
    [UnmanagedCallersOnly(EntryPoint = "uint16",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<TypeReference> UInt16(ManagedHandle<AssemblyDefinition> asm)=>new ManagedHandle<TypeReference>(asm.GetRef().MainModule.ImportReference(typeof(ushort)));
    // uint32 
    [UnmanagedCallersOnly(EntryPoint = "uint32",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<TypeReference> UInt32(ManagedHandle<AssemblyDefinition> asm)=>new ManagedHandle<TypeReference>(asm.GetRef().MainModule.ImportReference(typeof(uint)));
    // nuint 
    [UnmanagedCallersOnly(EntryPoint = "nuint",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<TypeReference> NUInt(ManagedHandle<AssemblyDefinition> asm)=>new ManagedHandle<TypeReference>(asm.GetRef().MainModule.ImportReference(typeof(nuint)));
    // valuetype 
    [UnmanagedCallersOnly(EntryPoint = "valuetype",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<TypeReference> ValueType(ManagedHandle<AssemblyDefinition> asm)=>new ManagedHandle<TypeReference>(asm.GetRef().MainModule.ImportReference(typeof(System.ValueType)));
    [UnmanagedCallersOnly(EntryPoint = "type_ref_to_pointer",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<TypeReference> TypeRefToPointer(ManagedHandle<TypeReference> type)=>new ManagedHandle<TypeReference>(TypeReferenceRocks.MakePointerType(type.GetRef()));
    [UnmanagedCallersOnly(EntryPoint = "set_typedef_baseclass",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static void SetTypedefBaseClass(ManagedHandle<ClassDefinition> type,ManagedHandle<TypeReference> baseClass){
        type.GetRef().BaseType = baseClass.GetRef();
    }
}
