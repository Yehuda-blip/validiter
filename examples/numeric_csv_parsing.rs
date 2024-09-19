use std::{num::ParseFloatError, vec};

use validiter::{AtLeast, ConstOver, Ensure};

fn main() {
    // In this example we will use the 'cast_errs' method to
    // create a 'Vec<Vec<f64>>' collection, while ensuring
    // the mathematical validity if this collection as a numerical
    // matrix. To exercise the 'ensure' adapter, we'll force all
    // elements to be non-negative as well

    // Here we define the errors we expect to encounter in
    // the parsing process:
    #[derive(Debug)]
    enum MatParseErr {
        NotAFloat(usize, usize, ParseFloatError),
        NoColumns(usize),
        Negative(usize, usize, f64),
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
                .map(|s| s.trim())
                .enumerate()
                .map(|(j, s)| {
                    s.parse::<f64>()
                        .map_err(|parse_err| MatParseErr::NotAFloat(i, j, parse_err))
                })
                .ensure(|val| *val >= 0.0, |j, val| MatParseErr::Negative(i, j, val))
                .at_least(1, |_| MatParseErr::NoColumns(i))
                .collect::<Result<Vec<f64>, MatParseErr>>()
        })
        .at_least(1, |_| MatParseErr::NoRows)
        .const_over(
            |vec| vec.len(),
            |i, vec, len, expected_len| MatParseErr::JaggedArray(i, vec, len, *expected_len),
        )
        .collect::<Result<Vec<_>, _>>();

    match mat {
        Ok(mat) => {
            assert_eq!(mat, vec![vec![1.2, 3.0], vec![4.2, 0.5]]);
            println!("{mat:?}")
        }
        Err(mperr) => match mperr {
            MatParseErr::NotAFloat(i, j, err) => println!("Got {err} at pos [{i}, {j}]"),
            MatParseErr::NoColumns(i) => {
                println!("Row {i} is without any data, which would force the matrix to be empty")
            }
            MatParseErr::Negative(i, j, val) => {
                println!("value {val} at pos [{i}, {j}] is negative")
            }
            MatParseErr::NoRows => println!("There are no rows in the matrix"),
            MatParseErr::JaggedArray(i, _vec, len, expected_len) => {
                println!("Row {i} has len {len}, when all rows should have length {expected_len}")
            }
        },
    }
}
