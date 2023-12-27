#[derive(Debug)]
pub enum ValidResult<E> {
    Err(E),
    Ok(E)
}
