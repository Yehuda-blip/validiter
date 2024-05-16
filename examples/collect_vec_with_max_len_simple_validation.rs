use validiter::{too_many, Unvalidatable, ValidIter};

extern crate validiter;

fn main() {
    let collection_failure = (0..10).validate().at_most(7, too_many!("Too many example: " plus_auto)).collect::<Result<Vec<_>, _>>();
    print!("{:?}", collection_failure)
}
