
#include <pthread.h>
// Statics

static char *exec_fname;

// Functions

// 128 bit
#define System_Int128_op_Addition_i128i128(a, b) ((a) + (b))
#define System_UInt128_op_Addition_u128u128(a, b) ((a) + (b))

#define System_Int128_op_Subtraction_i128i128(a, b) ((a) - (b))
#define System_UInt128_op_Subtraction_u128u128(a, b) ((a) - (b))

#define System_Int128_op_Multiply_i128i128(a, b) ((a) * (b))
#define System_UInt128_op_Multiply_u128u128(a, b) ((a) * (b))

#define System_UInt128_op_Division_u128u128(a, b) ((a) / (b))

#define System_UInt128_op_Equality_u128u128(a, b) ((a) == (b))
#define System_Int128_op_Equality_i128i128(a, b) ((a) == (b))

#define System_UInt128_op_LessThan_u128u128(a, b) ((a) < (b))
#define System_Int128op_LessThan(a, b) ((a) < (b))

#define System_UInt128_op_ExclusiveOr_u128u128(a, b) ((a) ^ (b))
#define System_Int128_op_ExclusiveOr_i128i128(a, b) ((a) ^ (b))

#define System_UInt128_op_RightShift_u128i32(a, b) ((a) >> (b))
#define System_UInt128_op_RightShift_u128i32(a, b) ((a) >> (b))
#define System_Int128_op_RightShift_i128i32(a, b) ((a) >> (b))

#define System_UInt128_op_LeftShift_u128i32(a, b) ((a) << (b))
#define System_Int128_op_LeftShift_i128i32(a, b) ((a) << (b))

#define System_UInt128_op_OnesComplement_u128(a) (~(a))
#define System_Int128_op_OnesComplement_i128(a) (~(a))

#define System_UInt128_op_BitwiseAnd_u128u128(a, b) ((a) & (b))
#define System_Int128_op_BitwiseAnd_i128i128(a, b) ((a) & (b))

#define System_Int128_op_BitwiseOr_i128i128(a, b) ((a) | (b))
#define System_UInt128_op_BitwiseOr_u128u128(a, b) ((a) | (b))

#define System_UInt128_op_Modulus_u128u128(a, b) ((a) % (b))

#define System_Int128op_UnaryNegation(a) (-(a))

#define System_Int128op_Explicit(arg) ((__int128)(arg))
#define System_UInt128_op_Explicit_i8(arg) ((unsigned __int128)(arg))
#define System_UInt128_op_Explicit_i16(arg) ((unsigned __int128)(arg))
#define System_UInt128_op_Explicit_i32(arg) ((unsigned __int128)(arg))
#define System_UInt128_op_Explicit_i64(arg) ((unsigned __int128)(arg))
#define System_UInt128_op_Explicit_f32(arg) ((unsigned __int128)(arg))

#define System_Int128_op_Implicit_i8(arg) ((__int128)(arg))
#define System_Int128_op_Implicit_i16(arg) ((__int128)(arg))
#define System_Int128_op_Implicit_i32(arg) ((__int128)(arg))
#define System_Int128_op_Implicit_i64(arg) ((__int128)(arg))
#define System_Int128_op_Implicit_is4(arg) ((__int128)(arg))
#define System_Int128_op_Implicit_i128(arg) ((__int128)(arg))

#define System_Int128_op_Implicit_u8(arg) ((__int128)(arg))
#define System_Int128_op_Implicit_u16(arg) ((__int128)(arg))
#define System_Int128_op_Implicit_u32(arg) ((__int128)(arg))
#define System_Int128_op_Implicit_u64(arg) ((__int128)(arg))

#define System_UInt128_op_Explicit_u128(arg) ((unsigned __int128)(arg))
#define System_Int128_op_Explicit_i128(arg) (__int128)(arg)

#define System_Int128_op_Explicit_f64(arg) (__int128)(arg)
#define System_Int128_op_Explicit_f32(arg) (__int128)(arg)

#define System_UInt128_op_Implicit_u8(arg) ((unsigned __int128)(arg))
#define System_UInt128_op_Implicit_u16(arg) ((unsigned __int128)(arg))
#define System_UInt128_op_Implicit_u32(arg) ((unsigned __int128)(arg))
#define System_UInt128_op_Implicit_u64(arg) ((unsigned __int128)(arg))
#define System_UInt128_op_Implicit_us(arg) ((unsigned __int128)(arg))

#define System_Int128_op_UnaryNegation_i128(val) -(val)

