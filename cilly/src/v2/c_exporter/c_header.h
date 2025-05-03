#include <stdio.h>
#include <stdint.h>
/* Backup for targets that don't support i128 - TODO: replace this with software emulation!*/
#if !defined(__SIZEOF_INT128__) || defined(__LCC__)
#define __int128_t long long
#define __int128 __int128_t
#define __uint128_t unsigned long long
#define uint128_t unsigned long long
#endif
#if !defined(__TINYC__) && !defined(__LCC__)
#include <stdbool.h>
#elif defined(__LCC__)
#define bool uint8_t
#define false 0
#define true 1
#else
#define bool _Bool
#define false 0
#define true 1
#endif
#ifdef __LCC__
#define __func__ "unknown"
#endif
#include <stdlib.h>
#include <string.h>
#ifndef __SDCC
#include <math.h>
#endif
#ifdef __LCC__
#define inline
#endif
#if !(defined(__TINYC__) || defined(__SDCC) || defined(_MSC_VER))
#include <mm_malloc.h>
#elif defined(_MSC_VER)
void * _aligned_malloc(
    size_t size,
    size_t alignment
);
inline void* _mm_malloc(size_t size, size_t align){
    return _aligned_malloc(align,size);
}
inline void _mm_free (void *p){
    _aligned_free(p);
}
#else
inline void* _mm_malloc(size_t size, size_t align){
    return aligned_alloc(align,size);
}
inline void _mm_free (void *p){
    free(p);
}
#endif
#if (defined(__TINYC__) || defined(__SDCC) || defined(__LCC__))
inline uint16_t __builtin_bswap16(uint16_t val){
    uint16_t res;
    uint8_t* ptr = (uint8_t*)&val;
    uint8_t* rptr = (uint8_t*)&res;
    rptr[0] = ptr[1];
    rptr[1] = ptr[0];
    return res;
}
inline uint32_t __builtin_bswap32(uint32_t val){
    uint32_t res;
    uint8_t* ptr = (uint8_t*)&val;
    uint8_t* rptr = (uint8_t*)&res;
    rptr[0] = ptr[3];
    rptr[1] = ptr[2];
    rptr[2] = ptr[1];
    rptr[3] = ptr[0];
    return res;
}
inline uint64_t __builtin_bswap64(uint64_t val){
    uint64_t res;
    uint8_t* ptr = (uint8_t*)&val;
    uint8_t* rptr = (uint8_t*)&res;
    rptr[0] = ptr[7];
    rptr[1] = ptr[6];
    rptr[2] = ptr[5];
    rptr[3] = ptr[4];
    rptr[4] = ptr[3];
    rptr[5] = ptr[2];
    rptr[6] = ptr[1];
    rptr[7] = ptr[0];
    return res;
}
inline uint128_t __builtin_bswap128(uint128_t val){
    uint128_t res;
    uint8_t* ptr = (uint8_t*)&val;
    uint8_t* rptr = (uint8_t*)&res;
    rptr[0] = ptr[15];
    rptr[1] = ptr[14];
    rptr[2] = ptr[13];
    rptr[3] = ptr[12];
    rptr[4] = ptr[11];
    rptr[5] = ptr[10];
    rptr[6] = ptr[9];
    rptr[7] = ptr[8];
    rptr[8] = ptr[7];
    rptr[9] = ptr[6];
    rptr[10] = ptr[5];
    rptr[11] = ptr[4];
    rptr[12] = ptr[3];
    rptr[13] = ptr[2];
    rptr[14] = ptr[1];
    rptr[16] = ptr[0];
    return res;
}
#endif
#ifdef __clang__
static inline __uint128_t __builtin_bswap128(__uint128_t val){
    __uint128_t res;
    uint8_t* ptr = &val;
    uint8_t* rptr = &res;
    rptr[0] = ptr[15];
    rptr[1] = ptr[14];
    rptr[2] = ptr[13];
    rptr[3] = ptr[12];
    rptr[4] = ptr[11];
    rptr[5] = ptr[10];
    rptr[6] = ptr[9];
    rptr[7] = ptr[8];
    rptr[8] = ptr[7];
    rptr[9] = ptr[6];
    rptr[10] = ptr[5];
    rptr[11] = ptr[4];
    rptr[12] = ptr[3];
    rptr[13] = ptr[2];
    rptr[14] = ptr[1];
    rptr[15] = ptr[0];
    return res;
}
#else
#if defined(__SIZEOF_INT128__) && !defined(__LCC__)
__uint128_t __builtin_bswap128(__uint128_t val);
#endif
#endif
#if defined(__TINYC__) || defined(__COMPCERT__) || defined(__LCC__)
#define _Thread_local __attribute__((section(".tbss")))
#elif defined(_MSC_VER)
#define _Thread_local __declspec(thread)
#elif defined(__SDCC)
// Assumes single-threaded env!
#define _Thread_local 
#endif

