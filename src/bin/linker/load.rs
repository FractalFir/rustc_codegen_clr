use ar::Archive;
use rustc_codegen_clr::assembly::Assembly;
use std::io::Read;

fn load_ar(r: &mut impl std::io::Read) -> std::io::Result<Assembly> {
    let mut final_assembly = Assembly::empty();
    let mut archive = Archive::new(r);
    // Iterate over all entries in the archive:
    while let Some(entry_result) = archive.next_entry() {
        let mut entry = entry_result.unwrap();
        let name = String::from_utf8_lossy(entry.header().identifier());
        if name.contains(".bc") {
            let mut asm_bytes = Vec::with_capacity(0x100);
            entry
                .read_to_end(&mut asm_bytes)
                .expect("ERROR: Could not load the assembly file!");
            let assembly = postcard::from_bytes(&asm_bytes)
                .expect("ERROR:Could not decode the assembly file!");
            final_assembly = final_assembly.join(assembly);
        }
    }
    Ok(final_assembly)
}
pub fn load_assemblies(raw_files: &[&String], archives: &[&String]) -> Assembly {
    let mut final_assembly = Assembly::empty();
    for asm_path in raw_files {
        let mut asm_file =
            std::fs::File::open(asm_path).expect("ERROR:Could not open the assembly file!");
        let mut asm_bytes = Vec::with_capacity(0x100);
        asm_file
            .read_to_end(&mut asm_bytes)
            .expect("ERROR: Could not load the assembly file!");
        let assembly =
            postcard::from_bytes(&asm_bytes).expect("ERROR:Could not decode the assembly file!");
        final_assembly = final_assembly.join(assembly);
    }
    for asm_path in archives {
        let mut asm_file =
            std::fs::File::open(asm_path).expect("ERROR: Could not open the assembly file!");
        let assembly = load_ar(&mut asm_file).expect("Could not open archive");
        final_assembly = final_assembly.join(assembly);
    }
    final_assembly
}
