use validiter::{lift_errs::ErrLiftable, valid_iter::ValidIter, valid_result::ValidErr};

fn main() {
    let csv = "1.2, 3.0
               4.2, -0.5";
    let mat = csv
        .lines()
        .map(
            |line| {
                line.split(",")
                    .map(|s| s.trim())
                    .map(|s| s.parse::<f64>().map_err(|_| ValidErr::<f64>::Mapped))
                    .lift_errs() // the iterator is over VResult<f64>, but map is not a ValidIter!
                    .ensure(|f| *f >= 0.0)
                    .collect::<Result<Vec<f64>, ValidErr<f64>>>()
            }, // OkType is a vector, but ErrType is f64!
        )
        .lift_errs()
        .collect::<Result<Vec<_>, _>>(); // now ErrType is also a Vec<f64>
    assert_eq!(mat, Err(ValidErr::Lifted)); // the element at pos [1][1] would have been negative, failing the ensure validation
}
