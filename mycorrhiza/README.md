# Mycorrhiza - Rust/.NET interop layer
Mycorrhiza is a part of the `rustc_codegen_clr` project, and it is responsible for allowing Rust code to call .NET functions and facilitates interacting with managed objects. In the future, it will include an inter-op guide, and some safe abstractions, to help with development of Rust applications targeting .NET.
It is heavily WIP, and not yet recommended for general use.
# Examples:
## Hello World using StringBuilder and Console:
```rust
    let sb = mycorrhiza::system::text::StringBuilder::empty();
    sb.append_char('H');
    sb.append_char('e');
    sb.append_char('l');
    sb.append_char('l');
    sb.append_char('o');
    sb.append_char(' ');
    sb.append_char('W');
    sb.append_char('o');
    sb.append_char('r');
    sb.append_char('l');
    sb.append_char('d');
    sb.append_char('!');
    sb.append_char('\n');
    sb.append_char('\r');

    let mstr = sb.to_mstring();
    mycorrhiza::system::console::Console::writeln_string(mstr);
```
