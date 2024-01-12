namespace AssemblyUtilis;
using Mono.Cecil;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
public class TypeDef
{
    [UnmanagedCallersOnly(EntryPoint = "new_type_def",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<TypeDefinition> NewTypeDef(RustString nameSpace,RustString name,TypeAttributes attributes){
        var typeDef = new TypeDefinition(nameSpace.ToString(),name.ToString(),attributes);
        return new ManagedHandle<TypeDefinition>(typeDef);
    }
    [UnmanagedCallersOnly(EntryPoint = "new_field_def",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<FieldDefinition> NewFieldDef(ManagedHandle<AssemblyDefinition> asm, RustString fieldName,FieldAttributes attributes,ManagedHandle<Type> type,bool hasOffset,uint offset){
        var typeref = asm.GetRef().MainModule.ImportReference(type.GetRef());
        var fieldDef = new FieldDefinition(fieldName.ToString(),attributes,typeref);
        if(hasOffset){
            fieldDef.Offset = (int)offset; 
        }
        return new ManagedHandle<FieldDefinition>(fieldDef);
    }
    /*
      [UnmanagedCallersOnly(EntryPoint = "new_type_ref",CallConvs = new[] { typeof(CallConvCdecl) })]
    ManagedHandle<TypeReference> NewTypeRef(RustString nameSpace,RustString name,bool isValueType){
        var tref = new TypeReference(nameSpace.ToString(),name.ToString());
        tref.IsValueType = isValueType;
        return new ManagedHandle<TypeReference>(tref)
    }*/
    [UnmanagedCallersOnly(EntryPoint = "int32",CallConvs = new[] { typeof(CallConvCdecl) })]
    public static ManagedHandle<Type> Int32()=>new ManagedHandle<Type>(typeof(int));
}
