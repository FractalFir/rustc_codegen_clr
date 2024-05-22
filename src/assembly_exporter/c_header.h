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
// Consts
#define System_UIntPtrget_MinValue() ((size_t)0)
#define System_UIntPtrget_MaxValue() (~((size_t)0))
// Bswap
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
// Allocation
#define System_Runtime_InteropServices_NativeMemoryAlignedAlloc _mm_malloc
#define System_Runtime_InteropServices_NativeMemoryAlignedFree(ptr) _mm_free(ptr)
//Atomics
#define System_Threading_InterlockedCompareExchange(addr,value,comparand) ({typeof(comparand) expected = comparand;typeof(value) val = value;  __atomic_compare_exchange((addr),&(expected),&(val),0,__ATOMIC_SEQ_CST,0); expected;})
#define System_Threading_InterlockedExchange(addr,val) ({typeof(val) value = val;typeof(val) ret; __atomic_exchange((addr),&value,&ret,__ATOMIC_SEQ_CST);ret;})
// Misc
#define System_Numerics_BitOperationsPopCount(arg) __builtin_popcount(arg)
#define System_Numerics_BitOperationsTrailingZeroCount(arg) __builtin_ctz(arg)
#define System_TypeGetTypeFromHandle(handle) handle
#define System_ObjectGetHashCode(object) object
//Math
#define System_MathFSqrt(flot) sqrtf(flot)
//Types

typedef char* System_String;
typedef struct TypeInfo{
    int32_t hash;
};
void *System_Runtime_InteropServices_NativeMemoryAlignedRealloc(void* old, size_t new_size, size_t align){
	// Reallocating such buffers is not supported yet!
	abort();
}
// Used for startup
#define System_Arrayget_Length(_) 0
#define black_box(val) val