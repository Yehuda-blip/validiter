use crate::valid_err::ValidErr;


pub struct AtLeast<I: Iterator> {
    iter: I,
    min_count: usize,
    counter: usize,
}

impl<I: Iterator> AtLeast<I> {
    pub(crate) fn new(iter: I, min_count: usize) -> AtLeast<I> {
        AtLeast {
            iter,
            min_count,
            counter: 0,
        }
    }
}

impl<I: Iterator> Iterator for AtLeast<I> {
    type Item = Result<I::Item, ValidErr<I>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(val) => {
                self.counter += 1;
                Some(Ok(val))
            }
            None => match self.counter >= self.min_count {
                true => None,
                false => {
                    self.counter = self.min_count;
                    Some(Err(ValidErr::TooFew))
                }
            },
        }
    }
}


