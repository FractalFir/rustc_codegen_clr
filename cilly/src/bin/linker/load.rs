use ar::Archive;
use cilly::asm::Assembly;
use cilly::IString;
use std::io::Read;
pub struct LinkableFile {
    name: IString,
    file: Box<[u8]>,
}

impl LinkableFile {
    pub fn new(name: IString, file: Box<[u8]>) -> Self {
        Self { name, file }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn file(&self) -> &[u8] {
        &self.file
    }
}
fn load_ar(
    r: &mut impl std::io::Read,
) -> std::io::Result<(cilly::v2::Assembly, Vec<LinkableFile>)> {
    let mut final_assembly = cilly::v2::Assembly::default();
    let mut archive = Archive::new(r);
    let mut linkables = Vec::new();
    // Iterate over all entries in the archive:
    while let Some(entry_result) = archive.next_entry() {
        let mut entry = entry_result?;
        let name: String = String::from_utf8_lossy(entry.header().identifier()).into();
        let ext = if let Some(ext) = name.split('.').last() {
            ext
        } else {
            continue;
        };
        if ext.contains("bc") || ext.contains("cilly") {
            let mut asm_bytes = Vec::with_capacity(0x100);
            entry
                .read_to_end(&mut asm_bytes)
                .expect("ERROR: Could not load the assembly file!");
            let assembly = postcard::from_bytes(&asm_bytes)
                .unwrap_or_else(|_| panic!("ERROR:Could not decode the assembly file {name}!"));
            final_assembly = final_assembly.link(assembly);
        } else if ext.contains("o") {
            let mut file_bytes = Vec::with_capacity(0x100);
            entry
                .read_to_end(&mut file_bytes)
                .expect("ERROR: Could not load the assembly file!");
            linkables.push(LinkableFile::new(name.clone().into(), file_bytes.into()));
        } else if name.contains(".so") {
            eprintln!("shr:{name}");
        }
    }
    Ok((final_assembly, linkables))
}
pub fn load_assemblies(
    raw_files: &[&String],
    archives: &[String],
) -> (cilly::v2::Assembly, Vec<LinkableFile>) {
    println!("Preparing to load assmeblies");
    let mut final_assembly = cilly::v2::Assembly::default();
    let mut linkables = Vec::new();
    for asm_path in raw_files {
        let mut asm_file =
            std::fs::File::open(asm_path).expect("ERROR:Could not open the assembly file!");
        let mut asm_bytes = Vec::with_capacity(0x10000);
        asm_file
            .read_to_end(&mut asm_bytes)
            .expect("ERROR: Could not load the assembly file!");
        let asm: cilly::v2::Assembly =
            postcard::from_bytes(&asm_bytes).expect("ERROR:Could not decode the assembly file!");

        final_assembly = final_assembly.link(asm);
    }
    for asm_path in archives {
        let mut asm_file =
            std::fs::File::open(asm_path).expect("ERROR: Could not open the assembly file!");
        let (asm, linkable) = load_ar(&mut asm_file).expect("Could not open archive");
        final_assembly = final_assembly.link(asm);
        linkables.extend(linkable);
    }
    println!("Loaded assmeblies");
    (final_assembly, linkables)
}
