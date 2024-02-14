use std::io::Write;
static DOTNET_ASSEMBLY:&[u8] = include_bytes!("{exec_file}");
static RUNTIME_COFIG:&[u8] = b"{{
    \"runtimeOptions\": {{
      \"tfm\": \"netcoreapp3.1\",
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
fn main(){{
    let curr_path = std::env::current_exe().unwrap();
    let dll_path = curr_path.with_extension("dll");
    let config = curr_path.with_extension("runtimeconfig.json");

    if !dll_path.exists(){{
        let mut file = std::fs::File::create(dll_path.clone()).expect("Could not create a file to unpack the .NET assembly");
        file.write_all(DOTNET_ASSEMBLY).expect("Could not unpack the .NET assembly");
    }}
    if !config.exists(){{
        let mut file = std::fs::File::create(config).expect("Could not create a file to save .NET runtime settings.");
        file.write_all(RUNTIME_COFIG).expect("Could not save .NET runtime settings");
    }}
    std::process::Command::new("dotnet").arg(dll_path).status().expect("Could not start the .NET runtime.");
}}