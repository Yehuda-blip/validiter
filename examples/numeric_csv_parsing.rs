use std::{num::ParseFloatError, vec};

use validiter::{AtLeast, ConstOver};

fn main() {
    // In this example we will use the 'cast_errs' method to
    // create a 'Vec<Vec<f64>>' collection, while ensuring
    // the mathematical validity if this collection as a numerical
    // matrix.

    // Here we define the errors we expect to encounter in
    // the parsing process:
    #[derive(Debug)]
    enum MatParseErr {
        NotAFloat(usize, usize, ParseFloatError),
        NoColumns(usize),
        NoRows,
        JaggedArray(usize, Vec<f64>, usize, usize),
    }

    // this is a CSV format str, with 2 rows and 2 columns
    let csv = "1.2, 3.0
                4.2, 0.5";

    // we'll use iterator methods on the CSV to build an actual matrix over f64
    let mat = csv
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.split(",")
                .enumerate()
                .map(|(j, s)| (j, s.trim()))
                .map(|(j, s)| match s.parse::<f64>() {
                    Ok(float) => Ok((j, float)),
                    Err(parse_err) => Err(MatParseErr::NotAFloat(i, j, parse_err)),
                })
                .at_least(1, || (MatParseErr::NoColumns(i)))
                .map(|row| match row {
                    Ok((_, row)) => Ok(row),
                    Err(err) => Err(err),
                })
                .collect::<Result<Vec<f64>, MatParseErr>>()
        })
        .enumerate()
        .map(|(i, row)| match row {
            Ok(row) => Ok((i, row)),
            Err(row) => Err(row),
        })
        .at_least(1, || MatParseErr::NoRows)
        .const_over(
            |(_, vec)| vec.len(),
            |(i, vec), len, expected_len| MatParseErr::JaggedArray(i, vec, len, *expected_len),
        )
        .map(|row| match row {
            Ok((_, row)) => Ok(row),
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<_>, _>>();

    match mat {
        Ok(mat) => assert_eq!(mat, vec![vec![1.2, 3.0], vec![4.2, 0.5]]),
        Err(mperr) => match mperr {
            MatParseErr::NotAFloat(i, j, err) => println!("Got {err} at position [{i}, {j}]"),
            MatParseErr::NoColumns(i) => {
                println!("Row {i} is without any data, which would force the matrix to be empty")
            }
            MatParseErr::NoRows => println!("There are no rows in the matrix"),
            MatParseErr::JaggedArray(i, _vec, len, expected_len) => {
                println!("Row {i} has len {len}, when all rows should have length {expected_len}")
            }
        },
    }
}
