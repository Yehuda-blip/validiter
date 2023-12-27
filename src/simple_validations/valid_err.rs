#[derive(Debug, PartialEq)]
pub enum ValidatedIteratorErr<I: Iterator> {
    TooMany(I::Item),
    TooFew,
    InvalidItem(I::Item),
    OutOfBounds(I::Item)
}
