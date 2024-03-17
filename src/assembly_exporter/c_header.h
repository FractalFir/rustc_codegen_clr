#define System_Int128op_Addition(a,b) ((a) + (b))
#define System_UInt128op_Addition(a,b) ((a) + (b))
#define System_UInt128op_Equality(a,b) ((a) == (b))
#define System_Int128op_Equality(a,b) ((a) == (b))
#define System_ConsoleWriteLine(arg)
/*
#define System_ConsoleWriteLine(arg) _Generic((arg), \
    char *: puts(arg), \
    const char *: puts(arg), \
    int8_t: printf("%" PRId8 "\n", arg), \
    uint8_t: printf("%" PRIu8 "\n", arg), \
    int16_t: printf("%" PRId16 "\n", arg), \
    uint16_t: printf("%" PRIu16 "\n", arg), \
    int32_t: printf("%" PRId32 "\n", arg), \
    uint32_t: printf("%" PRIu32 "\n", arg), \
    int64_t: printf("%" PRId64 "\n", arg), \
    uint64_t: printf("%" PRIu64 "\n", arg), \
    float: printf("%f\n", arg), \
    double: printf("%lf\n", arg), \
    default: (fprintf(stdout, "Error: Unsupported type\n"), -1) \
)(arg)*/
#define System_ConsoleWrite(chr) putc(chr,stdout)
#define ctor_System_UInt128(upper,lower) (((unsigned __int128)lower) | (((unsigned __int128)upper)<<64))
#define ctor_System_Int128(upper,lower) (((__int128)lower) | (((__int128)upper)<<64))