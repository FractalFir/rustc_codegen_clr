// Statics

static char* exec_fname;

// Functions

// 128 bit
#define System_Int128op_Addition(a,b) ((a) + (b))
#define System_UInt128op_Addition(a,b) ((a) + (b))

#define System_Int128op_Subtraction(a,b) ((a) - (b))
#define System_UInt128op_Subtraction(a,b) ((a) - (b))

#define System_Int128op_Multiply(a,b) ((a) * (b))
#define System_UInt128op_Multiply(a,b) ((a) * (b))

#define System_UInt128op_Equality(a,b) ((a) == (b))
#define System_Int128op_Equality(a,b) ((a) == (b))

#define System_UInt128op_LessThan(a,b) ((a) < (b))
#define System_Int128op_LessThan(a,b) ((a) < (b))

#define System_UInt128op_ExclusiveOr(a,b) ((a) ^ (b))
#define System_Int128op_ExclusiveOr(a,b) ((a) ^ (b))

#define System_UInt128op_RightShift(a,b) ((a) >> (b))
#define System_Int128op_RightShift(a,b) ((a) >> (b))

#define System_UInt128op_OnesComplement(a) (~(a))
#define System_Int128op_OnesComplement(a) (~(a))

#define System_Int128op_UnaryNegation(a) (-(a))

#define System_Int128op_Explicit(arg) ((__int128)(arg))
#define System_UInt128op_Explicit(arg) ((unsigned __int128)(arg))
#define System_Int128op_Implicit(arg) ((__int128)(arg))
#define System_UInt128op_Implicit(arg) ((unsigned __int128)(arg))

#define ctor_System_UInt128(upper,lower) (((unsigned __int128)lower) | (((unsigned __int128)upper)<<64))
#define ctor_System_Int128(upper,lower) (((__int128)lower) | (((__int128)upper)<<64))

#define System_Buffers_Binary_BinaryPrimitivesReverseEndianness(val) __bswap_32(val)
// Assembly utilis needed for statup
#define System_Reflection_AssemblyGetEntryAssembly() exec_fname
#define System_Reflection_Assemblyget_Location(arg) (arg)
// String 
#define System_Stringget_Length(arg) (strlen(arg) - 1)
#define System_Runtime_InteropServices_MarshalStringToCoTaskMemUTF8(arg) arg
// IO
#define System_ConsoleWrite(chr) putc(chr,stdout)
#define System_ConsoleWriteLine(arg)
// Allcation
#define System_Runtime_InteropServices_NativeMemoryAlignedAlloc _mm_malloc
// Misc
#define System_Numerics_BitOperationsPopCount(arg) __builtin_popcount(arg)

//Types

typedef char* System_String;
typedef struct TypeInfo{
    int32_t hash;
}
