extern crate validiter;

use validiter::{invalid, out_of_bounds, too_few, Unvalidatable, ValidIter};

fn main() {
    // This is the standard way to use validiter - call validate on
    // some 'Unvalidatable' iterator, and then place restrictions
    // on the iteration. Notice that 'ValidErr' type errors are always
    // ignored by validiter adapters, so the order of validation
    // placement matters, if the iteration fails - there might be
    // ignored errors, on elements that already failed a different
    // validation.
    (0..10)
        .validate()
        .at_most(7, |elmt, index, max_len| format!("oops, an iteration with at most {max_len} elements can't have a {index}-th element, but we found {elmt}!"))
        .between(2, 8, out_of_bounds!("dammit: " plus_auto))
        .ensure(|i| i % 2 == 0, invalid!(""))
        .at_least(4, too_few!("just not quite enough... " plus_auto))
        .for_each(|v| println!("{:#?}", v));
}
