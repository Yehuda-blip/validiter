use validiter::{ErrCastable, Unvalidatable, ValidErr, ValidIter};

fn main() {
    let ok_s = "abcd
abcd
abcd
abcd";
    println!("good matrix");
    match s_to_mat(ok_s) {
        Ok(mat) => println!("{:?}", mat),
        Err(err) => println!("bad input string, could not build matrix - err: {:?}", err),
    }

    let empty_s = "";
    println!("empty matrix");
    match s_to_mat(empty_s) {
        Ok(mat) => println!("{:?}", mat),
        Err(err) => println!("bad input string, could not build matrix - err: {:?}", err),
    }

    let empty_lines_s = "

";
    println!("empty lines matrix");
    match s_to_mat(empty_lines_s) {
        Ok(mat) => println!("{:?}", mat),
        Err(err) => println!("bad input string, could not build matrix - err: {:?}", err),
    }

    let different_length_lines_s = "abcd
abcd
abc
abcd";
    println!("different length rows matrix");
    match s_to_mat(different_length_lines_s) {
        Ok(mat) => println!("{:?}", mat),
        Err(err) => println!("bad input string, could not build matrix - err: {:?}", err),
    }
}

fn s_to_mat(s: &str) -> Result<Vec<Vec<char>>, ValidErr<Vec<char>>> {
    s.lines()
        .map(|line| {
            line.chars()
                .validate()
                .at_least(1, "no columns!")
                .collect::<Result<Vec<char>, _>>()
        })
        .cast_errs()
        .at_least(1, "no rows!")
        .const_over(|vec| vec.len(), "row length changed!")
        .collect::<Result<Vec<Vec<char>>, _>>()
}
