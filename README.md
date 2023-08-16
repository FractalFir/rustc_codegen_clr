# What is rustc_codegen_clr?
This is a compiler backend for rustc which targets the .NET platform and runtime, and could enable compiling rust code for the `.NET` runtime. This woudl enable you to use Rust libraries from C#/F#, without little effort.
# .NET runtime has GC, so would not Rusts memory managament be useless here?
Rust code usually heavily uses stack instead of Heap. This would speed up code ruining within the runtime too. As for the heap allocated objects, Rust ownership rules naturally lead to simpler relationships between objects, making GC potentially faster. Additionally, when escape analysis finally lands in the .NET runtime, objects with easy-to-deduce lifetimes could be freed automatically(kind of like in rust).
# .NET is very different form Rust, and a lot of Rust concepts have no CLR equivalent. How do you plan to fix that?
There are 2 routes I could take, which I will likely be unable to mix. The proper, "better" route(managed, more "native to .NET"), and unmanaged(Far harder to inter-op with, maaaybe could be faster in some cases??? but a lot of stuff needs to be done from scratch, and is easy to mess up). 

I am currently going with the managed route, since: 

1. You can mix managed code with unmanaged easily, but doing it vice-versa is a not-pleasant experience.
2. It seems to be mostly working, at least for now?

# How far is the project along:
- [X] Basic functions get translated properly. 
- [X] Basic integer and float types are supported.
- [X] Arithmetic operations work
- [X] Most `if`'s work.
- [X] Basic `match` works.
- [X] While loops work.
- [X] **VERY** basic references work
- [X] Basic optimization.
- [ ] Calls
- [ ] Structs
- [ ] Enums
- [ ] Basic generics
- [ ] iterators
- [ ] for loops
- [ ] References to references(might work, but requires further investigation).
# Issues
Documentation of internals of `rustc` is very often very lacking. This makes this project far harder than it should be.
References in CLR seem to be far more limited than in Rust. This needs to be worked around.
The backend is buggy and rarely produces invalid CLR IR.
The backend crashes any time it encounters something not supported yet.
