use validiter::{broken_const, invalid, too_few, ErrLiftable, ValidErr, ValidIter};

fn main() {
    // In this example we will use the 'lift_errs' method to
    // create a 'Vec<Vec<f64>>' collection, while ensuring
    // the mathematical validity if this collection as a numerical
    // matrix. We will also force the matrix to be non-negative,
    // just for funsies.

    // this is a CSV format str, with 2 rows and 2 columns
    let csv = "1.2, 3.0
                4.2, 0.5";

    // we'll use iterator methods on the CSV to build an actual
    // split the csv by rows/lines
    let mat = csv
        .lines()
        // convert each row to a matrix row
        .map(|line| {
            line.split(",")
                .map(|s| s.trim())
                // if we get a parse error, we want to map it to our own error types - ValidErr<f64>
                .map(|s| {
                    s.parse::<f64>()
                        .map_err(|pe| ValidErr::<f64>::Mapped(format!("{pe}")))
                })
                // because 'Map' is not a 'ValidIter', we need to convert the underlying data structure type
                .lift_errs()
                .at_least(1, too_few!("Not enough elements in row: " plus_auto))
                .ensure(
                    |f| *f >= 0.0,
                    invalid!("Negative value encountered: " plus_auto),
                )
                // collecting each row to a vector, but now Ok Type is a vector, and Err Type is f64!
                .collect::<Result<Vec<f64>, ValidErr<f64>>>()
        })
        // we use lift_errs again to fix the typing issues
        .lift_errs()
        .at_least(1, too_few!("must have rows in matrix"))
        .const_over(
            |vec| vec.len(),
            broken_const!("All rows must be of the same length: " plus_auto_debug),
        )
        .collect::<Result<Vec<_>, _>>();
    assert_eq!(mat, Ok(vec![vec![1.2, 3.0], vec![4.2, 0.5]]));
    print!("{:?}", mat)
}
