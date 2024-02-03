#[derive(Debug)]
pub struct ThreadingError {
    msg: String,
}
impl ThreadingError {
    pub fn new(msg: &str) -> ThreadingError {
        ThreadingError {
            msg: format!("Threading Error: {}", msg),
        }
    }
}
impl std::fmt::Display for ThreadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl std::error::Error for ThreadingError {}

pub type ThreadingResult<T> = Result<T, ThreadingError>;
