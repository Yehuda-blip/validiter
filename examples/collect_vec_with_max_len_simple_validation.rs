extern crate validiter;

use validiter::simple_validations::validated_iterator::ValidatedIterator;

fn main() {
    let collection_failure = (0..10).at_most(7).collect::<Result<Vec<_>, _>>();
    print!("{:?}", collection_failure)
}
