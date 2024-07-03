#![feature(iter_intersperse)]
use regex::{self, Captures, Regex};
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
fn main() {
    let path = std::env::args().nth(1).unwrap();
    let tests = find_tests(&path);
    let out_path = std::env::args().nth(2).unwrap();
    if let Some("main") = std::env::args().nth(3).as_deref() {
        std::fs::File::create(out_path)
            .unwrap()
            .write_all(Test::into_main_test(tests).as_bytes())
            .unwrap();
    } else {
        std::fs::File::create(out_path)
            .unwrap()
            .write_all(Test::into_classic_tests(tests).as_bytes())
            .unwrap();
    }
}
#[derive(Debug)]
pub struct Test {
    source: String,
    features: HashSet<String>,
}
impl Test {
    pub fn new(mut source: String) -> Self {
        let pattern = Regex::new("#!\\[feature\\(([^,\\)]+(?:,[^,\\)]+)*)\\)\\]").unwrap();
        let mut features = HashSet::new();
        let replaced = pattern.replace_all(&source, |caps: &Captures| {
            features.extend(caps[1].split(',').map(|s| s.to_string()));
            String::new()
        });
        source = replaced.to_string();
        Self { source, features }
    }

    /// Finds all tests in the specified source file.
    /// ```
    /// #use crate::*;
    /// let test = Test::from_read(&include_bytes!("testgen.rs")[..]);
    /// ```
    /// ```should_panic
    /// #use crate::*;
    /// panic!();
    /// ```
    fn from_file(path: std::path::PathBuf) -> Vec<Self> {
        Self::from_read(
            std::fs::File::open(&path).unwrap(),
            path.as_os_str().to_str().unwrap(),
        )
    }
    fn from_read(read: impl Read, fname: &str) -> Vec<Self> {
        let mut lines = BufReader::new(read).lines().enumerate();
        let mut res = vec![];
        while let Some((idx, line)) = lines.next() {
            let line = line.unwrap();
            let line = line.trim();
            // Too short
            if line.len() < 3 {
                continue;
            }
            // Does not start with a doc comment
            if line.as_bytes()[..3] != *b"///" {
                continue;
            }
            // Remove the leading thingies in doc comment
            let line = line.trim_start_matches('/').trim_start();
            // Too short
            if line.len() < 3 {
                continue;
            }
            // Does not start with a test thingy
            if line.as_bytes()[..3] != *b"```" {
                continue;
            }
            let attr = &line[3..];
            let mut test_code = String::with_capacity(1024);
            // This is intenrional, and the clippy "fix" breaks the code by taking a copy of the interator instead of the iterator itself.
            #[allow(clippy::while_let_on_iterator)]
            while let Some((idx, line)) = lines.next() {
                let line = line.unwrap();
                let line = line.trim();
                // Too short
                if line.len() < 3 {
                    eprintln!("Error: doc test ends without the closing '```'. line:{line:?}, at {fname}:{idx}");
                    break;
                }
                // Does not start with a doc comment
                if line[..3].as_bytes() != *b"///" {
                    eprintln!("Error: doc test ends without the closing '```'. line:{line:?}, at {fname}:{idx}");
                    break;
                }
                // Remove the leading thingies in doc comment
                let line = line.trim_start_matches('/').trim_start();

                let line = if line.len() > 1
                    && (&line.as_bytes()[..2] == b"#!" || &line.as_bytes()[..2] == b"#[")
                {
                    line
                } else {
                    line.trim_start_matches('#')
                };
                // Too short
                if line.len() < 3 {
                    test_code.push_str(line);
                    continue;
                }
                //Starts with a test thingy, so this *should* be the end of this test!
                if line.as_bytes()[..3] == *b"```" {
                    if !attr.is_empty() && attr != "rust" {
                        eprintln!("Unsuported test attribute {attr}. Skipping.");
                        break;
                    }
                    if rust_syntax_valid(&test_code) {
                        res.push(Test::new(test_code));
                    } else {
                        eprintln!("WARNING: test invalid. line:{line:?}, at {fname}:{idx}");
                    }
                    break;
                }
                test_code.push_str(line);
                test_code.push('\n');
            }
        }
        res
    }

    fn into_classic_tests(tests: Vec<Test>) -> String {
        let mut res = String::new();
        let all_features: HashSet<_> = tests.iter().flat_map(|test| test.features.iter()).collect();
        if !all_features.is_empty() {
            res.push_str("#![feature(");
            let sep = ",".to_string();
            for feature in all_features.iter().intersperse(&&sep) {
                res.push_str(feature);
            }
            res.push_str(")]");
        }
        for (index, test) in tests.iter().enumerate() {
            res.push_str(&format!(
                "#[test]fn test{index:x}n(){{{source}}}",
                source = test.source
            ));
        }
        if let Some(fmt) = rust_format(&res) {
            res = fmt
        }
        res
    }
    fn into_main_test(tests: Vec<Test>) -> String {
        let mut res = String::new();
        let all_features: HashSet<_> = tests.iter().flat_map(|test| test.features.iter()).collect();
        if !all_features.is_empty() {
            res.push_str("#![feature(");
            let sep = ",".to_string();
            for feature in all_features.iter().intersperse(&&sep) {
                res.push_str(feature);
            }
            res.push_str(")]");
        }
        let mut main = String::new();
        main.push_str("fn main(){");
        for (index, test) in tests.iter().enumerate() {
            res.push_str(&format!(
                "fn test{index:x}n()-> impl Sized{{{source}}}",
                source = test.source
            ));
            main.push_str(&format!("println!(\"prepraing to run `test{index:x}n`\");test{index:x}n();println!(\"`test{index:x}n` OK\");",));
        }
        main.push('}');
        res.push_str(&main);
        if let Some(fmt) = rust_format(&res) {
            res = fmt
        }
        res
    }
}
fn find_tests(path: impl AsRef<Path>) -> Vec<Test> {
    let mut res = Vec::new();
    for entry in std::fs::read_dir(path).unwrap() {
        let Ok(entry) = entry else { continue };
        if entry.metadata().unwrap().is_dir() {
            res.extend(find_tests(entry.path()));
        }
        if entry.metadata().unwrap().is_file()
            && entry.path().extension().is_some_and(|ext| ext == "rs")
        {
            res.extend(Test::from_file(entry.path()));
        }
    }
    res
}
fn rust_syntax_valid(input: &str) -> bool {
    rust_format(&format!("#[test]fn testn(){{{input}}}",)).is_some()
}
fn rust_format(input: &str) -> Option<String> {
    let mut rustfmt = Command::new("rustfmt")
        .args(["--edition", "2021"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    rustfmt
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let mut stdout = Vec::with_capacity(input.len());
    rustfmt
        .stdout
        .take()
        .unwrap()
        .read_to_end(&mut stdout)
        .unwrap();
    let ecode = rustfmt.wait().expect("failed to wait on child");
    if ecode.success() {
        Some(String::from_utf8_lossy(&stdout).to_string())
    } else {
        None
    }
}
#[test]
fn test_search() {
    let test = Test::from_read(&include_bytes!("testgen.rs")[..], "testgen.rs");
}
