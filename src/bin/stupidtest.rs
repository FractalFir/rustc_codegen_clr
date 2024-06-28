use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug)]
struct Test {
    path: PathBuf,
    test_result: Option<Result<(), String>>,
}

impl Test {
    fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            test_result: None,
        }
    }

    fn run(&mut self, rflags: &str) {
        let out = Command::new("rustc")
            .arg(&self.path)
            .arg("-o")
            .arg(self.path.with_extension("elf"))
            .output()
            .unwrap();
        let native_err = String::from_utf8_lossy(&out.stderr).to_string();
        let out = Command::new("rustc")
            .arg(&self.path)
            .arg("-o")
            .arg(self.path.with_extension("dn"))
            .env("RUSTFLAGS", rflags)
            .output()
            .unwrap();

        let dotnet_err = String::from_utf8_lossy(&out.stderr).to_string();
        if native_err.is_empty() && !dotnet_err.is_empty() {
            self.test_result = Some(Err(dotnet_err));
        } else if native_err.is_empty() && dotnet_err.is_empty() {
            // Compilation did not fail, so we can try running the tests.
            let Ok(out) = Command::new(self.path.with_extension("elf")).output() else {
                return;
            };
            let native_err = String::from_utf8_lossy(&out.stderr).to_string();

            let out = Command::new(self.path.with_extension("dn"))
                .output()
                .unwrap();

            let dotnet_err = String::from_utf8_lossy(&out.stderr).to_string();
            eprintln!("{native_err:?} {dotnet_err:?}");
            self.test_result = Some(Ok(()));
        }
    }
}
fn find_tests(path: &Path) -> Vec<Test> {
    let mut res = Vec::new();
    for entry in std::fs::read_dir(path).unwrap() {
        let Ok(entry) = entry else { continue };
        if entry.metadata().unwrap().is_dir() {
            res.extend(find_tests(&entry.path()));
        }
        if entry.metadata().unwrap().is_file()
            && entry.path().extension().is_some_and(|ext| ext == "rs")
        {
            res.push(Test::new(entry.path()));
        }
    }
    res
}
fn main() {
    println!("Started stupid test.");
    let path = std::env::args().nth(1).expect("no search path");
    println!("Stupid test will search for tests in {path}");
    let rflags = std::env::args().nth(2).expect("no RUSTFLAGS path");
    let mut tests = find_tests(path.as_str().as_ref());
    for test in tests.iter_mut() {
        test.run(&rflags);

        /*if test.test_result.is_none() {
            continue;
        }*/
        println!("test:{test:?}")
    }
}
