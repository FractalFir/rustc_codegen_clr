.method public static hidebysig float32 sqrtf32(float32) aggressiveinlining{
    ldarg.0
    call float32 [System.Runtime]System.MathF::Sqrt(float32)
    ret
}
.method public static void putc(uint8){
    ldarg.0
    conv.i2
    call void [System.Console]System.Console::Write(char)
    ret
}
.method public static void puts(uint8*){
    .locals(
        [0] uint8
    )
    LOOP:
    ldarg.0
    ldind.i1
    stloc.0
    ldloc.0
    //ldstr "char is:"
    //call void [System.Console]System.Console::Write([System.Runtime]System.String)
    //ldloc.0
    //call void [System.Console]System.Console::WriteLine(unsigned int32)
    ldc.i4 0
    beq END_LOOP
    ldloc.0
    call void putc(uint8)
    ldarg.0
    ldc.i4.1
    add
    starg.s 0
    //ldstr "PUTING A CHAR"
    //call void [System.Console]System.Console::WriteLine([System.Runtime]System.String)
    //ldstr "STRING PTR IS:"
    //call void [System.Console]System.Console::Write([System.Runtime]System.String)
    //ldarg.0
    //conv.i
    //conv.i8
    //call void [System.Console]System.Console::WriteLine(int64)
    
    br LOOP
    END_LOOP:
    ret
}
//Rust core
.class public core.option.Option extends [System.Runtime]System.ValueType{
}
.class public core.panic.Location extends [System.Runtime]System.ValueType{
}
