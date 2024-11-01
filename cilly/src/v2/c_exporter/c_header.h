#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

#include <stdlib.h>
#include <string.h>
#include <math.h>
/*
#include <unistd.h>
#include <sys/uio.h>
#include <poll.h>
#include <sched.h>
#include "threads.h"*/
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
int execvp(void *file, void *argv);
#define System_Runtime_InteropServices_Marshal_AllocHGlobali4is(size) malloc(size)
#define System_Runtime_InteropServices_Marshal_AllocHGlobalisis(size) malloc(size)
#define System_Runtime_InteropServices_Marshal_ReAllocHGlobalisisis(ptr, new_size) realloc(ptr, new_size)
#define System_Runtime_InteropServices_Marshal_FreeHGlobalisv(ptr) free(ptr)
#define System_Collections_ICollection_get_Count14System_Runtime30System_Collections_IDictionaryi4(dict) 0
#define System_Runtime_InteropServices_NativeMemory_AlignedAllocususpv(size, align) aligned_alloc(align, size)
#define System_Runtime_InteropServices_NativeMemory_AlignedFreepvv free
void *System_Runtime_InteropServices_NativeMemory_AlignedReallocpvususpv(void *ptr, uintptr_t size, uintptr_t align)
{
    void *new_buff = aligned_alloc(align, size);
    memcpy(new_buff, ptr, size);
    free(ptr);
    return new_buff;
}

#define System_Collections_IEnumerator_MoveNext14System_Runtime40System_Collections_IDictionaryEnumeratorb(arg) false
#define System_UInt128__ctorru16u8u8v(high, low) ((unsigned __int128)(low) | ((unsigned __int128)(high) << 64))
#define System_Int128__ctorri16u8u8v(high, low) (__int128)((unsigned __int128)(low) | ((unsigned __int128)(high) << 64))

#define System_UInt128_op_Additionu16u16u16(lhs, rhs) (lhs + rhs)
#define System_Int128_op_Additioni16i16i16(lhs, rhs) (__int128)((unsigned __int128)lhs + (unsigned __int128)rhs)

#define System_UInt128_op_Subtractionu16u16u16(lhs, rhs) (lhs - rhs)
#define System_Int128_op_Subtractioni16i16i16(lhs, rhs) (__int128)((unsigned __int128)lhs - (unsigned __int128)rhs)

#define System_UInt128_op_Equalityu16u16b(lhs, rhs) (lhs == rhs)
#define System_Int128_op_Equalityi16i16b(lhs, rhs) (lhs == rhs)

#define System_Int128_op_LessThani16i16b(lhs, rhs) (lhs < rhs)
#define System_UInt128_op_LessThanu16u16b(lhs, rhs) (lhs < rhs)

#define System_Int128_op_GreaterThani16i16b(lhs, rhs) (lhs > rhs)
#define System_UInt128_op_GreaterThanu16u16b(lhs, rhs) (lhs > rhs)

#define System_UInt128_op_Multiplyu16u16u16(lhs, rhs) (lhs * rhs)
#define System_Int128_op_Multiplyi16i16i16(lhs, rhs) ((unsigned __int128)lhs * (unsigned __int128)rhs)

#define System_UInt128_op_Divisionu16u16u16(lhs, rhs) (lhs / rhs)

#define System_Int128_op_UnaryNegationi16i16(val) (__int128_t)(0 - ((__uint128_t)(val)))

#define System_Int128_op_BitwiseOri16i16i16(lhs, rhs) (lhs | rhs)
#define System_UInt128_op_BitwiseOru16u16u16(lhs, rhs) (lhs | rhs)

#define System_Int128_op_ExclusiveOri16i16i16(lhs, rhs) (lhs ^ rhs)
#define System_UInt128_op_ExclusiveOru16u16u16(lhs, rhs) (lhs ^ rhs)

#define System_Int128_op_BitwiseAndi16i16i16(lhs, rhs) (lhs & rhs)
#define System_UInt128_op_BitwiseAndu16u16u16(lhs, rhs) (lhs & rhs)

#define System_UInt128_op_RightShiftu16i4u16(val, ammount) val >> ammount
#define System_Int128_op_RightShifti16i4i16(val, ammount) val >> ammount

#define System_Int128_op_LeftShifti16i4i16(val, ammount) val << ammount
#define System_UInt128_op_LeftShiftu16i4u16(val, ammount) val << ammount

