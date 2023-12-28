extern crate validiter;

use validiter::valid_iter::{Unvalidatable, ValidIter};

fn main() {
    (0..10)
        .validate()
        .at_most(7)
        .between(2, 8)
        .ensure(|i| i % 2 == 0)
        .at_least(4)
        .for_each(|v| println!("{:?}", v));
}
