#[derive(PartialEq, Eq, Clone)]
pub struct Error {
    pub(crate) message: String,
    pub(crate) index: usize,
}

impl Error {
    /// Create a new instance of `Error`.
    pub fn new(message: String, index: usize) -> Self {
        Self {
            message,
            index,
        }
    }
}