#define System_UInt128_op_Explicitu16u8(val) (uint64_t)(val)
#define System_UInt128_op_Explicitu16u4(val) (uint32_t)(val)
#define System_UInt128_op_Explicitu16u2(val) (uint16_t)(val)
#define System_UInt128_op_Explicitu16u1(val) (uint8_t)(val)
#define System_UInt128_op_Explicitu16us(val) (uintptr_t) val
#define System_UInt128_op_Explicitu16i8(val) (int64_t)(val)
#define System_UInt128_op_Explicitu16i4(val) (int32_t)(val)
#define System_UInt128_op_Explicitu16i2(val) (int16_t)(val)
#define System_UInt128_op_Explicitu16i1(val) (int8_t)(val)
#define System_UInt128_op_Explicitu16is(val) (intptr_t) val

#define System_UInt128_op_Explicitu16f4(val) (float)(val)
#define System_UInt128_op_Explicitu16f8(val) (double)(val)

#define System_Int128_op_Expliciti16f4(val) (float)(val)
#define System_Int128_op_Expliciti16f8(val) (double)(val)

#define System_Int128_op_Expliciti16i1(val) (int8_t)(val)
#define System_Int128_op_Expliciti16i2(val) (int16_t)(val)
#define System_Int128_op_Expliciti16i4(val) (int32_t)(val)
#define System_Int128_op_Expliciti16i8(val) (int64_t)(val)
#define System_Int128_op_Expliciti16is(val) (intptr_t) val
#define System_Int128_op_Expliciti16u1(val) (uint8_t)(val)
#define System_Int128_op_Expliciti16u2(val) (uint16_t)(val)
#define System_Int128_op_Expliciti16u4(val) (uint32_t)(val)
#define System_Int128_op_Expliciti16u8(val) (uint64_t)(val)
#define System_Int128_op_Expliciti16us(val) (uintptr_t) val

#define System_UInt128_op_Explicitu16i16(val) (__int128_t)(val)

#define System_UInt128_op_Expliciti1u16(val) (__uint128_t)(val)
#define System_UInt128_op_Expliciti2u16(val) (__uint128_t)(val)
#define System_UInt128_op_Expliciti4u16(val) (__uint128_t)(val)
#define System_UInt128_op_Expliciti8u16(val) (__uint128_t)(val)
#define System_Int128_op_Expliciti16u16(val) (__uint128_t)(val)

#define System_UInt128_op_Explicitf4u16(val) (__uint128_t)(val)
#define System_UInt128_op_Explicitf8u16(val) (__uint128_t)(val)
#define System_UInt128_op_Implicitusu16(val) (__uint128_t)(val)

#define System_Int128_op_Explicitf8i16(val) (__int128_t)(val)
#define System_Int128_op_Explicitf4i16(val) (__int128_t)(val)

#define System_Int128_op_Impliciti1i16(val) (__int128_t)(val)
#define System_Int128_op_Implicitu1i16(val) (__int128_t)(val)
#define System_Int128_op_Impliciti2i16(val) (__int128_t)(val)
#define System_Int128_op_Implicitu2i16(val) (__int128_t)(val)
#define System_Int128_op_Impliciti4i16(val) (__int128_t)(val)
#define System_Int128_op_Implicitu4i16(val) (__int128_t)(val)
#define System_Int128_op_Impliciti8i16(val) (__int128_t)(val)
#define System_Int128_op_Implicitu8i16(val) (__int128_t)(val)
#define System_Int128_op_Implicitisi16(val) (__int128_t)(val)
#define System_Int128_op_Implicitusi16(val) (__int128_t)(val)

#define System_UInt128_op_Implicitu1u16(val) (__uint128_t)(val)
#define System_UInt128_op_Implicitu2u16(val) (__uint128_t)(val)
#define System_UInt128_op_Implicitu4u16(val) (__uint128_t)(val)
#define System_UInt128_op_Implicitu8u16(val) (__uint128_t)(val)
#define System_UInt128_op_Implicitusu16(val) (__uint128_t)(val)

#define System_Int128_op_OnesComplementi16i16(val) ~val
#define System_UInt128_op_OnesComplementu16u16(val) ~val

#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessi16i16(val) (__int128_t) __builtin_bswap128((__uint128_t)val)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessu16u16 __builtin_bswap128
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessi1i1(val) val

