use validiter::{validerr, error_only, with_element, ErrorOnly, ValidErr, WithElement};

// enum MyValidErr<T> {
//     AtMost(T),
//     Other(T),
//     AtLeast,
// }

// impl<T> ValidErr<T> for MyValidErr<T> {}

// with_element!(AtMost, MyValidErr);
// with_element!(Other, MyValidErr);
// error_only!(AtLeast, MyValidErr);


// fn func() -> MyValidErr<i32> {
//     AtMost::from_element(1)
// }

// fn other() -> MyValidErr<u32> {
//     Other::from_element(0)
// }

// fn at_least() -> MyValidErr<u16> {
//     AtLeast::new()
// }

fn main() {

}

validerr!{
    MyValidErr {
        WithElement {
            Err1
        }
        ErrorOnly {
            Err2
        }
    }
}