#if !(defined(__SDCC) || defined(_MSC_VER) || defined(__COMPCERT__) || defined(__LCC__))
#define FORCE_NOT_ZST 
#else
#define FORCE_NOT_ZST char force_not_zst;
#endif

#if !(defined(__SDCC) || defined(_MSC_VER) || defined(__LCC__))
#include <alloca.h>
#define register_alloca_aligned(type, align, hash)
#define loc_alloc_aligned(name, type, align, hash) name = (void*)((((size_t)(alloca(sizeof(type) + align) + align - 1)) / align)*align);
#elif defined(_MSC_VER)
#define alloca _alloca
#define register_alloca_aligned(type, align, hash)
#define loc_alloc_aligned(name, type, align, hash) name = (void*)((((size_t)(alloca(sizeof(type) + align) + align - 1)) / align)*align);
#else
#define register_alloca_aligned(type, align, hash) char hash[sizeof(type) + align];  
#define loc_alloc_aligned(name, type, align, hash) name = (void*)((((size_t)(&hash + align - 1)) / align)*align); 
#endif

#ifdef __clang__
#define __atomic_compare_exchange_4 __atomic_compare_exchange_n
#define __atomic_compare_exchange_8 __atomic_compare_exchange_n
#define _Float128 long double
#elif (defined(__SDCC) || defined(__LCC__))
// WARNING! Assumes a single-threaded, no-interrupt eviroment!
bool __atomic_compare_exchange_4(uint32_t *ptr, uint32_t *expected, uint32_t desired, bool weak, int success_memorder, int failure_memorder){
    if(*ptr == *expected){
        *ptr = desired;
        return true;
    }
    return false;
}
bool __atomic_compare_exchange_8(uint64_t *ptr, uint64_t *expected, uint64_t desired, bool weak, int success_memorder, int failure_memorder){
    if(*ptr == *expected){
        *ptr = desired;
        return true;
    }
    return false;
}
bool __atomic_compare_exchange_n(uintptr_t *ptr, uintptr_t *expected, uintptr_t desired, bool weak, int success_memorder, int failure_memorder){
    if(*ptr == *expected){
        *ptr = desired;
        return true;
    }
    return false;
}
#endif

#ifdef FLT16_MIN
static inline _Float16 System_Half_op_Explicitf32f16(float val){
    return (_Float16)val;
}
#endif

/* Allocator APIs*/
#define System_Runtime_InteropServices_Marshal_AllocHGlobali32isize(size) malloc(size)
#define System_Runtime_InteropServices_Marshal_AllocHGlobalisizeisize(size) malloc(size)
#define System_Runtime_InteropServices_Marshal_ReAllocHGlobalisizeisizeisize(ptr, new_size) realloc(ptr, new_size)
#define System_Runtime_InteropServices_Marshal_FreeHGlobalisizev(ptr) free(ptr)
static void pal_internal_error(){}
#ifdef __LCC__
void* aligned_alloc(size_t align, size_t size){
	if(align > 8) abort();
	return malloc(size);
}
#endif
static inline void* System_Runtime_InteropServices_NativeMemory_AlignedAllocusizeusizepv(size_t size,size_t align) {
    if (align > (0x10000))pal_internal_error();
    return aligned_alloc(align, size);
}
#define System_Runtime_InteropServices_NativeMemory_AlignedFreepvv free
static inline void *System_Runtime_InteropServices_NativeMemory_AlignedReallocpvusizeuspv(void *ptr, uintptr_t size, uintptr_t align)
{
    void *new_buff = aligned_alloc(align, size);
    memcpy(new_buff, ptr, size);
    free(ptr);
    return new_buff;
}
/*Utility macros*/
#ifndef __SDCC
#define eprintf(...) fprintf(stderr, __VA_ARGS__)
#define BUILTIN_UNSUPORTED(NAME,OUTPUT, ARGLIST) static inline OUTPUT NAME ARGLIST { eprintf("Function " #NAME "is not yet supported!"); abort();}
#else
#define eprintf(...) printf(__VA_ARGS__)
#define BUILTIN_UNSUPORTED(NAME,OUTPUT, ARGLIST) static inline OUTPUT NAME ARGLIST { eprintf("Function " #NAME "is not yet supported!"); abort();}
#endif
#if defined(__SDCC) || defined(__LCC__)
#define NAN (0.0 / 0.0)
#endif
#ifdef __SDCC
void abort(){
    printf("Called abort");
    while(1);
}
#endif
/*Wrappers for certain 128 bit ops: TODO: remove this once all ops are ported to new cilly builtins*/
#define System_UInt128_op_Additionu128u128u128(lhs, rhs) (lhs + rhs)
#define System_Int128_op_Additioni128i128i128(lhs, rhs) (__int128)((unsigned __int128)lhs + (unsigned __int128)rhs)

