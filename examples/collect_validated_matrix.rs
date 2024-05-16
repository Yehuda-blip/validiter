use validiter::{broken_const, too_few, ErrLiftable, Unvalidatable, ValidErr, ValidIter};

fn main() {
    let ok_s = "abcd
abcd
abcd
abcd";
    println!("good matrix");
    match s_to_mat(ok_s) {
        Ok(mat) => println!("{:?}", mat),
        Err(err) => println!("{}", err),
    }

    let empty_s = "";
    println!("empty matrix");
    match s_to_mat(empty_s) {
        Ok(mat) => println!("{:?}", mat),
        Err(err) => println!("{}", err),
    }

    let empty_lines_s = "

";
    println!("empty lines matrix");
    match s_to_mat(empty_lines_s) {
        Ok(mat) => println!("{:?}", mat),
        Err(err) => println!("{}", err),
    }

    let different_length_lines_s = "abcd
abcd
abc
abcd";
    println!("different length rows matrix");
    match s_to_mat(different_length_lines_s) {
        Ok(mat) => println!("{:?}", mat),
        Err(err) => println!("{}", err),
    }
}

fn s_to_mat(s: &str) -> Result<Vec<Vec<char>>, ValidErr<Vec<char>>> {
    s.lines()
        .map(|line| {
            line.chars()
                .validate()
                .at_least(1, too_few!("Matrix row cannot be emtpy: " plus_auto))
                .collect::<Result<Vec<char>, _>>()
        })
        .lift_errs()
        .at_least(1, too_few!("Matrix must have rows: " plus_auto))
        .const_over(|vec| vec.len(), broken_const!("All matrix rows must have uniform length: " plus_auto_debug))
        .collect::<Result<Vec<Vec<char>>, _>>()
}