#define System_UInt128__ctor_mu128u64u64(upper, lower) (((unsigned __int128)lower) | (((unsigned __int128)upper) << 64))
#define System_Int128__ctor_mu128u64u64(upper, lower) (((__int128)lower) | (((__int128)upper) << 64))
// Consts
#define System_UIntPtr_get_MinValue_() ((uintptr_t)0)
#define System_UIntPtr_get_MaxValue_() (~((uintptr_t)0))
#define System_IntPtr_get_MaxValue_() ((intptr_t)0)
#define System_IntPtr_get_MinValue_() (~((intptr_t)0))

// Bswap
#define System_Buffers_Binary_BinaryPrimitivesReverseEndianness(val) __bswap_32(val)
// Assembly utilis needed for statup
#define System_Reflection_AssemblyGetEntryAssembly() exec_fname
#define System_Reflection_Assemblyget_Location(arg) (arg)
// String
#define System_Stringget_Length(arg) (strlen(arg) - 1)
#define System_Runtime_InteropServices_Marshal_StringToCoTaskMemUTF8_System_String(arg) (uint8_t *)arg
// IO
#define System_ConsoleWrite(chr) putc(chr, stdout)

#define System_Console_WriteLine_u32(arg) printf("%u\n", arg)
#define System_Console_WriteLine_u64(arg) printf("%lu\n", arg)
#define System_Console_WriteLine_i32(arg) printf("%d\n", arg)
#define System_Console_Write_System_String(arg) printf("%s\n", arg)
// Allocation
#define System_Runtime_InteropServices_NativeMemory_AlignedAlloc_usus _mm_malloc
// #define System_Runtime_InteropServices_NativeMemory_AlignedRealloc_pvusus _mm_realloc
#define System_Runtime_InteropServices_NativeMemory_AlignedFree_pv(ptr) _mm_free(ptr)
// Atomics
#define System_Threading_InterlockedCompareExchange(addr, value, comparand) ({typeof(comparand) expected = comparand;typeof(value) val = value;  __atomic_compare_exchange((addr),&(expected),&(val),0,__ATOMIC_SEQ_CST,0); expected; })
#define System_Threading_InterlockedExchange(addr, val) ({typeof(val) value = val;typeof(val) ret; __atomic_exchange((addr),&value,&ret,__ATOMIC_SEQ_CST);ret; })

#define atomic_fetch_or

// Misc
#define System_Numerics_BitOperationsPopCount(arg) __builtin_popcount(arg)
#define System_Numerics_BitOperationsTrailingZeroCount(arg) __builtin_ctz(arg)
#define System_TypeGetTypeFromHandle(handle) handle
#define System_ObjectGetHashCode(object) object
// Math
#define System_MathFSqrt(flot) sqrtf(flot)
// Types
#define a1System_String_getLength(_) 0
typedef char System_String;
typedef struct TypeInfo
{
	int32_t hash;
};
void *System_Runtime_InteropServices_NativeMemoryAlignedRealloc(void *old, size_t new_size, size_t align)
{
	// Reallocating such buffers is not supported yet!
	abort();
}
// Used for startup
#define System_Arrayget_Length(_) 0
#define black_box(val) val
#define System_Environment_GetCommandLineArgs_(_) commandLineArgs;
static char **commandLineArgs = {0};
#define System_Single_IsNaN_f32(val) isnan(val)
#define System_Double_IsNaN_f64(val) isnan(val)

#define System_Buffers_Binary_BinaryPrimitives_ReverseEndianness_u128(val) __builtin_bswap128(val)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndianness_i64(val) __builtin_bswap64(val)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndianness_i32(val) __builtin_bswap32(val)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndianness_i16(val) __builtin_bswap16(val)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndianness_i8(val) (val)

#if defined(__LP64__) || defined(_LP64)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndianness_is(val) (intptr_t)(int64_t) __builtin_bswap64((uint64_t)(int64_t)val)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndianness_us(val) (uintptr_t) __builtin_bswap64((uint64_t)val)
#else
#error "Unsuported architecture size"
#endif

#define System_Exception__ctor_System_ExceptionSystem_String(msg) msg
typedef struct System_Object
{
};
#define System_Object__ctor_() \
	System_Object              \
	{                          \
	}
// other

#define System_Threading_Monitor_Enter_System_Object(_) pthread_mutex_lock(&global_lock)
#define System_Threading_Monitor_Exit_System_Object(_) pthread_mutex_unlock(&global_lock)
pthread_mutex_t global_lock;