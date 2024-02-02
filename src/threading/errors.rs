#[derive(Debug)]
pub struct ThreadingError {}
pub type ThreadingResult<T> = Result<T, ThreadingError>;