#define System_UInt128_op_Subtractionu128u128u128(lhs, rhs) (lhs - rhs)
#define System_Int128_op_Subtractioni128i128i128(lhs, rhs) (__int128)((unsigned __int128)lhs - (unsigned __int128)rhs)

#define System_Int128_op_LessThani128i128b(lhs, rhs) (lhs < rhs)
#define System_UInt128_op_LessThanu128u128b(lhs, rhs) (lhs < rhs)

#define System_Int128_op_GreaterThani128i128b(lhs, rhs) (lhs > rhs)
#define System_UInt128_op_GreaterThanu128u128b(lhs, rhs) (lhs > rhs)

#define System_UInt128_op_Multiplyu128u128u128(lhs, rhs) (lhs * rhs)
#define System_Int128_op_Multiplyi128i128i128(lhs, rhs) ((unsigned __int128)lhs * (unsigned __int128)rhs)

#define System_UInt128_op_Divisionu128u128u128(lhs, rhs) (lhs / rhs)
#define System_UInt128_op_RightShiftu128i32u128(val, amount) val >> amount
#define System_Int128_op_RightShifti128i32i128(val, amount) val >> amount

#define System_Int128_op_LeftShifti128i32i128(val, amount) val << amount
#define System_UInt128_op_LeftShiftu128i32u128(val, amount) val << amount

#define System_Int128_op_BitwiseOri128i128i128(lhs, rhs) (lhs | rhs)
#define System_UInt128_op_BitwiseOru128u128u128(lhs, rhs) (lhs | rhs)

#define System_Int128_op_ExclusiveOri128i128i128(lhs, rhs) (lhs ^ rhs)
#define System_UInt128_op_ExclusiveOru128u128u128(lhs, rhs) (lhs ^ rhs)

#define System_Int128_op_BitwiseAndi128i128i128(lhs, rhs) (lhs & rhs)
#define System_UInt128_op_BitwiseAndu128u128u128(lhs, rhs) (lhs & rhs)

#define System_Int128_op_UnaryNegationi128i128(val) (__int128_t)(0 - ((__uint128_t)(val)))

#define System_UInt128_op_Explicitu128u64(val) (uint64_t)(val)
#define System_UInt128_op_Explicitu128u32(val) (uint32_t)(val)
#define System_UInt128_op_Explicitu128u16(val) (uint16_t)(val)
#define System_UInt128_op_Explicitu128u8(val) (uint8_t)(val)
#define System_UInt128_op_Explicitu128usize(val) (uintptr_t) val
#define System_UInt128_op_Explicitu128i64(val) (int64_t)(val)
#define System_UInt128_op_Explicitu128i32(val) (int32_t)(val)
#define System_UInt128_op_Explicitu128i16(val) (int16_t)(val)
#define System_UInt128_op_Explicitu128i8(val) (int8_t)(val)
#define System_UInt128_op_Explicitu128isize(val) (intptr_t) val

#define System_UInt128_op_Explicitu128f32(val) (float)(val)
#define System_UInt128_op_Explicitu128f64(val) (double)(val)

#define System_Int128_op_Expliciti128f32(val) (float)(val)
#define System_Int128_op_Expliciti128f64(val) (double)(val)

#define System_Int128_op_Expliciti128i8(val) (int8_t)(val)
#define System_Int128_op_Expliciti128i16(val) (int16_t)(val)
#define System_Int128_op_Expliciti128i32(val) (int32_t)(val)
#define System_Int128_op_Expliciti128i64(val) (int64_t)(val)
#define System_Int128_op_Expliciti128isize(val) (intptr_t) val
#define System_Int128_op_Expliciti128u8(val) (uint8_t)(val)
#define System_Int128_op_Expliciti128u16(val) (uint16_t)(val)
#define System_Int128_op_Expliciti128u32(val) (uint32_t)(val)
#define System_Int128_op_Expliciti128u64(val) (uint64_t)(val)
#define System_Int128_op_Expliciti128usize(val) (uintptr_t) val

#define System_UInt128_op_Explicitu128i128(val) (__int128_t)(val)

#define System_UInt128_op_Expliciti8u128(val) (__uint128_t)(val)
#define System_UInt128_op_Expliciti16u128(val) (__uint128_t)(val)
#define System_UInt128_op_Expliciti32u128(val) (__uint128_t)(val)
#define System_UInt128_op_Expliciti64u128(val) (__uint128_t)(val)
#define System_Int128_op_Expliciti128u128(val) (__uint128_t)(val)
#ifndef NO_FLOAT
static inline __uint128_t System_UInt128_op_Explicitf32u128(float val) {
    if(val < 0.0){
        return 0;
    }else{
        return (__uint128_t)val;
    }
}
static inline __uint128_t System_UInt128_op_Explicitf64u128(double val) {
    if(val < 0.0){
        return 0;
    }else{
        return (__uint128_t)val;
    }
}
#endif

#define System_Int128_op_Explicitf64i128(val) (__int128_t)(val)
#define System_Int128_op_Explicitf32i128(val) (__int128_t)(val)

