use std::rc::Rc;

use validiter::{ErrCastable, ValidErr, ValidIter};

fn main() {
    // In this example we will use the 'cast_errs' method to
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
            // split by elements
            line.split(",")
                // trim whitespace
                .map(|s| s.trim())
                .map(|s| {
                    // map to f64
                    s.parse::<f64>()
                        // if we get a parse error, we want to map it to our own error types - ValidErr<f64>
                        .map_err(|e| ValidErr::<f64>::Description(Rc::from(format!("{e}"))))
                })
                // the iterator is over VResult<f64>, but map is not a ValidIter!
                // because 'Map' is not a 'ValidIter', we need to convert the underlying data structure type
                .cast_errs() 
                // force non-empty rows
                .at_least(1, "no columns!")
                // simple 'greater than 0' validation
                .ensure(|f| *f >= 0.0, "negative!")
                // collecting each row to a vector, but now Ok Type is a vector, and Err Type is f64!
                .collect::<Result<Vec<f64>, ValidErr<f64>>>()
        })
        // we use cast_errs again to fix the typing issues
        .cast_errs()
        // force non-empty matrix
        .at_least(1, "no rows!")
        // force equal-sized rows
        .const_over(|vec| vec.len(), "row size changed!")
        // collect into a matrix
        .collect::<Result<Vec<_>, _>>();
    assert_eq!(mat, Ok(vec![vec![1.2, 3.0], vec![4.2, 0.5]]));
    print!("{:?}", mat)
}
