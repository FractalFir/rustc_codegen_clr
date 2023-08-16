# What is rustc_codegen_clr?
This is a compiler backend for rustc which targets the .NET platform and runtime, and could enable compiling rust code for the `.NET` runtime. This woudl enable you to use Rust libraries from C#/F#, without little effort.
# .NET runtime has GC, so would not Rust be useless here?
Rust code usually heavily uses stack instead of Heap. This would speed up code ruining within the runtime too. As for the heap allocated objects, Rust ownership rules naturally lead to simpler relationships between objects, making GC potentially faster. Additionally, when escape analysis finally lands in the .NET runtime, objects with easy-to-deduce lifetimes could be freed automatically(kind of like in rust).