#define System_Int128_op_Impliciti8i128(val) (__int128_t)(val)
#define System_Int128_op_Implicitu8i128(val) (__int128_t)(val)
#define System_Int128_op_Impliciti16i128(val) (__int128_t)(val)
#define System_Int128_op_Implicitu16i128(val) (__int128_t)(val)
#define System_Int128_op_Impliciti32i128(val) (__int128_t)(val)
#define System_Int128_op_Implicitu32i128(val) (__int128_t)(val)
#define System_Int128_op_Impliciti64i128(val) (__int128_t)(val)
#define System_Int128_op_Implicitu64i128(val) (__int128_t)(val)
#define System_Int128_op_Implicitisizei128(val) (__int128_t)(val)
#define System_Int128_op_Implicitusizei128(val) (__int128_t)(val)

#define System_UInt128_op_Implicitu8u128(val) (__uint128_t)(val)
#define System_UInt128_op_Implicitu16u128(val) (__uint128_t)(val)
#define System_UInt128_op_Implicitu32u128(val) (__uint128_t)(val)
#define System_UInt128_op_Implicitu64u128(val) (__uint128_t)(val)
#define System_UInt128_op_Implicitusizeu128(val) (__uint128_t)(val)

#define System_Int128_op_OnesComplementi128i128(val) ~val
#define System_UInt128_op_OnesComplementu128u128(val) ~val

#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessi128i128(val) (__int128_t) __builtin_bswap128((__uint128_t)val)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessu128u128 __builtin_bswap128
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessi8i8(val) val

#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessu64u64 __builtin_bswap64
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessi64i64(val) (int64_t) __builtin_bswap64((uint64_t)val)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessu32u32 __builtin_bswap32
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessi32i32(val) (int32_t) __builtin_bswap32((uint32_t)val)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessu16u16 __builtin_bswap16
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessi16i16(val) (int16_t) __builtin_bswap16((uint16_t)val)
/*Assumes a 64 bit OS.*/
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessisizeisize(val) (intptr_t) __builtin_bswap64((uint64_t)val)
#define System_Buffers_Binary_BinaryPrimitives_ReverseEndiannessusizeusize __builtin_bswap64
#if !(defined(__SDCC) || defined(__LCC__))
static inline int32_t System_Numerics_BitOperations_TrailingZeroCountusizei32(uintptr_t val) {if (val == 0) return sizeof(uintptr_t) * 8; return (int32_t) __builtin_ctzl((uint64_t)val);}
static inline int32_t System_Numerics_BitOperations_TrailingZeroCountu32i32(uint32_t val) {if (val == 0) return sizeof(uint32_t) * 8; return (int32_t) __builtin_ctzl((uint32_t)val);}
static inline int32_t System_Numerics_BitOperations_TrailingZeroCounti32i32(int32_t val) {if (val == 0) return sizeof(int32_t) * 8; return (int32_t) __builtin_ctzl((uint32_t)val);}
static inline int32_t System_Numerics_BitOperations_TrailingZeroCountu64i32(uint64_t val) {if (val == 0) return sizeof(uint64_t) * 8; return (int32_t) __builtin_ctzl((uint64_t)val);}
static inline int32_t System_UInt128_TrailingZeroCountu128u128(__uint128_t val) {
    if (val == 0) return sizeof(__uint128_t) * 8; 
    if (((uint64_t)val) == 0){
        return (int32_t) System_Numerics_BitOperations_TrailingZeroCountu64i32((uint64_t)(val >> 64)) + 64;
    }
    else{
        return (int32_t) System_Numerics_BitOperations_TrailingZeroCountu64i32((uint64_t)(val));
    }
}
static inline int32_t System_Numerics_BitOperations_LeadingZeroCountu64i32(uint64_t val) { if (val == 0) return 64; return __builtin_clzl(val); }
static inline int32_t System_Numerics_BitOperations_LeadingZeroCountusizei32(uintptr_t val) { if (val == 0) return sizeof(uintptr_t) * 8; return __builtin_clzl((uint64_t)val); }
#endif

#define System_Numerics_BitOperations_PopCountusizei32(val) __builtin_popcountll((uint64_t)val)
#define System_Numerics_BitOperations_PopCountu32i32(val) __builtin_popcountll((uint32_t)val)
#define System_Numerics_BitOperations_PopCountu64i32(val) __builtin_popcountll((uint64_t)val)
#ifndef __SDCC
static inline __uint128_t System_UInt128_PopCountu128u128(__uint128_t val) {
    return __builtin_popcountll((uint64_t)val) +  __builtin_popcountll((uint64_t)(val>>64)); 
}
#endif

#define System_Console_WriteLinev() printf("\n")
#define System_Console_WriteLinestv(msg) printf("%s\n", msg)
#define System_Console_WriteLinef64v(val) printf("%f\n", val)

#define System_Console_Writeu64v(val) printf("%d", val)