#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessu8u8 __builtin_bswap64
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessi8i8(val) (int64_t) __builtin_bswap64((uint64_t)val)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessu4u4 __builtin_bswap32
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessi4i4(val) (int32_t) __builtin_bswap32((uint32_t)val)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessu2u2 __builtin_bswap16
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessi2i2(val) (int16_t) __builtin_bswap16((uint16_t)val)
// Assumes a 64 bit OS.
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessisis(val) (intptr_t) __builtin_bswap64((ulong)val)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessusus __builtin_bswap64

#define System_Numerics_BitOperations_TrailingZeroCountusi4(val) (int32_t) __builtin_ctzl((uint64_t)val)
#define System_Numerics_BitOperations_TrailingZeroCountu4i4(val) (int32_t) __builtin_ctzl((uint64_t)val)
#define System_Numerics_BitOperations_TrailingZeroCountu8i4(val) (int32_t) __builtin_ctzl((uint64_t)val)
#define System_Numerics_BitOperations_LeadingZeroCountu8i4(val) (int32_t) __builtin_ctzl((uint64_t)val)
#define System_Numerics_BitOperations_PopCountusi4(val) __builtin_popcountl((uint64_t)val)
#define System_Numerics_BitOperations_PopCountu4i4(val) __builtin_popcountl((uint32_t)val)
#define System_Numerics_BitOperations_PopCountu8i4(val) __builtin_popcountl((uint64_t)val)

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
    char *res = (char *)malloc(1);
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
#define System_UIntPtr_get_MinValueus() ((uintptr_t)0)

#define System_IntPtr_get_MaxValueis() INTPTR_MAX
#define System_IntPtr_get_MinValueis() INTPTR_MIN

