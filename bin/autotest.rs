#!/usr/bin/env -S cargo +nightly -Zscript
use core::str;
use std::collections::HashSet;
#[macro_export]
macro_rules! config {
    ($name:ident,bool,$default:expr) => {
        pub static $name: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| {
            std::env::vars()
                .find_map(|(key, value)| {
                    if key == stringify!($name) {
                        Some(value)
                    } else {
                        None
                    }
                })
                .map(|value| match value.as_ref() {
                    "0" | "false" | "False" | "FALSE" => false,
                    "1" | "true" | "True" | "TRUE" => true,
                    _ => panic!(
                        "Boolean enviroment variable {} has invalid value {}",
                        stringify!($name),
                        value
                    ),
                })
                .unwrap_or($default)
        });
    };
    ($name:ident,bool,$default:expr,$comment:literal) => {
        #[doc = $comment]
        pub static $name: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| {
            std::env::vars()
                .find_map(|(key, value)| {
                    if key == stringify!($name) {
                        Some(value)
                    } else {
                        None
                    }
                })
                .map(|value| match value.as_ref() {
                    "0" | "false" | "False" | "FALSE" => false,
                    "1" | "true" | "True" | "TRUE" => true,
                    _ => panic!(
                        "Boolean enviroment variable {} has invalid value {}",
                        stringify!($name),
                        value
                    ),
                })
                .unwrap_or($default)
        });
    };
}
config!(C_MODE, bool, false);
fn main() {
    let exec_path = std::env::args().nth(1).unwrap();
    let mut ok: HashSet<String> = HashSet::default();
    let mut failures: HashSet<String> = HashSet::default();
    let mut broken: Vec<String> = Vec::default();
    let timeout = std::env::args().nth(2).unwrap_or("20".to_owned());
    let mut shuffles = 4;
    loop {
        let mut cmd = std::process::Command::new("timeout");
        cmd.arg("-k");
        cmd.arg(&timeout);
        cmd.arg(&timeout);
        if !*C_MODE {
            cmd.arg("dotnet");
        }

        cmd.arg(exec_path.clone());
        if shuffles > 0 {
            cmd.arg("--shuffle");
            cmd.arg("-Z");
            cmd.arg("unstable-options");
        }
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
        eprintln!("stderr:{stderr}");
        let stdout = str::from_utf8(&out.stdout).unwrap().to_string();
        for line in stdout.lines() {
            if !line.contains("test ") || line.contains("finished in") {
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
            } else if status.contains("ignored") {
                continue;
            } else if !broken.iter().any(|exisitng| exisitng == name) {
                broken.push(name.to_owned());
            }
        }
        if stderr.contains("failures:")
            | stdout.contains("failures:")
            | stderr.contains("finished")
            | stdout.contains("finished")
        {
            if shuffles > 0 {
                shuffles -= 1;
            } else {
                println!("search done.");
                break;
            }
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
    if *C_MODE {
        let mut cmd = std::process::Command::new(exec_path.clone());

        cmd.args(broken.iter().flat_map(|arg| ["--skip", arg]));
        println!("{cmd:?}");
    } else {
        let mut cmd = std::process::Command::new("dotnet");
        cmd.arg(exec_path.clone());
        cmd.args(broken.iter().flat_map(|arg| ["--skip", arg]));
        println!("{cmd:?}");
    }
    println!(
        "\nsearch result: ok:{ok}, failures:{failures} broken:{broken}",
        ok = ok.len(),
        failures = failures.len(),
        broken = broken.len()
    );
}