#define System_String_Concatststst(a, b) a b
#define System_String_Concatstststst(a, b, c) a b c
#define System_String_Concatststststst(a, b, c, d) a b c d
static inline void System_Console_WriteLineu64v(uint64_t arg)
{
    printf("%lu\n", arg);
}
static inline void System_Console_WriteLinei64v(int64_t arg)
{
    printf("%ld\n", arg);
}
#define System_Console_WriteLineu32v(arg) printf("%u\n", arg)
static inline void System_Console_WriteLinei32v(int32_t arg)
{
    printf("%u\n", arg);
}
int execvp(void *file, void *argv);

#define System_UIntPtr_get_MaxValueusize() UINTPTR_MAX
#define System_UIntPtr_get_MinValueusize() ((uintptr_t)0)

#define System_IntPtr_get_MaxValueisize() INTPTR_MAX
#define System_IntPtr_get_MinValueisize() INTPTR_MIN

#define System_Exception__ctor14System_Runtime16System_Exceptionsv
#define System_Exception__ctorp14System_Runtime16System_Exceptionsv
#define System_Exception__ctorp14System_Runtime16System_Exceptionstv
#ifndef NO_FLOAT
static inline float System_Single_Clampf32f32f32f32(float d, float min, float max)
{
    const float t = d < min ? min : d;
    return t > max ? max : t;
}
static inline double System_Double_Clampf64f64f64f64(double d, double min, double max)
{
    const double t = d < min ? min : d;
    return t > max ? max : t;
}
static inline double System_Double_FusedMultiplyAddf64f64f64f64(double left, double right, double addend)
{
    return left * right + addend;
}
#endif 
#define System_Type_GetTypeFromHandle14System_Runtime24System_RuntimeTypeHandle14System_Runtime11System_Type
#define System_Type_GetTypeFromHandlep14System_Runtime24System_RuntimeTypeHandle14System_Runtime11System_Type
#define System_Object_GetHashCode14System_Runtime11System_Typei32
#define System_Object_GetHashCodep14System_Runtime11System_Typei32

#define System_Int128_get_MinValuei128() ((__uint128_t)((__int128_t)(-1)) >> 1)
#ifndef NO_FLOAT
static inline float System_Single_MaxNumberf32f32f32(float a, float b)
{
    if (a != a) return b;
    if (b != b) return a;
    if (a > b)
        return a;
    else
        return b;
}
static inline double System_Double_MaxNumberf64f64f64(double a, double b)
{
    if (a != a) return b;
    if (b != b) return a;
    if (a > b)
        return a;
    else
        return b;
}
static inline float System_Single_MinNumberf32f32f32(float a, float b)
{
    if (a != a) return b;
    if (b != b) return a;
    if (a < b)
        return a;
    else
        return b;
}
static inline double System_Double_MinNumberf64f64f64(double a, double b)
{
    if (a != a) return b;
    if (b != b) return a;
    if (a < b)
        return a;
    else
        return b;
}
static inline float System_Single_FusedMultiplyAddf32f32f32f32(float left, float right, float addend)
{
    return left * right + addend;
}
static inline float System_Single_CopySignf32f32f32(float mag, float sign)
{
    if (sign > 0)
    {
        if (mag > 0)
            return mag;
        else
            return -mag;
    }
    else
    {
        if (mag > 0)
            return -mag;
        else
            return mag;
    }
}
static inline double System_Double_CopySignf64f64f64(double mag, double sign)
{
    if (sign > 0)
    {
        if (mag > 0)
            return mag;
        else
            return -mag;
    }
    else
    {
        if (mag > 0)
            return -mag;
        else
            return mag;
    }
}
#endif
BUILTIN_UNSUPORTED(System_MathF_Truncatef32f32,float,(float val))
#ifdef __TINYC__
BUILTIN_UNSUPORTED(__atomic_compare_exchange_4,uint32_t,(uint32_t *ptr, uint32_t *expected, uint32_t desired, bool weak, int success_memorder, int failure_memorder))
BUILTIN_UNSUPORTED(__atomic_compare_exchange_8,uint64_t,(uint64_t *ptr, uint64_t *expected, uint64_t desired, bool weak, int success_memorder, int failure_memorder))
BUILTIN_UNSUPORTED(__atomic_compare_exchange_n,uintptr_t,(uintptr_t *ptr, uintptr_t *expected, uintptr_t desired, bool weak, int success_memorder, int failure_memorder))
#endif
double fabsf64(double val);
#define System_Single_Cosf32f32(x) ((float)cos(x))
#define System_Double_Cosf64f64 cos
#define System_Single_Sinf32f32(x) ((float)sin(x))
#define System_Double_Sinf64f64 sin
#define System_Double_Absf64f64 fabsf64
#define System_Single_Absf32f32 fabsf32
#define System_Single_Sqrtf32f32(x) (float)sqrt((double)x)
#define System_MathF_Sqrtf32f32(x) (float)sqrt((double)x)
#define System_Double_Sqrtf64f64 sqrt
#define System_MathF_Sqrtf64f64 sqrt
#define System_Single_Floorf32f32(x) (float)floor((double)x)
#define System_Single_Ceilingf32f32(x) (float)ceil((double)x)
#define System_MathF_Roundf32f32(x) (float)round((double)x)
#define System_Single_Powf32f32f32(a, b) (float)pow(a, b)
#define System_Single_Powf64f64f64 pow
#define System_Double_Powf64f64f64 pow
#define System_Int128_get_Zeroi128(v) ((__int128_t)0)
#define System_UInt128_get_Zerou128(v) ((__uint128_t)0)
#define System_Math_Minisizeisizeisize(x, y) (((x) < (y)) ? (x) : (y))
#define System_Math_Maxisizeisizeisize(x, y) (((x) > (y)) ? (x) : (y))
#define System_Math_Minusizeusizeusize(x, y) (((x) < (y)) ? (x) : (y))
#define System_Math_Maxusizeusizeusize(x, y) (((x) > (y)) ? (x) : (y))
#ifndef __LCC__
typedef struct TSWData
{
    void *start_routine;
    void *arg;
} TSWData;
void _tcctor();
static inline void *thread_start_wrapper(TSWData *data)
{
    _tcctor();
    void *(*start_routine)(void *) = (void *)data->start_routine;
    void *arg = data->arg;
    free(data);
    return start_routine(arg);
}
#endif
#if !(defined(__SDCC) || defined(__LCC__))
int32_t pthread_create(void *thread,
                       void *attr,
                       void *(*start_routine)(void *),
                       void *threadarg);
