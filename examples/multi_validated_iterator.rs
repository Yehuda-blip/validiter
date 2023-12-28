extern crate validiter;

use validiter::valid_iter::{Unvalidatable, ValidIter};

fn main() {
    (0..10)
        .to_validation_space()
        .at_most(2)
        .at_most(2)
        .at_least(2)
        .for_each(|v| println!("{:?}", v));
}
