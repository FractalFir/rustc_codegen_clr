#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include "threads.h"
#include <stdlib.h>
#include <string.h>
#ifdef __STDC_VERSION__

#else
#error "C version too old(< C99), and unsuported"
#endif

#define eprintf(...) fprintf(stderr, __VA_ARGS__)
union System_Collections_IDictionary
{
};
union System_Collections_IDictionaryEnumerator
{
};
union System_Collections_DictionaryEntry
{
};
union System_String
{
};
#define System_Runtime_InteropServices_Marshal_AllocHGlobali4is(size) malloc(size)
#define System_Runtime_InteropServices_Marshal_AllocHGlobalisis(size) malloc(size)
#define System_Collections_ICollection_get_Count14System_Runtime30System_Collections_IDictionaryi4(dict) 0
#define System_Runtime_InteropServices_NativeMemory_AlignedAllocususpv(size, align) aligned_alloc(align, size)
#define System_Collections_IEnumerator_MoveNext14System_Runtime40System_Collections_IDictionaryEnumeratorb(arg) false
#define System_UInt128__ctorru16u8u8v(high, low) ((unsigned __int128)(low) || ((unsigned __int128)(high) << 64))
#define System_Int128__ctorri16u8u8v(high, low) (__int128)((unsigned __int128)(low) || ((unsigned __int128)(high) << 64))

#define System_UInt128_op_Additionu16u16u16(lhs, rhs) (lhs + rhs)
#define System_Int128_op_Additioni16i16i16(lhs, rhs) (__int128)((unsigned __int128)lhs + (unsigned __int128)rhs)

#define System_UInt128_op_Subtractionu16u16u16(lhs, rhs) (lhs - rhs)
#define System_Int128_op_Subtractioni16i16i16(lhs, rhs) (__int128)((unsigned __int128)lhs - (unsigned __int128)rhs)

#define System_UInt128_op_Equalityu16u16b(lhs, rhs) (lhs == rhs)
#define System_Int128_op_Equalityi16i16b(lhs, rhs) (lhs == rhs)

#define System_Int128_op_LessThani16i16b(lhs, rhs) (lhs < rhs)
#define System_UInt128_op_LessThanu16u16b(lhs, rhs) (lhs < rhs)

#define System_Int128_op_Greaterhani16i16b(lhs, rhs) (lhs > rhs)
#define System_UInt128_op_GreaterThanu16u16b(lhs, rhs) (lhs > rhs)

#define System_UInt128_op_Multiplyu16u16u16(lhs, rhs) (lhs * rhs)

#define System_Int128_op_UnaryNegationi16i16(val) -(val)
#define System_Int128_op_BitwiseOri16i16i16(lhs, rhs) (lhs | rhs)

#define System_Int128_op_BitwiseAndi16i16i16(lhs, rhs) (lhs & rhs)

#define System_UInt128_op_RightShiftu16i4u16(val, ammount) val << ammount

#define System_UInt128_op_Explicitu16is(val) (intptr_t) val
#define System_UInt128_op_Explicitu16i8(val) (int64_t)(val)
#define System_UInt128_op_Explicitu16i4(val) (int32_t)(val)
#define System_UInt128_op_Explicitu16i2(val) (int16_t)(val)
#define System_UInt128_op_Explicitu16i1(val) (int8_t)(val)

#define System_UInt128_op_Explicitu16u8(val) (uint64_t)(val)
#define System_UInt128_op_Explicitu16u4(val) (uint32_t)(val)
#define System_UInt128_op_Explicitu16u2(val) (uint16_t)(val)
#define System_UInt128_op_Explicitu16u1(val) (uint8_t)(val)

#define System_UInt128_op_Explicitu16f4(val) (float)(val)

#define System_Int128_op_Expliciti16f4(val) (float)(val)
#define System_Int128_op_Expliciti16u16(val) (__uint128_t)(val)

#define System_UInt128_op_Explicitf4u16(val) (__uint128_t)(val)
#define System_UInt128_op_Implicitusu16(val) (__uint128_t)(val)

#define System_Int128_op_Explicitf8i16(val) (__int128_t)(val)

#define System_Int128_op_Impliciti1i16(val) (__int128_t)(val)
#define System_Int128_op_Implicitu1i16(val) (__int128_t)(val)
#define System_Int128_op_Impliciti2i16(val) (__int128_t)(val)
#define System_Int128_op_Implicitu2i16(val) (__int128_t)(val)
#define System_Int128_op_Impliciti4i16(val) (__int128_t)(val)
#define System_Int128_op_Implicitu4i16(val) (__int128_t)(val)
#define System_Int128_op_Impliciti8i16(val) (__int128_t)(val)
#define System_Int128_op_Implicitu8i16(val) (__int128_t)(val)

#define System_Int128_op_OnesComplementi16i16(val) ~val
#define System_UInt128_op_OnesComplementu16u16(val) ~val

#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessi16i16(val) (__int128_t) __builtin_bswap128((__uint128_t)val)

// Assumes a 64 bit OS.
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessisis(val) (intptr_t) __builtin_bswap64((ulong)val)

#define System_Numerics_BitOperations_TrailingZeroCountusi4(val) (int32_t) __builtin_ctzl((uint64_t)val)
#define System_Numerics_BitOperations_PopCountusi4(val) __builtin_popcountl((uint64_t)val)

union System_Collections_IDictionary System_Environment_GetEnvironmentVariables14System_Runtime30System_Collections_IDictionary()
{
    union System_Collections_IDictionary res;
    return res;
};
union System_Collections_IDictionaryEnumerator System_Collections_IDictionary_GetEnumerator14System_Runtime30System_Collections_IDictionary14System_Runtime40System_Collections_IDictionaryEnumerator(union System_Collections_IDictionary dict)
{
    union System_Collections_IDictionaryEnumerator res;
    return res;
}
union System_Collections_DictionaryEntry System_Collections_IEnumerator_get_Current14System_Runtime40System_Collections_IDictionaryEnumeratoro(union System_Collections_IDictionaryEnumerator dict)
{
    union System_Collections_DictionaryEntry res;
    return res;
}
union System_String System_String_Concat_()
{
    union System_String res;
    return res;
}
const char *System_Runtime_InteropServices_Marshal_StringToCoTaskMemUTF8sis(union System_String str)
{
    char *res = malloc(1);
    res[0] = '\0';
    return res;
}
void System_Console_WriteLineu8v(ulong arg)
{
    printf("%lu\n", arg);
}
void System_Console_WriteLinei8v(long arg)
{
    printf("%ld\n", arg);
}
void System_Console_WriteLineu4v(uint arg)
{
    printf("%u\n", arg);
}
void System_Console_WriteLinei4v(int arg)
{
    printf("%u\n", arg);
}
#define System_String_Concatooos(...) System_String_Concat_()

#define System_UIntPtr_get_MaxValueus() UINTPTR_MAX
#define System_UIntPtr_get_MinValueus() UINTPTR_MIN

#define System_IntPtr_get_MaxValueis() INTPTR_MAX
#define System_IntPtr_get_MinValueis() INTPTR_MIN

#define System_Exception__ctor14System_Runtime16System_Exceptionsv(arg) arg
float System_Single_Clampf4f4f4f4(float d, float min, float max)
{
    const float t = d < min ? min : d;
    return t > max ? max : t;
}
double System_Double_Clampf8f8f8f8(double d, double min, double max)
{
    const double t = d < min ? min : d;
    return t > max ? max : t;
}
