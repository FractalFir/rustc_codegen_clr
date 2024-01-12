namespace AssemblyUtilis;
using System.Runtime.InteropServices;
[StructLayout(LayoutKind.Sequential)]
public unsafe struct RustString
{
    byte* utf8Bytes;
    nuint length;
    public override string ToString()=>
        Marshal.PtrToStringUTF8((nint)utf8Bytes,(int)length);
}
