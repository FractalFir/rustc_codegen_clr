#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    let_chains,
    never_type,
    unsized_const_params,
    pointer_is_aligned_to
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
/// Name of an option. Either a string or a single char.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Name {
    /// A string representing the long name of an option.
    /// For example: "help"
    Long(String),
    /// A char representing the short name of an option.
    /// For example: 'h'
    Short(char),
}
/// A description of a possible option.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Opt {
    /// Name of the option
    name: Name,
}
fn find_opt(opts: &[Opt], nm: &Name) -> Option<usize> {
    // Search main options.
    let pos = opts.iter().position(|opt| &opt.name == nm);
    if pos.is_some() {
        return pos;
    }

    None
}
fn main() {
    test_eq!(
        find_opt(
            &[
                Opt {
                    name: Name::Short('b'),
                },
                Opt {
                    name: Name::Short('a'),
                },
            ],
            &Name::Short('a'),
        ),
        Some(1)
    );
}
