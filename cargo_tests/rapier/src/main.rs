#![feature(test)]
extern crate test;
use test::test::parse_opts;
use test::*;
fn main() {
    let fake_args = vec![String::new()];
    let opts = parse_opts(&fake_args).unwrap().unwrap();

    run_tests_console(&opts, one_ignored_one_unignored_test());
}
fn one_ignored_one_unignored_test() -> Vec<TestDescAndFn> {
    vec![
        /*TestDescAndFn {
            desc: TestDesc {
                name: StaticTestName("1"),
                ignore: true,
                ignore_message: None,
                source_file: "",
                start_line: 0,
                start_col: 0,
                end_line: 0,
                end_col: 0,
                should_panic: ShouldPanic::No,
                compile_fail: false,
                no_run: false,
                test_type: TestType::Unknown,
            },
            testfn: DynTestFn(Box::new(move || Ok(()))),
        },*/
        TestDescAndFn {
            desc: TestDesc {
                name: StaticTestName("2"),
                ignore: false,
                ignore_message: None,
                source_file: "",
                start_line: 0,
                start_col: 0,
                end_line: 0,
                end_col: 0,
                should_panic: ShouldPanic::No,
                compile_fail: false,
                no_run: false,
                test_type: TestType::Unknown,
            },
            testfn: DynTestFn(Box::new(move || Ok(()))),
        },
        /*TestDescAndFn {
            desc: TestDesc {
                name: StaticTestName("3"),
                ignore: false,
                ignore_message: None,
                source_file: "",
                start_line: 0,
                start_col: 0,
                end_line: 0,
                end_col: 0,
                should_panic: ShouldPanic::No,
                compile_fail: false,
                no_run: false,
                test_type: TestType::Unknown,
            },
            testfn: DynTestFn(Box::new(move || Ok(()))),
        },
        TestDescAndFn {
            desc: TestDesc {
                name: StaticTestName("4"),
                ignore: false,
                ignore_message: None,
                source_file: "",
                start_line: 0,
                start_col: 0,
                end_line: 0,
                end_col: 0,
                should_panic: ShouldPanic::No,
                compile_fail: false,
                no_run: false,
                test_type: TestType::Unknown,
            },
            testfn: DynTestFn(Box::new(move || Ok(()))),
        },*/
    ]
} /*
  #[test]
  fn test() {}*/
