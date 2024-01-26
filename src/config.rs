use lazy_static::*;
macro_rules! config_flag{
    ($var:ident,$default:expr)=>{
        lazy_static!{
            pub static ref $var:bool = {
                std::env::vars().into_iter().find_map(|(key,value)|if key == stringify!($var){Some(value)}else{None}).map(|value|match value.as_ref(){
                    "0" | "false" | "False" | "FALSE" =>false,
                    "1" | "true" | "True" | "TRUE" =>true,
                    _=>panic!("Boolean enviroment variable {} has invalid value {}",stringify!($var),value),
                }).unwrap_or($default)
            };
        }
    };
    ($var:ident,$default:expr,$comment:literal)=>{
        lazy_static!{
            #[doc = $comment]
            pub static ref $var:bool = {
                std::env::vars().into_iter().find_map(|(key,value)|if key == stringify!($var){Some(value)}else{None}).map(|value|match value.as_ref(){
                    "0" | "false" | "False" | "FALSE" =>false,
                    "1" | "true" | "True" | "TRUE" =>true,
                    _=>panic!("Boolean enviroment variable {} has invalid value {}",stringify!($var),value),
                }).unwrap_or($default)
            };
        }
    }
}
config_flag! {ABORT_ON_ERROR,false,"Should the codegen stop working when ecountering an error, or try to press on, replacing unusuported code with exceptions throws?"}
config_flag! {TEST_WITH_MONO,false}
config_flag! {TRACE_CALLS,false,"Preapends each function call with a debug message"}
config_flag! {ALWAYS_INIT_LOCALS,false,"Changes `.locals` into `.locals init`. Causes the runtime to always initialize local variables.\nTry turining on in cause of issues. If it fixes them, then their root cause is use of uninitailized memory."}
config_flag! {TRACE_STATEMENTS,false,"Preapends each statement with a debug message"}
config_flag! {INLINE_SIMPLE_FUNCTIONS,false,"Allows the optimizer to inline very simple functions. It is buggy."}
config_flag! {REMOVE_UNSUED_LOCALS,false,"Turns on the local removal optimization."}
config_flag! {SPLIT_LOCAL_STRUCTS,false,"Turns on the struct spliting optimzation."}
config_flag! {ALLOW_MISCOMPILATIONS,true,"Should the codegen continue working after it encoutnered a miscompilation?"}
config_flag! {INSERT_MIR_DEBUG_COMMENTS,false,"Tells the codegen to insert comments containing the MIR statemtens after each one of them."}
config_flag! {PRINT_LOCAL_TYPES,false,"Prints local types of all compiled MIR functions."}
config_flag! {OPTIMIZE_CIL,true,"Tells the codegen to optmize the emiited CIL."}
config_flag! {USE_CECIL_EXPORTER,false,"Tells the codegen to use the CECIL based assembly exporter"}
config_flag! {ESCAPE_NAMES,false,"Tells the codegen to escape class and method names."}
