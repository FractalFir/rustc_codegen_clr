use core::str;
use std::collections::HashSet;

fn main() {
    let exec_path = std::env::args().nth(1).unwrap();
    let mut ok: HashSet<String> = HashSet::default();
    let mut failures: HashSet<String> = HashSet::default();
    let mut broken: Vec<String> = Vec::default();

    loop {
        let mut cmd = std::process::Command::new("timeout");
        cmd.arg("-k");
        cmd.arg("20");
        cmd.arg("20");
        cmd.arg("dotnet");
        cmd.arg(exec_path.clone());
        cmd.arg("--test-threads=1");
        cmd.args(broken.iter().flat_map(|arg| ["--skip", arg]));
        println!("\n{cmd:?}");
        println!(
            "\nsearch status: ok:{ok}, failures:{failures} broken:{broken}",
            ok = ok.len(),
            failures = failures.len(),
            broken = broken.len()
        );
        let out = cmd.output().unwrap();
        let stderr = str::from_utf8(&out.stderr).unwrap().to_string();
        let stdout = str::from_utf8(&out.stdout).unwrap().to_string();
        for line in stdout.lines() {
            if !line.contains("test ") {
                continue;
            }
            let mut split = line.split("...");

            let name = match split.next() {
                Some(some) => some,
                None => continue,
            };
            let name = name["test ".len()..].trim();
            let name = name.split("-").next().unwrap().trim();
            let status = split.next().unwrap_or("");
            if status.contains("ok") {
                ok.insert(name.to_owned());
            } else if status.contains("FAILED") {
                failures.insert(name.to_owned());
            } else {
                if !broken.iter().any(|exisitng| exisitng == name) {
                    broken.push(name.to_owned());
                }
            }
        }
        if stderr.contains("failures:")
            | stdout.contains("failures:")
            | stderr.contains("finished")
            | stdout.contains("finished")
        {
            println!("search done.");
            break;
        }
    }
    println!();
    println!("BROKEN:");
    for broken in &broken {
        println!("{broken}");
    }
    println!("FAILURES:");
    for faliure in &failures {
        println!("{faliure}");
    }
    println!("COMMAND:");
    let mut cmd = std::process::Command::new("dotnet");
    cmd.arg(exec_path.clone());
    cmd.args(broken.iter().flat_map(|arg| ["--skip", arg]));
    println!("{cmd:?}");
    println!(
        "\nsearch result: ok:{ok}, failures:{failures} broken:{broken}",
        ok = ok.len(),
        failures = failures.len(),
        broken = broken.len()
    );
}
