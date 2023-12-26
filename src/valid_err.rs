#[derive(Debug, PartialEq)]
pub enum ValidErr<I: Iterator> {
    TooMany(I::Item),
    TooFew,
    InvalidItem(I::Item)
}
