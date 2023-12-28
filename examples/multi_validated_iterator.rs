extern crate validiter;

use validiter::valid_iter::{Unvalidatable, ValidIter};

fn main() {
    (0..10)
        .validate()
        .at_most(2)
        .at_most(2)
        .at_least(2)
        .between(4, 8)
        .ensure(|i| i % 2 == 0)
        .for_each(|v| println!("{:?}", v));
}
