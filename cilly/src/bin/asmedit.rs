use std::{
    io::{stdin, Read},
    num::NonZeroU32,
};

use cilly::v2::{
    access, asm::Assembly, il_exporter::ILExporter, Access, CILIter, MethodImpl, MethodRefIdx,
};

fn main() {
    let mut asm = Assembly::default();
    let mut cmd = String::new();

    loop {
        cmd.clear();
        stdin().read_line(&mut cmd).unwrap();
        if cmd.trim() == "exit" {
            return;
        }

        let (stem, body) = match cmd.split_once('(') {
            Some((stem, body)) => (stem, body),
            None => (cmd.as_ref(), ""),
        };
        let body = body.trim().trim_matches(')').trim();
        let stem = stem.trim();
        match stem {
            "deadcode" => asm.eliminate_dead_code(),
            "opt" => {
                let mut fuel = asm.default_fuel();
                eprintln!("Preparing to optimize the assembly with {fuel:?}");
                let start = std::time::Instant::now();
                asm.opt(&mut fuel);
                if fuel.exchausted() {
                    eprintln!(
                        "Optimization exchausted fuel, finishing early. Took {} ms",
                        start.elapsed().as_millis()
                    );
                } else {
                    eprintln!(
                        "Optimized the assemblty, {fuel:?} remains. Took {} ms",
                        start.elapsed().as_millis()
                    );
                }
            }
            "gc" => asm = asm.clone().link(Assembly::default()),
            "exit" => return,
            "open" => {
                let path = body;
                let path = path.trim().trim_matches('\'').trim();
                let mut asm_file =
                    std::fs::File::open(path).expect("ERROR:Could not open the assembly file!");
                let mut asm_bytes = Vec::with_capacity(0x10000);
                asm_file
                    .read_to_end(&mut asm_bytes)
                    .expect("ERROR: Could not load the assembly file!");
                println!("Loading an assembly");
                let loaded_asm = postcard::from_bytes(&asm_bytes).unwrap();
                // Loading an assembly

                if asm.class_defs().is_empty() {
                    asm = loaded_asm;
                } else {
                    println!("Linking an assembly");
                    asm = asm.clone().link(loaded_asm);
                }
                println!("Loaded assembly");
            }
            "save" => {
                let path = body;
                let path = path.trim().trim_matches('\'').trim();
                println!("Preparing to save the assembly");
                asm.save_tmp(&mut std::fs::File::create(path).unwrap())
                    .unwrap();
                println!("Saved the assembly");
            }
            "mbyaccess" => {
                // Print all methods with a certain visibility
                let access = body;
                let access = match access.as_ref() {
                    "extern" => Access::Extern,
                    "priv" | "private" => Access::Private,
                    "pub" | "public" => Access::Public,
                    _ => {
                        eprintln!("Unknown accessibility {access:?}");
                        continue;
                    }
                };
                for (id, def) in asm.methods_with(|_, _, def| *def.access() == access) {
                    println!("{name:?} {id:?}", name = asm.get_string(def.name()));
                }
            }
            "mbyname" => {
                // Print all methods with a certain visibility
                let name = body;
                let Some(iter) = asm.find_methods_matching(name) else {
                    continue;
                };
                for id in iter {
                    println!("{id:?}");
                }
            }
            "toil" => {
                let path = body;
                let path = path.trim().trim_matches('\'').trim();
                println!("Preparing to export the assembly");
                asm.export(path, ILExporter::new(cilly::v2::IlasmFlavour::Clasic, true))
            }
            "mmakemissing" => {
                let id = parse_id(body, &asm);
                let Some(id) = asm.method_ref_to_def(id) else {
                    eprintln!("Invalid method!");
                    continue;
                };
                asm.modify_methodef(
                    |_, method| *method.implementation_mut() = MethodImpl::Missing,
                    id,
                );
            }
            "mstats" => {
                let id = parse_id(body, &asm);
                let Some(id) = asm.method_ref_to_def(id) else {
                    eprintln!("Invalid method!");
                    continue;
                };
                let def = asm.method_def(id);
                let name = asm.get_string(def.name());
                let (blocks, locals) = match def.resolved_implementation(&asm) {
                    MethodImpl::MethodBody { blocks, locals } => (blocks, locals),
                    MethodImpl::Extern {
                        lib,
                        preserve_errno,
                    } => {
                        let lib = asm.get_string(*lib);
                        eprintln!(
                            "Extern method {name} is in {lib} preserve_errno:{preserve_errno}"
                        );
                        return;
                    }
                    MethodImpl::AliasFor(_) => panic!("Unresolved method alias"),
                    MethodImpl::Missing => {
                        eprintln!("Missing method {name}");
                        return;
                    }
                };
                eprintln!(
                    "Method {name} has {locals} locals and {blocks} blocks.\n It has {roots} roots and {nodes} elements(roots + nodes).",
                    locals = locals.len(),
                    blocks = blocks.len(),
                    roots = blocks.iter().flat_map(|block|block.iter_roots()).count(),
                    nodes = blocks.iter().flat_map(|block|block.iter_roots()).flat_map(|root|CILIter::new(asm.get_root(root).clone(),&asm)).count(),
                );
            }
            "msetaccess" => {
                let mut body = body.split(',');
                let (id, access) = (parse_id(body.next().unwrap(), &asm), body.next().unwrap());
                let access = match access.as_ref() {
                    "extern" => Access::Extern,
                    "priv" | "private" => Access::Private,
                    "pub" | "public" => Access::Public,
                    _ => {
                        eprintln!("Unknown accessibility {access:?}");
                        continue;
                    }
                };

                let Some(id) = asm.method_ref_to_def(id) else {
                    eprintln!("Invalid method!");
                    continue;
                };
                asm.modify_methodef(|_, method| method.set_access(access), id);
            }
            _ => eprintln!("unknown command {cmd:?}"),
        }
    }
}
fn parse_id(id: &str, asm: &Assembly) -> MethodRefIdx {
    if let Ok(id) = id.parse::<u32>() {
        unsafe { MethodRefIdx::from_raw(NonZeroU32::new(id).unwrap()) }
    } else {
        let Some(mut iter) = asm.find_methods_matching(id) else {
            panic!("{id:?} is neithier a method name nor a method id")
        };
        *iter
            .next()
            .unwrap_or_else(|| panic!("{id:?} is neithier a method name nor a method id"))
    }
}
