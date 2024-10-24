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
macro_rules! config_flag {
    ($var:ident,$default:expr) => {
        config! {$var,bool,$default}
    };
    ($var:ident,$default:expr,$comment:literal) => {
        config! {$var,bool,$default,$comment}
    };
}
config_flag! {ABORT_ON_ERROR,false,"Should the codegen stop working when ecountering an error, or try to press on, replacing unusuported code with exceptions throws?"}

config_flag! {NO_UNWIND,false,"Tells the codegen to never emmit try/catch statements."}

config_flag! {INLINE_SIMPLE_FUNCTIONS,false,"Allows the optimizer to inline very simple functions. It is buggy."}
config_flag! {REMOVE_UNSUED_LOCALS,false,"Turns on the local removal optimization."}
config_flag! {CHECK_ALLOCATIONS,false,"Turns on allocation checks/debug info."}
config_flag! {VERIFY_METHODS,false,"Typechecks all methods"}

config_flag! {SPLIT_LOCAL_STRUCTS,false,"Turns on the struct spliting optimzation."}
config_flag! {ALLOW_MISCOMPILATIONS,true,"Should the codegen continue working after it encoutnered a miscompilation?"}
config_flag! {INSERT_MIR_DEBUG_COMMENTS,false,"Tells the codegen to insert comments containing the MIR statemtens after each one of them."}
config_flag! {PRINT_LOCAL_TYPES,false,"Prints local types of all compiled MIR functions."}
config_flag! {VALIDTE_VALUES,false,"Tells the codegen to insert additional checks on each variable asigement."}
config_flag! {OPTIMIZE_CIL,true,"Tells the codegen to optmize the emiited CIL."}

config_flag! {NEW_UNSIZE,false,"Turns out the new unsizing code"}

config_flag! {ESCAPE_NAMES,false,"ells the codegen to escape class and method names."}
config_flag! {TEST_WITH_MONO,false,"Tells the codegen to use the mono runtime for tests."}

config_flag! {JS_MODE,false,"Tells the codegen to emmit JS source files."}

config_flag! {C_MODE,false,"Tells the codegen to emmit C source files."}
config_flag! {C_SANITIZE,false,"Tells the codegen sanitize C."}

config_flag! {RANDOMIZE_LAYOUT,false,"Tells the codegen to randomize TEST type layout."}
config_flag! {NATIVE_PASSTROUGH,false,"Tells the codegen compile linked static libraries into a shared library, which will be bundled with the .NET executable."}

config_flag! {ENFORCE_CIL_VALID,false,"Tells the codegen to preform additonal checks before saving the ."}

config_flag! {CHECK_REFS,false,"Tells codegen to check if references it assigns are valid."}

config_flag! {TYPECHECK_CIL,false,"Checks the geneareted CIL for type safety."}

config_flag! {TRACE_CIL_OPS,false,"Tells the print each CIL op before it is executed."}

config_flag! {DRY_RUN,false,"Tells the codegen test suite to not execute or link any test code, enabling testing on platforms without the .NET runtime present."}