static inline int32_t pthread_create_wrapper(void *thread,
                               void *attr,
                               void *start_routine,
                               void *arg)
{
    TSWData *data = (TSWData*)malloc(sizeof(TSWData));
    data->start_routine = start_routine;
    data->arg = arg;

    return pthread_create(thread, attr, (void *)thread_start_wrapper, data);
}
#define pthread_create pthread_create_alias
#endif
#ifndef NO_FLOAT
double exp(double);
double exp2(double);
static inline float System_Single_Exp2f32f32(float input){
    return exp2(input);
}
static inline double System_Double_Exp2f64f64(double input){
    return exp2(input);
}
static inline float System_Single_Expf32f32(float input){
    return exp(input);
}
static inline double System_Double_Expf64f64(double input){
    return exp(input);
}
static inline float System_Single_Log2f32f32(float input){
    return log2f(input);
}
static inline float System_Double_Log2f64f64(float input){
    return log2(input);
}
static inline float System_Single_Log10f32f32(float input){
    return log10f(input);
}
static inline float System_Double_Log10f64f64(float input){
    return log10(input);
}
static inline float System_Single_Logf32f32(float input){
    return logf(input);
}
static inline float System_Double_Logf64f64(float input){
    return log(input);
}
#endif 
#define System_Math_Roundf64f64(input) round(input)
#define System_Math_Floorf64f64(input) floor(input)
#define System_Math_Sqrtf64f64(input) sqrt(input)
#define System_Math_Ceilingf64f64(input) ceil(input)
#define System_Double_Ceilingf64f64(input) ceil(input)
#define System_MathF_Ceilingf32f32(input) (float)ceil((double)input)
#define System_Math_Floorf64f64(input) floor(input)
#define System_Double_Floorf64f64(input) floor(input)
#define System_MathF_Floorf32f32(input) (float)floor((double)input)
#define System_Math_Truncatef64f64(input) trunc(input)
#define System_Double_Truncatef64f64(input) trunc(input)
#define System_Single_Truncatef32f32(input) (float)trunc((double)input)
static inline uint32_t System_Threading_Interlocked_CompareExchangeru32u32u32u32(uint32_t *addr, uint32_t value, uint32_t comparand)
{
    uint32_t res = 0;
    if (__atomic_compare_exchange_4(addr, &comparand, value, true, 5, 5))
    {
        return comparand;
    }
    else
    {
        /* On failure, value is written to comparand. */
        return comparand;
    }
}
static inline int32_t System_Threading_Interlocked_CompareExchangeri32i32i32i32(int32_t* addr, int32_t value, int32_t comparand){
    return System_Threading_Interlocked_CompareExchangeru32u32u32u32((uint32_t *) addr, value, comparand);
}
static inline uint64_t System_Threading_Interlocked_CompareExchangeru64u64u64u64(uint64_t *addr, uint64_t value, uint64_t comparand)
{
    uint64_t res = 0;
    if (__atomic_compare_exchange_8(addr, &comparand, value, true, 5, 5))
    {
        return comparand;
    }
    else
    {
        /* On failure, value is written to comparand. */
        return comparand;
    }
}
static inline uintptr_t System_Threading_Interlocked_CompareExchangerusizeusizeusizeusize(uintptr_t *addr, uintptr_t value, uintptr_t comparand)
{
    uintptr_t res = 0;
    if (__atomic_compare_exchange_n(addr, &comparand, value, true, 5, 5))
    {
        return comparand;
    }
    else
    {
        /* On failure, value is written to comparand. */
        return comparand;
    }
}
static inline intptr_t System_Threading_Interlocked_CompareExchangerisizeisizeisizeisize(intptr_t *addr, intptr_t value, intptr_t comparand)
{
    intptr_t res = 0;
    if (__atomic_compare_exchange_n(addr, &comparand, value, true, 5, 5))
    {
        return comparand;
    }
    else
    {
        /* On failure, value is written to comparand. */
        return comparand;
    }
}
#if !(defined(__SDCC) || defined(__LCC__))
static inline uint32_t System_Threading_Interlocked_Exchangeru32u32u32(uint32_t *addr, uint32_t val)
{
    uint32_t ret;
    __atomic_exchange(addr, &val, &ret, 5);
    return ret;
}
static inline uintptr_t System_Threading_Interlocked_Exchangerusizeusizeusize(uintptr_t *addr, uintptr_t val)
{
    uintptr_t ret;
    __atomic_exchange(addr, &val, &ret, 5);
    return ret;
}
#endif
static inline uint32_t System_Threading_Interlocked_Addru32u32u32(uint32_t *addr, uint32_t addend)
{
    eprintf("Can't System_Threading_Interlocked_Addru32u32u32 yet.\n");
    abort();
}
static inline uint32_t System_UInt32_RotateLeftu32i32u32(uint32_t val, int32_t amount)
{
    amount = amount % (sizeof(uint32_t)*8);
    if(amount == 0) return val;
    return ((val << amount) | (val >> ( (sizeof(uint32_t)*8) - amount)));
}
static inline uintptr_t System_UIntPtr_RotateLeftusizei32usize(uintptr_t val, uintptr_t amount)
{
     amount = amount % (sizeof(uintptr_t)*8);
      if(amount == 0) return val;
    return ((val << amount) | (val >> ( (sizeof(uintptr_t)*8) - amount)));
}

