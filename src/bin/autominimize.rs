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
    pub fn from_file(mut src: impl Read + BufRead) -> std::io::Result<Self> {
        let mut lines = Vec::new();
        let mut is_removed = Vec::new();
        for line in src.lines() {
            lines.push(line?);
            is_removed.push(false);
        }
        Ok(Self { lines, is_removed })
    }
    pub fn into_file(&self, mut w: impl Write) -> std::io::Result<()> {
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
    fn try_remove_line(&mut self, line: usize, is_equivalent: &impl Fn(&Self) -> bool) {
        if self.is_removed[line] {
            return;
        }
        self.remove_line(line);
        let can_remove = is_equivalent(self);
        if !can_remove {
            self.restore_line(line);
        } else {
            println!("Removed line {line}.");
        }
    }

    pub fn try_remove_lines(&mut self, is_equivalent: &impl Fn(&Self) -> bool) {
        let line_count = self.lines.len();
        // For time estimates
        let start = std::time::Instant::now();
        for index in 0..line_count {
            self.try_remove_line(index, is_equivalent);
            let time_per_line = (start.elapsed().as_millis() as f64 / 1000.0) / (index as f64);
            let estimate_sec = time_per_line * (line_count - index) as f64;
            println!("Trying to remove line {index}. Progress:{:.2}% tpl:{time_per_line:.2}s. Remaining {estimate_sec:.2}s",(index as f64/line_count as f64)*100.0);
        }
    }
    pub fn lines(&self) -> impl Iterator<Item = &String> {
        self.lines
            .iter()
            .zip(self.is_removed.iter())
            .filter_map(|(line, is_removed)| if *is_removed { None } else { Some(line) })
    }
}
/*fn try_remove_n_lines(
    &mut self,
    line: usize,
    max_combo_len: usize,
    is_equivalent: &impl Fn(&Self) -> bool,
) {
    for offset in 0..max_combo_len {
        if self.is_removed[line + offset] {
            return;
        }
    }
    for offset in 0..max_combo_len {
        self.remove_line(line + offset);
    }

    let can_remove = is_equivalent(self);
    //println!("Trying to remove line {line} can_remove:{can_remove}");
    if !can_remove {
        for offset in 0..max_combo_len {
            self.restore_line(line + offset);
        }
    }
} */
fn is_valid(source_file: &RustSourceFile, target_path: &str) -> bool {
    let mut par_ballance = 0;
    let mut bra_ballance = 0;
    let mut in_string = false;
    for c in source_file.lines().flat_map(|line| line.chars()) {
        match c {
            '(' => {
                if !in_string {
                    par_ballance += 1
                }
            }
            ')' => {
                if !in_string {
                    par_ballance -= 1
                }
            }
            '{' => {
                if !in_string {
                    bra_ballance += 1
                }
            }
            '}' => {
                if !in_string {
                    bra_ballance -= 1
                }
            }
            '"' => in_string = !in_string,
            _ => (),
        }
    }
    if par_ballance != 0 || bra_ballance != 0 {
        return false;
    }
    source_file
        .into_file(std::fs::File::create(target_path).unwrap())
        .unwrap();
    let out = std::process::Command::new("rustc")
        .args([target_path, "--edition", "2021"])
        .output()
        .expect("failed to execute process");
    if !out.stderr.is_empty() {
        let stdout =
            String::from_utf8(out.stdout).expect("rustc error contained non-UTF8 characters.");
        let stderr =
            String::from_utf8(out.stderr).expect("rustc error contained non-UTF8 characters.");

        if stderr.contains("error") {
            println!("rustc error");
            return false;
        }
    }
    true
}
fn is_miri_happy(crate_path: &str, target_path: &str) -> bool {
    let out = std::process::Command::new("timeout")
        .env("MIRIFLAGS", "-Zmiri-tree-borrows")
        .current_dir(crate_path)
        .args([
            "-v",
            "2",
            "cargo",
            "miri",
            "run",
            "--target",
            "x86_64-win7-windows-msvc",
        ])
        .output()
        .expect("failed to execute process");
    if !out.stderr.is_empty() {
        let stdout =
            String::from_utf8(out.stdout).expect("rustc error contained non-UTF8 characters.");
        let stderr =
            String::from_utf8(out.stderr).expect("rustc error contained non-UTF8 characters.");

        if stderr.contains("error") || stderr.contains("sending signal") {
            println!("miri unhappy :(.");
            return false;
        }
    }
    true
}
/* */
fn main() {
    let source_path = "/home/michal/Rust/rustc_codegen_clr/test/fuzz/minfuzz/src/original.rs";
    let target_path = "/home/michal/Rust/rustc_codegen_clr/test/fuzz/minfuzz/src/main.rs";
    let crate_path = "/home/michal/Rust/rustc_codegen_clr/test/fuzz/minfuzz/";
    let out_path = "/home/michal/Rust/rustc_codegen_clr/test/fuzz/minfuzz0.exe";
    let exec_path = "/home/michal/Rust/rustc_codegen_clr/test/minfuzz0.exe";
    let file = BufReader::new(std::fs::File::open(source_path).unwrap());
    let mut source_file = RustSourceFile::from_file(file).unwrap();
    source_file.try_remove_lines(&|source_file| {
        if !is_valid(source_file, target_path) {
            return false;
        }
        if !is_miri_happy(crate_path, target_path) {
            return false;
        }
        // Compiles the test project
        let mut cmd = std::process::Command::new("rustc");
        //.env("RUST_TARGET_PATH","../../")
        cmd.env("TRACE_STATEMENTS","1").args([
            //"-O",
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
            out_path,
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
        let mut dotnet = std::process::Command::new("timeout")
            .arg("-v")
            .arg("5")
            .arg("dotnet")
            .arg(exec_path)
            .output()
            .expect("failed to execute process");

        if !dotnet.stderr.is_empty() {
            println!("dotnet.stderr is not empty!");
            let stdout = String::from_utf8(dotnet.stdout)
                .expect("rustc error contained non-UTF8 characters.");
            let stderr = String::from_utf8(dotnet.stderr)
                .expect("rustc error contained non-UTF8 characters.");
            if stderr.contains("sending signal") {
                println!("dotnet timemout");
                return false;
            }
            if stderr.contains("System.NullReferenceException")
                && stderr.contains("(IntPtr , Int128 , Tuple2is , SByte* , Double , Int128 , Tuple2is , UInt128 , Tuple3f64 , UInt64 , Tuple3pi8 , Tuple13Tuple7u32u128 )")
                && stdout.contains("_3 = (_1,)")
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
