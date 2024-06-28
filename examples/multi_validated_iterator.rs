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
        .at_most(7, "too many!")
        .between(2, 8, "out of bounds!")
        .ensure(|i| i % 2 == 0, "odd!")
        .at_least(4, "not enough!")
        .for_each(|v| println!("{:?}", v));
}
