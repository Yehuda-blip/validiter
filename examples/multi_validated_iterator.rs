extern crate validiter;

use validiter::{Unvalidatable, ValidIter};

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
        .at_most(7)
        .between(2, 8)
        .ensure(|i| i % 2 == 0)
        .at_least(4)
        .for_each(|v| println!("{:?}", v));
}