#define System_Exception__ctor14System_Runtime16System_Exceptionsv
#define System_Exception__ctorp14System_Runtime16System_Exceptionsv
#define System_Exception__ctorp14System_Runtime16System_Exceptionstv
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
double System_Double_FusedMultiplyAddf8f8f8f8(double left, double right, double addend)
{
    return left * right + addend;
}
#define System_Type_GetTypeFromHandle14System_Runtime24System_RuntimeTypeHandle14System_Runtime11System_Type
#define System_Type_GetTypeFromHandlep14System_Runtime24System_RuntimeTypeHandle14System_Runtime11System_Type
#define System_Object_GetHashCode14System_Runtime11System_Typei4
#define System_Object_GetHashCodep14System_Runtime11System_Typei4
float System_Single_MaxNumberf4f4f4(float a, float b)
{
    if (a > b)
        return a;
    else
        return b;
}
double System_Double_MaxNumberf8f8f8(double a, double b)
{
    if (a > b)
        return a;
    else
        return b;
}
float System_Single_MinNumberf4f4f4(float a, float b)
{
    if (a < b)
        return a;
    else
        return b;
}
double System_Double_MinNumberf8f8f8(double a, double b)
{
    if (a < b)
        return a;
    else
        return b;
}
float System_Single_FusedMultiplyAddf4f4f4f4(float left, float right, float addend)
{
    return left * right + addend;
}
float System_Single_CopySignf4f4f4(float mag, float sign)
{
    if (sign > 0)
        return fabs(mag);
    else
        return -fabs(mag);
}
double System_Double_CopySignf8f8f8(double mag, double sign)
{
    if (sign > 0)
        return fabs(mag);
    else
        return -fabs(mag);
}
float System_MathF_Truncatef4f4(float val)
{
    fprintf(stderr, "Can't System_MathF_Truncatef4f4 yet.\n");
    abort();
    return 0;
}
double fabsf64(double val);
#define System_Single_Cosf4f4(x) ((float)cos(x))
#define System_Single_Cosf8f8 cos
#define System_Single_Sinf4f4(x) ((float)sin(x))
#define System_Single_Sinf8f8 sin
#define System_Double_Absf8f8 fabsf64
#define System_Double_Absf4f4 fabsf32
#define System_MathF_Sqrtf4f4(x) (float)sqrt((double)x)
#define System_MathF_Sqrtf8f8 sqrt
#define System_Single_Powf4f4f4(a, b) (float)pow(a, b)
#define System_Single_Powf8f8f8 pow
#define System_Double_Powf8f8f8 pow
#define System_Int128_get_Zeroi16(v) ((__int128_t)0)
float System_Single_Exp2f4f4(float input)
{
    fprintf(stderr, "Can't System_Single_Exp2f4f4 yet.\n");
    abort();
    return 0.0f;
}
float System_Single_Logf4f4(float input)
{
    fprintf(stderr, "Can't System_Single_Logf4f4 yet.\n");
    abort();
    return 0.0f;
}
float System_Single_Log2f4f4(float input)
{
    fprintf(stderr, "Can't System_Single_Log2f4f4 yet.\n");
    abort();
    return 0.0f;
}
float System_Single_Log10f4f4(float input)
{
    fprintf(stderr, "Can't System_Single_Log10f4f4 yet.\n");
    abort();
    return 0.0f;
}
float System_Math_Floorf8f8(float input)
{
    fprintf(stderr, "Can't System_Math_Floorf8f8 yet.\n");
    abort();
    return 0.0f;
}
double System_Math_Sqrtf8f8(double input)
{
    fprintf(stderr, "Can't System_Math_Sqrtf8f8 yet.\n");
    abort();
    return 0.0f;
}
double System_Double_Log10f8f8(double input)
{
    fprintf(stderr, "Can't System_Double_Log10f8f8 yet.\n");
    abort();
    return 0.0f;
}
double System_Math_Truncatef8f8(double input)
{
    fprintf(stderr, "Can't System_Math_Truncatef8f8 yet.\n");
    abort();
    return 0.0f;
}
uint32_t System_UInt32_RotateRightu4i4u4(uint32_t val, int32_t ammount)
{
    fprintf(stderr, "Can't System_UInt32_RotateRightu4i4u4 yet.\n");
    abort();
    return 0;
}
uint8_t System_Byte_RotateRightu1i4u1(uint8_t val, int32_t ammount)
{
    fprintf(stderr, "Can't System_Byte_RotateRightu1i4u1 yet.\n");
    abort();
    return 0;
}
uint32_t System_Threading_Interlocked_CompareExchangeru4u4u4u4(uint32_t *addr, uint32_t value, uint32_t comparand)
{
    fprintf(stderr, "Can't System_Threading_Interlocked_CompareExchangeru4u4u4u4 yet.\n");
    abort();
    return 0;
}
uint64_t System_Threading_Interlocked_CompareExchangeru8u8u8u8(uint64_t *addr, uint64_t value, uint64_t comparand)
{
    fprintf(stderr, "Can't System_Threading_Interlocked_CompareExchangeru8u8u8u8 yet.\n");
    abort();
    return 0;
}
uintptr_t System_Threading_Interlocked_CompareExchangerusususus(uintptr_t *addr, uintptr_t value, uintptr_t comparand)
{
    fprintf(stderr, "Can't System_Threading_Interlocked_CompareExchangerusususus yet.\n");
    abort();
    return 0;
}
intptr_t System_Threading_Interlocked_CompareExchangerisisisis(intptr_t *addr, intptr_t value, intptr_t comparand)
{
    fprintf(stderr, "Can't System_Threading_Interlocked_CompareExchangerisisisis yet.\n");
    abort();
    return 0;
}

uint32_t System_Threading_Interlocked_Exchangeru4u4u4(uint32_t *addr, uint32_t val)
{
    fprintf(stderr, "Can't System_Threading_Interlocked_Exchangeru4u4u4 yet.\n");
    abort();
    return 0;
}
uintptr_t System_Threading_Interlocked_Exchangerususus(uintptr_t *addr, uintptr_t val)
{
    fprintf(stderr, "Can't System_Threading_Interlocked_Exchangerususus yet.\n");
    abort();
    return 0;
}

uint32_t System_Threading_Interlocked_Addru4u4u4(uint32_t *addr, uint32_t addend)
{
    fprintf(stderr, "Can't System_Threading_Interlocked_Addru4u4u4 yet.\n");
    abort();
}
uint32_t System_UInt32_RotateLeftu4i4u4(uint32_t val, int32_t ammount)
{
    fprintf(stderr, "Can't System_UInt32_RotateLeftu4i4u4 yet.\n");
    abort();
}
uint16_t System_UInt16_RotateRightu2i4u2(uint16_t val, int32_t ammount)
{
    fprintf(stderr, "Can't System_UInt16_RotateRightu2i4u2 yet.\n");
    abort();
}
uint16_t System_UInt16_RotateLeftu2i4u2(uint16_t val, int32_t ammount)
{
    fprintf(stderr, "Can't System_UInt16_RotateLeftu2i4u2 yet.\n");
    abort();
}

