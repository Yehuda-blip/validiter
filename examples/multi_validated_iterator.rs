extern crate validiter;

use validiter::complex_validations::valid_iter::{Unvalidatable, ValidIter};

fn main() {
    (0..10).to_validation_space().at_most(2);
}