static inline uint16_t System_UInt16_RotateRightu16i32u16(uint16_t val, int32_t amount)
{
   amount = amount % 16;
    if(amount == 0) return val;
    return ((val >> amount) | (val << (16 - amount)));
}
static inline uint16_t System_UInt16_RotateLeftu16i32u16(uint16_t val, int32_t amount)
{
    amount = amount % 16;
     if(amount == 0) return val;
    return ((val << amount) | (val >> (16 - amount)));
}
static inline uint32_t System_UInt32_RotateRightu32i32u32(uint32_t val, int32_t amount)
{
    amount = amount % (sizeof(uint32_t) * 8);
     if(amount == 0) return val;
    return (val >> amount) | (val << ((sizeof(uint32_t) * 8) - amount));
}
static inline uintptr_t System_UIntPtr_RotateRightusizei32usize(uintptr_t val, int32_t amount)
{
    amount = amount % (sizeof(uintptr_t) * 8);
     if(amount == 0) return val;
    return (val >> amount) | (val << ((sizeof(uintptr_t) * 8) - amount));
}
static inline uint8_t System_Byte_RotateRightu8i32u8(uint8_t val, int32_t amount)
{
    amount = amount % 8;
     if(amount == 0) return val;
    return (val >> amount) | (val << (8 - amount));
}
static inline uint64_t System_UInt64_RotateRightu64i32u64(uint64_t val, int32_t amount)
{
    amount = amount % 64;
     if(amount == 0) return val;
    return (val >> amount) | (val << (64 - amount));
}
static inline unsigned __int128 System_UInt128_RotateLeftu128i32u128(unsigned __int128 val, int32_t amount)
{
    amount = amount % (sizeof(unsigned __int128)*8);
     if(amount == 0) return val;
    return ((val << amount) | (val >> ( (sizeof(unsigned __int128)*8) - amount)));
}
static inline uint64_t System_UInt64_RotateLeftu64i32u64(uint64_t val, int32_t amount)
{
    amount = amount % 64;
     if(amount == 0) return val;
    return (val << amount) | (val >> (64 - amount));
}
static inline unsigned __int128 System_UInt128_RotateRightu128i32u128(unsigned __int128 val, int32_t amount)
{
    amount = amount % (sizeof(unsigned __int128)*8);
     if(amount == 0) return val;
    return ((val >> amount) | (val << ( (sizeof(unsigned __int128)*8) - amount)));
}
static inline uint8_t System_Byte_RotateLeftu8i32u8(uint8_t val, int32_t amount)
{
     amount = amount % (sizeof(uint8_t)*8);
      if(amount == 0) return val;
    return ((val << amount) | (val >> ( (sizeof(uint8_t)*8) - amount)));
}
#if !(defined(__SDCC) || defined(__LCC__))
static inline unsigned __int128 System_UInt128_LeadingZeroCountu128u128(__uint128_t val) {
    if (val == 0) return sizeof(__uint128_t) * 8; 
    if ((val>>64) == 0){
        return (unsigned __int128) System_Numerics_BitOperations_LeadingZeroCountu64i32((uint64_t)(val)) + 64;
    }
    else{
        return (unsigned __int128) System_Numerics_BitOperations_LeadingZeroCountu64i32((uint64_t)(val >> 64));
    }
}
#endif
static inline uint32_t System_Math_Minu32u32u32(uint32_t lhs, uint32_t rhs)
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
static inline int32_t System_Math_Clampi32i32i32i32(int32_t val, int32_t min, int32_t max)
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
static inline int64_t System_Math_Clampi64i64i64i64(int64_t val, int64_t min, int64_t max)
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

