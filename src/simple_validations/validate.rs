use super::valid_err::ValidatedIteratorErr;

pub struct Validate<I: Iterator, F: FnMut(&I::Item) -> bool> {
    iter: I,
    validation: F,
}

impl<I: Iterator, F> Validate<I, F>
where
    F: FnMut(&I::Item) -> bool,
{
    pub(crate) fn new(iter: I, validation: F) -> Validate<I, F> {
        Self { iter, validation }
    }
}

impl<I: Iterator, F> Iterator for Validate<I, F>
where
    F: FnMut(&I::Item) -> bool,
{
    type Item = Result<I::Item, ValidatedIteratorErr<I>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(element) => match (self.validation)(&element) {
                true => Some(Ok(element)),
                false => Some(Err(ValidatedIteratorErr::InvalidItem(element))),
            },
            None => None,
        }
    }
}