use std::io::Write;
static DOTNET_ASSEMBLY:&[u8] = include_bytes!("{exec_file}");
static RUNTIME_COFIG:&[u8] = b"{{
    \"runtimeOptions\": {{
      \"tfm\": \"net8.0\",
      \"framework\": {{
        \"name\": \"Microsoft.NETCore.App\",
        \"version\": \"8.0.1\"
      }},
      \"configProperties\": {{
        \"System.Threading.ThreadPool.MinThreads\": 4,
        \"System.Threading.ThreadPool.MaxThreads\": 25
      }}
    }}
}}";
macro_rules! include_bytes_if{{
  (true,$path:literal)=>{{
      include_bytes!($path)
  }};
  (false,$path:literal)=>{{b""}};
}}

static BUNDLED_SHARED_LIB:&[u8] = include_bytes_if!({has_native_companion},"{native_companion_file}");
static BUNDLED_PDB:&[u8] = include_bytes_if!({has_pdb},"{pdb_file}");
fn main(){{
    let curr_path = std::env::current_exe().unwrap();
    let dll_path = curr_path.with_extension("dll");
    let config = curr_path.with_extension("runtimeconfig.json");
    let pdb_file = curr_path.with_file_name("{pdb_file}");
    let mut requires_refresh = false;
    if dll_path.exists() {{
    	let ondisk_len = std::fs::File::open(dll_path.clone()).expect("Could not create a file to unpack the .NET assembly").metadata().unwrap().len();
    	// If the length on disk is != expected, write the new file TODO: this can very rarely not detect if an update is needed. Check assembly GUID too, to ensure recompilation works.
    	if ondisk_len != DOTNET_ASSEMBLY.len() as u64{{
    		requires_refresh = true;
    	}}
    }}
    if !dll_path.exists() || requires_refresh {{
        let mut file = std::fs::File::create(dll_path.clone()).expect("Could not create a file to unpack the .NET assembly");
        file.write_all(DOTNET_ASSEMBLY).expect("Could not unpack the .NET assembly");
    }}
    if !config.exists() ||requires_refresh {{
        let mut file = std::fs::File::create(config).expect("Could not create a file to save .NET runtime settings.");
        file.write_all(RUNTIME_COFIG).expect("Could not save .NET runtime settings");
    }}
    if {has_native_companion} {{
      if !std::path::Path::new("{native_companion_file}").exists() || requires_refresh{{
          let mut file = std::fs::File::create("{native_companion_file}").expect("Could not create a file to provide the native companion.");
          file.write_all(BUNDLED_SHARED_LIB).expect("Could create a file to provide the native companion");
      }}
  
    }}
    if {has_pdb}{{
      if !pdb_file.exists() || requires_refresh{{
          println!("creating the pdb file");
          let mut file = std::fs::File::create(pdb_file).expect("Could not create a file to provide the pdb debug info.");
          file.write_all(BUNDLED_PDB).expect("Could create a file to provide the pdb debug info.");
      }}
      else{{
          println!("Not creating the pdb file");
      }}
    }}
    let args:Vec<String> = std::env::args().collect();
    let args = &args[1..];
    std::process::Command::new("{jumpstart_cmd}").arg(dll_path).args(args).status().expect("Could not start the .NET runtime.");
}}

