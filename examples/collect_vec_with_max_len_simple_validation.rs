extern crate validiter;
use validiter::AtMost;


struct TooMany(i32);

fn main() {
    let collection_failure = (0..10)
        .map(|i| Ok(i))
        .at_most(7, |i| TooMany(i))
        .collect::<Result<Vec<_>, _>>();
    match collection_failure {
        Ok(_vector) => unreachable!(),
        Err(TooMany(v)) => print!("{v} is the first value obtained after reaching limit")
    }
}
