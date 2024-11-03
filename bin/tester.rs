#!/usr/bin/env -S cargo +nightly -Zscript
#![feature(iter_intersperse)]
use core::str;
use std::collections::HashSet;
use std::path::Path;
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
fn get_test_list(exec_path: &String) -> Vec<String> {
    if *C_MODE {
        let mut cmd = std::process::Command::new(exec_path);
        cmd.arg("--list");
        let out = cmd.output().unwrap();
        let stdout = std::str::from_utf8(&out.stdout).unwrap();
        stdout
            .split('\n')
            .map(|name| {
                name.trim()
                    .rsplit_once(':')
                    .unwrap_or(("", ""))
                    .0
                    .to_owned()
            })
            .filter(|name| !name.is_empty())
            .collect()
    } else {
        let mut cmd = std::process::Command::new("dotnet");
        cmd.arg(exec_path.clone());
        cmd.arg("--list");
        let out = cmd.output().unwrap();
        let stdout = std::str::from_utf8(&out.stdout).unwrap();
        stdout
            .split('\n')
            .map(|name| {
                name.trim()
                    .rsplit_once(':')
                    .unwrap_or(("", ""))
                    .0
                    .to_owned()
            })
            .filter(|name| !name.is_empty())
            .collect()
    }
}

fn run_test(
    exec_path: &str,
    test_name: &str,
    broken: &mut Vec<String>,
    failed: &mut Vec<String>,
    successes: &mut Vec<String>,
) {
    let mut cmd = std::process::Command::new("timeout");
    cmd.arg("-k");
    cmd.arg("20");
    cmd.arg("20");
    if !*C_MODE {
        cmd.arg("dotnet");
    }

    cmd.arg(exec_path);
    cmd.arg(test_name);
    let out = cmd.output().unwrap();
    let stdout = std::str::from_utf8(&out.stdout).unwrap();
    if stdout.contains(" passed") && !stdout.contains(" 0 passed") {
        successes.push(test_name.to_owned());
        eprintln!("{test_name}: passed")
    } else if stdout.contains("1 ignored") {
        eprintln!("{test_name}: ignored")
    } else if stdout.contains(" failed") && !stdout.contains(" 0 failed") {
        failed.push(test_name.to_owned());
        eprintln!("{test_name}: failed")
    } else {
        broken.push(test_name.to_owned());
        eprintln!("{test_name}: broken")
    }
}
fn successes_from_disk(exec_name: &str) -> Vec<String> {
    let name = if *C_MODE {
        format!("c_success_{exec_name}.txt")
    } else {
        format!("success_{exec_name}.txt")
    };
    let Ok(file) = std::fs::File::open(name) else {
        return Vec::default();
    };
    use std::io::BufRead;
    std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().to_string())
        .collect()
}
fn successes_to_disk(successes: &[String], exec_name: &str) {
    use std::io::Write;
    let name = if *C_MODE {
        format!("c_success_{exec_name}.txt")
    } else {
        format!("success_{exec_name}.txt")
    };
    std::fs::File::create(&name)
        .unwrap_or_else(|err| panic!("file {name} could not be created due to {err:?}"))
        .write_all(
            successes
                .iter()
                .cloned()
                .intersperse("\n".into())
                .collect::<String>()
                .as_bytes(),
        )
        .unwrap();
}
fn main() {
    let exec_path = std::env::args().nth(1).unwrap();
    let exec_name = if exec_path.contains("coretests") {
        "coretests".to_string()
    } else if exec_path.contains("corebenches") {
        "corebenches".to_string()
    } else if exec_path.contains("core") {
        "core".to_string()
    } else if exec_path.contains("alloctests") {
        "alloctests".to_string()
    } else if exec_path.contains("allocbenches") {
        "allocbenches".to_string()
    } else if exec_path.contains("alloc") {
        "alloc".to_string()
    } else if exec_path.contains("stdtests") {
        "stdtests".to_string()
    } else if exec_path.contains("stdbenches") {
        "stdbenches".to_string()
    } else if exec_path.contains("std") {
        "std".to_string()
    } else {
        exec_path.clone()
    };
    let mut successes: Vec<String> = successes_from_disk(&exec_name);
    let mut failures: Vec<String> = Vec::default();
    let mut broken: Vec<String> = Vec::default();
    //let timeout = std::env::args().nth(2).unwrap_or("20".to_owned());
    let start = std::time::Instant::now();
    let mut test_list = get_test_list(&exec_path);
    test_list.retain(|test| !successes.contains(test));
    for (id, test) in test_list.iter().enumerate() {
        run_test(&exec_path, test, &mut broken, &mut failures, &mut successes);
        let seconds = (start.elapsed().as_secs_f64() / id as f64) * (test_list.len() as f64);
        println!(
            "Test {id} out of {}. Remaining time {seconds}s.",
            test_list.len()
        );
        successes_to_disk(&successes, &exec_name);
    }
    eprintln!("failed:{failures:?}\nbroken:{broken:?}")
    /*
    {
        let mut cmd = std::process::Command::new("timeout");
        cmd.arg("-k");
        cmd.arg(&timeout);
        cmd.arg(&timeout);
        cmd.arg("dotnet");
        cmd.arg(exec_path.clone());
    } */
}
