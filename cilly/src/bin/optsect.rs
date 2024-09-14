use std::io::{Read, Write};

use cilly::v2::{asm::ILASM_FLAVOUR, il_exporter::ILExporter, opt::OptFuel, Assembly};

fn main() {
    let asm_path = std::env::args().nth(1).expect("no cilly path");

    let mut asm_file =
        std::fs::File::open(asm_path).expect("ERROR:Could not open the assembly file!");
    let mut asm_bytes = Vec::with_capacity(0x10000);
    asm_file
        .read_to_end(&mut asm_bytes)
        .expect("ERROR: Could not load the assembly file!");
    println!("Loading an assembly");
    let asm: Assembly = postcard::from_bytes(&asm_bytes).unwrap();
    let fail = std::env::args().nth(2).expect("no fail string path");
    let args = std::env::args().nth(3).expect("no dotnet args");
    let args: Vec<&str> = args.split_whitespace().collect();
    let mut fuel_start = std::env::args()
        .nth(4)
        .map(|s| s.parse::<u32>().unwrap())
        .unwrap_or(0);
    let mut fuel_end = std::env::args()
        .nth(5)
        .map(|s| s.parse::<u32>().unwrap())
        .unwrap_or(asm.default_fuel().raw());
    while fuel_start < fuel_end {
        let fuel_mid = (fuel_start + fuel_end) / 2;
        eprintln!("Testing range {fuel_start} {fuel_end}, curr {fuel_mid}");
        let mut asm = asm.clone();
        let opt_time = std::time::Instant::now();
        asm.opt(&mut OptFuel::from_raw(fuel_mid));
        eprintln!(
            "Optimization done in {} ms, preparing to export the assembly...",
            opt_time.elapsed().as_millis()
        );
        let export_time = std::time::Instant::now();
        eprintln!("Prepraing to export.");
        let mut path = std::env::temp_dir();
        path.push("asm");
        path.set_extension("exe");
        asm.export(&path, ILExporter::new(*ILASM_FLAVOUR, false));
        eprintln!("Exported in {} ms", export_time.elapsed().as_millis());
        let mut config_path = std::env::temp_dir();
        config_path.push("asm");
        config_path.set_extension("runtimeconfig.json");
        let cfg = cilly::v2::il_exporter::get_runtime_config();
        std::fs::File::create(config_path)
            .unwrap()
            .write_all(cfg.as_bytes())
            .unwrap();
        let run_time = std::time::Instant::now();
        let out = std::process::Command::new("dotnet")
            .arg(path)
            .args(&args)
            .output()
            .unwrap();
        let stdout = std::str::from_utf8(&out.stdout).unwrap();
        let stderr = std::str::from_utf8(&out.stderr).unwrap();
        let fail = stdout.contains(&fail) || stderr.contains(&fail);
        if fail {
            fuel_end = fuel_mid;
        } else {
            fuel_start = fuel_mid;
        }
        eprintln!(
            "Run the result in in {} ms. fail:{fail}",
            run_time.elapsed().as_millis()
        );
    }
    eprintln!("Done. fuel_start:{fuel_start} fuel_end:{fuel_end}");
}
