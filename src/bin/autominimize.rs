use std::io::BufReader;
use std::{
    io::Write,
    io::{BufRead, Read},
};
struct RustSourceFile {
    lines: Vec<String>,
    is_removed: Vec<bool>,
}
impl RustSourceFile {
    fn from_file(mut src: impl Read + BufRead) -> std::io::Result<Self> {
        let mut lines = Vec::new();
        let mut is_removed = Vec::new();
        for line in src.lines() {
            lines.push(line?);
            is_removed.push(false);
        }
        Ok(Self { lines, is_removed })
    }
    fn into_file(&self, mut w: impl Write) -> std::io::Result<()> {
        for (line, is_removed) in self.lines.iter().zip(self.is_removed.iter()) {
            if !is_removed {
                writeln!(w, "{line}");
            }
        }
        Ok(())
    }
    fn remove_line(&mut self, line: usize) {
        self.is_removed[line] = true;
    }
    fn restore_line(&mut self, line: usize) {
        self.is_removed[line] = false;
    }
    fn try_remove_line(&mut self, line: usize, test: &impl Fn(&Self) -> bool) {
        self.remove_line(line);
        let can_remove = test(self);
        println!("Trying to remove line {line} can_remove:{can_remove}");
        if !can_remove {
            self.restore_line(line);
        }
    }
    fn try_remove_lines(&mut self, test: &impl Fn(&Self) -> bool) {
        let line_count = self.lines.len();
        //let line_count = line_count.min(100);
        for index in 0..(line_count - 2) {
            if !self.is_removed[index] {

                //self.try_remove_n_lines(index, 2,test);
                //self.try_remove_n_lines(index, 1,test);
                self.try_remove_line(index,test);
                
               
            }
        }
    }
}
fn main() {
    let source_path = "/home/michal/Rust/rustc_codegen_clr/test/fuzz/fuzz0.rs";
    let target_path = "/home/michal/Rust/rustc_codegen_clr/test/fuzz/minfuzz0.rs";
    let out_path = "/home/michal/Rust/rustc_codegen_clr/test/fuzz/minfuzz0.exe";
    let exec_path = "/home/michal/Rust/rustc_codegen_clr/test/minfuzz0.exe";
    let file = BufReader::new(std::fs::File::open(source_path).unwrap());
    let mut source_file = RustSourceFile::from_file(file).unwrap();
    source_file.try_remove_lines(&|source_file| {
        source_file
            .into_file(std::fs::File::create(target_path).unwrap())
            .unwrap();
        let out = std::process::Command::new("rustc")
            //.env("RUST_TARGET_PATH","../../")
            .args([
                target_path,
                "--edition",
                "2021",
                //"--target",
                //"clr64-unknown-clr"
            ])
            .output()
            .expect("failed to execute process");
        if !out.stderr.is_empty() {
            let stdout =
                String::from_utf8(out.stdout).expect("rustc error contained non-UTF8 characters.");
            let stderr =
                String::from_utf8(out.stderr).expect("rustc error contained non-UTF8 characters.");

            if stderr.contains("error") {
                return false;
            }
        }
        // Compiles the test project
        let mut cmd = std::process::Command::new("rustc");
        //.env("RUST_TARGET_PATH","../../")
        cmd.args([
            "-O",
            "-Z",
            &format!(
                "codegen-backend={}",
                rustc_codegen_clr::compile_test::absolute_backend_path()
                    .display()
                    .to_string()
            ),
            "-C",
            &format!(
                "linker={}",
                rustc_codegen_clr::compile_test::RUSTC_CODEGEN_CLR_LINKER.display()
            ),
            target_path,
            "-o",
            exec_path,
            "--edition",
            "2021",
            //"--target",
            //"clr64-unknown-clr"
        ]);
        let out = cmd.output().expect("failed to execute process");
        // If stderr is not empty, then something went wrong, so print the stdout and stderr for debuging.
        if !out.stderr.is_empty() {
            let stdout =
                String::from_utf8(out.stdout).expect("rustc error contained non-UTF8 characters.");
            let stderr =
                String::from_utf8(out.stderr).expect("rustc error contained non-UTF8 characters.");
            //panic!("stdout:\n{stdout}\nstderr:\n{stderr}");
        }
        let mut dotnet = std::process::Command::new("dotnet")
            .arg(exec_path)
            .output()
            .expect("failed to execute process");
        
        if !dotnet.stderr.is_empty() {
            println!("dotnet.stderr is not empty!");
            let stdout =
                String::from_utf8(dotnet.stdout).expect("rustc error contained non-UTF8 characters.");
            let stderr =
                String::from_utf8(dotnet.stderr).expect("rustc error contained non-UTF8 characters.");

            if stderr.contains("System.NullReferenceException") && stderr.contains("at RustModule._ZN8minfuzz03fn717hafc8e24c8516f781E")
               
            {
                return true;
            } else {
                println!("dotnet.stderr:{stderr:?}");
                return false;
            }
        }
       
        return false;
    });

    source_file
        .into_file(std::fs::File::create(target_path).unwrap())
        .unwrap();
}