static inline __int128 System_Int128_Clampi128i128i128i128(__int128 val, __int128 min, __int128 max)
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
#define System_Int128_get_MaxValuei128() (-(((__uint128_t)((__int128_t)(-1L))) >> 1) - 1)
static inline void System_Threading_Thread_MemoryBarrierv() {}
#ifndef __SDCC
#ifdef MAIN_FILE
int argc;
char **argv;
#else
extern int argc;
extern char **argv;
#endif
static inline char **System_Environment_GetCommandLineArgsa1st() { return argv; }
static inline uintptr_t ld_len(void *arr)
{
    void **elem = (void **)arr;
    uintptr_t len = 0;
    while (*elem != 0)
    {
        len += 1;
        elem += 1;
    }
    return len;
}
static inline intptr_t System_Runtime_InteropServices_Marshal_StringToCoTaskMemUTf64stisize(char *str)
{
    uintptr_t len = strlen(str);
    char *ptr = (char *)malloc(len + 1);
    memcpy(ptr, str, len + 1);
    return len;
}
#endif
float fabsf32(float input);
#ifdef FLT16_MIN
#define System_Half_op_Explicitf32f2(f)(_Float16)(f)
#endif
#define TYPEDEF_SIMDVEC(TYPE, MANGLED, SIZE) \
    typedef struct __simdvec##MANGLED##_##SIZE  \
    {                                        \
        TYPE arr[SIZE];                      \
    } __simdvec##MANGLED##_##SIZE;
#define TYPEDEF_SIMDVECS_TYPE(TYPE, MANGLED) TYPEDEF_SIMDVEC(TYPE, MANGLED, 2) TYPEDEF_SIMDVEC(TYPE, MANGLED, 4) TYPEDEF_SIMDVEC(TYPE, MANGLED, 8) TYPEDEF_SIMDVEC(TYPE, MANGLED, 16) TYPEDEF_SIMDVEC(TYPE, MANGLED, 32) TYPEDEF_SIMDVEC(TYPE, MANGLED, 64)
TYPEDEF_SIMDVECS_TYPE(int8_t, i8)
TYPEDEF_SIMDVECS_TYPE(int16_t, i16)
TYPEDEF_SIMDVECS_TYPE(int32_t, i32)
TYPEDEF_SIMDVECS_TYPE(int64_t, i64)
TYPEDEF_SIMDVECS_TYPE(uint8_t, u8)
TYPEDEF_SIMDVECS_TYPE(uint16_t, u16)
TYPEDEF_SIMDVECS_TYPE(uint32_t, u32)
TYPEDEF_SIMDVECS_TYPE(uint64_t, u64)
TYPEDEF_SIMDVECS_TYPE(float, f32)
TYPEDEF_SIMDVECS_TYPE(double, f64)

static const float inff = 1.0 / 0.0;
static const double inf = 1.0 / 0.0;
int fcntl(int fd, int op, ...);
long syscall(long number, ...);
#ifndef __SDCC
static inline uint8_t **get_environ()
{
    extern char **environ;
    return (uint8_t **)environ;
}
#endif
union System_MidpointRounding{int32_t inner;};
#ifndef NO_FLOAT
double round(double);
static inline double System_Math_Roundf6414System_Runtime23System_MidpointRoundingf64(double val,union System_MidpointRounding rounding){
	return round(val);
}
float roundf(float);
static inline float System_MathF_Roundf3214System_Runtime23System_MidpointRoundingf32(float val,union System_MidpointRounding rounding){
	return roundf(val);
}
#endif
int ioctl(int fd, unsigned long op, ...);
int pthread_attr_init(void* attr);
int pthread_attr_destroy(void* attr);
int poll(void *fds, uint64_t nfds, int timeout);
int pthread_getattr_np(uint64_t thread, void *attr);
int pthread_attr_getstack(void *attr,
                          void *stackaddr, size_t *stacksize);
int sched_getaffinity(int32_t pid, size_t cpusetsize,
                      void *mask);
int sigemptyset(void *set);
int sigaction(int sig, void* act,
       void* oact); 
int sigaltstack(void *new_ss, void* old_ss);
