# What is rustc_codegen_clr?
*NOTE: this project is a very early proof-of-concept*
This is a compiler backend for rustc which targets the .NET platform and runtime, and could enable compiling Rust code for the `.NET` runtime. This would enable you to use some Rust libraries from C#/F#, with little effort. 
# .NET runtime has GC, so would not Rusts memory management be useless here?
Rust code usually heavily uses stack instead of Heap. This would speed up code running within the CLR runtime too. As for the heap allocated objects, they will be allocated from unmanged(non-GC) memory, and will be allocated/freed exactly like in Rust.
# I can already load shared libraries from C# code, so is this not useless? Does this improve interop?
The Rust APIs this codegen exposes to C#/F# code would be only slightly easier to use than something you could expose in a .so or .dll Rust library.

Interop would still require some effort, but the Rust code would be bundled together with everything else. You will also have the guarantee that types you use from C# are exactly the same as the ones in C#, preventing any issues coming from such mismatch. All types can be safely sent between Rust and C#, with exactly the same layout.

Additionally, since all Rust code compiled with this codegen can be bundled with C#/F# code, you would no longer need to ship different versions of the library for different architectures. Any architecture supported by CLR would work out of the box, without the exact same binary.

You also avoid the cost of switches between code running within the runtime and outside it. While this cost is not something unbearable, it is not something you can easily get rid of, and reducing it has some safety penalties associated with. In this case, all code will run inside the runtime, meaning no transition between code running inside runtime and outside of it will occur.

Compiling Rust to CLR is potentially better for the JIT. Since CLR's JIT now "sees" all the code, it can make better decisions regarding optimization, producing faster code.
# Licensing
`rustc_codegen_clr` is dual licensed under MIT license or Apache License, Version 2.0.  
# Compatibility?
`rustc_codegen_clr` is tested solely on Linux x86_64. Anything else *should*, but does not have to work.
# How far is the project along:
## Functionality
- [X] Basic functions get translated properly. 
- [X] Arithmetic operations work
- [X] Most `if`'s work.
- [X] Basic `match` works.
- [X] While loops work.
- [X] Calls
- [X] Basic IL optimization.
- [X] Setting value of a reference
- [X] Getting value of a reference
- [ ] Creating slices from arrays
- [X] Creating arrays
- [ ] Indexing arrays *broken*
- [X] Getting values of fields
- [X] Setting fields
- [X] Pointer dereferecing
- [X] Generics *GATS don't work in some edge cases*
- [X] for loops *with some minor limitations*
## Types
*NOTE: This section says only if a type can be translated for .NET to understand. This **does not** mean the type is fully usable.*
- [X] All integer and float types are supported. Support for math with 128-bit integers is very limited 
- [X] References are supported
- [ ] Arrays, slices
- [X] Void type
- [X] Combinations of all of the above. 
- [X] Structs
- [X] Enums
- [ ] Tuples
- [ ] Traits *Some, not all*
- [X] iterators
# Issues
Backend still does not understand some Rust optimizations, and you may need to prevent them to allow for compilation.
While testing is a bit more extensive, there still are a lot of edge cases which may break this backend.
The backend crashes anytime it encounters something not supported yet.
# Basic benchmarks:
*NOTE* Those are benchmarks which put Rust on the worst footing, since they involve no allocations/GC at all. They serve as a baseline to determine the best possible performance.
All tests were run in CoreCLR .NET runtime, version `7.0.11` The host system was `Linux fedora 6.5.5-200.fc38.x86_64`, and the CPU was `13th Gen Intel(R) Core(TM) i5-13500HX`.

`Codegen Optimzations Disabled` means that the code was compiled in release mode., but post-MIR, codegen-internal optimizations were disabled. 
## Fibonachi of 10, recursive.
|Test Method| avg of 100K runs |
|-----------|------------------------|
| Rust native(release)| 100 ns |
| Rust native(debug)| 360 ns |
| Rust .NET(default optimizations) | 270 ns |
| Rust .NET(codegen optimizations disabled) | 330 ns |
| C# release | 250 ns |
| C# debug | 370 ns |
As you can see, the difference between optimized C# and optimized .NET Rust code is not all that big. It is noticeable(~10%), but I would say it is a pretty good result considering how few optimizations are done right now. With a couple bigger changes coming later down the line, the gap could become non-existent in the future. Since this benchmark is meant to show the worst case scenario, Rust could already outperform C# in a wide range of more memory-intensive scenarios. 