uint64_t System_UInt64_RotateRightu8i4u8(uint64_t val, int32_t ammount)
{
    fprintf(stderr, "Can't System_UInt64_RotateRightu8i4u8 yet.\n");
    abort();
}

uint16_t System_UInt128_RotateLeftu16i4u16(uint16_t val, int32_t ammount)
{
    fprintf(stderr, "Can't System_UInt128_RotateLeftu16i4u16 yet.\n");
    abort();
}

uint64_t System_UInt64_RotateLeftu8i4u8(uint64_t val, int32_t ammount)
{
    fprintf(stderr, "Can't System_UInt64_RotateLeftu8i4u8 yet.\n");
    abort();
}
unsigned __int128 System_UInt128_RotateRightu16i4u16(unsigned __int128 val, int32_t amount)
{
    fprintf(stderr, "Can't System_UInt128_RotateRightu16i4u16 yet.\n");
    abort();
}

uint8_t System_Byte_RotateLeftu1i4u1(uint8_t val, int32_t ammount)
{
    fprintf(stderr, "Can't System_Byte_RotateLeftu1i4u1 yet.\n");
    abort();
}
unsigned __int128 System_UInt128_LeadingZeroCountu16u16(unsigned __int128 val)
{
    fprintf(stderr, "Can't System_UInt128_LeadingZeroCountu16u16 yet.\n");
    abort();
}
unsigned __int128 System_UInt128_PopCountu16u16(unsigned __int128 val)
{
    fprintf(stderr, "Can't System_UInt128_PopCountu16u16 yet.\n");
    abort();
}
unsigned __int128 System_UInt128_TrailingZeroCountu16u16(unsigned __int128 val)
{
    fprintf(stderr, "Can't System_UInt128_TrailingZeroCountu16u16 yet.\n");
    abort();
}

uint32_t System_Math_Minu4u4u4(uint32_t lhs, uint32_t rhs)
{
    if (lhs > rhs)
    {
        return rhs;
    }
    else
    {
        return lhs;
    }
}
int32_t System_Math_Clampi4i4i4i4(int32_t val, int32_t min, int32_t max)
{
    if (val > max)
    {
        return max;
    }
    else if (val < min)
    {
        return min;
    }
    else
    {
        return val;
    }
}
int64_t System_Math_Clampi8i8i8i8(int64_t val, int64_t min, int64_t max)
{
    if (val > max)
    {
        return max;
    }
    else if (val < min)
    {
        return min;
    }
    else
    {
        return val;
    }
}

__int128 System_Int128_Clampi16i16i16i16(__int128 val, __int128 min, __int128 max)
{
    if (val > max)
    {
        return max;
    }
    else if (val < min)
    {
        return min;
    }
    else
    {
        return val;
    }
}
__int128 System_Int128_get_MinValuei16()
{
    fprintf(stderr, "Can't System_Int128_get_MinValuei16 yet.\n");
    abort();
}
__int128 System_Int128_get_MaxValuei16()
{
    fprintf(stderr, "Can't System_Int128_get_MinValuei16 yet.\n");
    abort();
}

double System_Double_Exp2f8f8(double val)
{
    fprintf(stderr, "Can't System_Double_Exp2f8f8 yet.\n");
    abort();
}
void System_Threading_Thread_MemoryBarrierv() {}
static int _argcStaticStorage;
static char **_argvStaticStorage;
char **System_Environment_GetCommandLineArgsa1st() { return _argvStaticStorage; }
uintptr_t ld_len(void *arr)
{
    return strlen((const char *)arr) / sizeof(uintptr_t);
}
intptr_t System_Runtime_InteropServices_Marshal_StringToCoTaskMemUTF8stis(char *str)
{
    uintptr_t len = strlen(str);
    char *ptr = (char *)malloc(len + 1);
    memcpy(ptr, str, len + 1);
    return len;
}
typedef struct __simdvecf44
{
    float arr[4];
} __simdvecf44;
typedef struct __simdveci44
{
    int32_t arr[4];
} __simdveci44;
const float inff = 1.0 / 0.0;
const double inf = 1.0 / 0.0;
int fcntl(int fd, int op, ...);
long syscall(long number, ...);