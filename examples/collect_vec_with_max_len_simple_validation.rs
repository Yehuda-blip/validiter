use validiter::{Unvalidatable, ValidIter};

extern crate validiter;

fn main() {
    let collection_failure = (0..10)
        .validate()
        .at_most(7, "too many!")
        .collect::<Result<Vec<_>, _>>();
    print!("{:?}", collection_failure)
}
