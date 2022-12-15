#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Error {
    pub(crate) message: String,
    pub(crate) data: String,
    pub(crate) index: usize,
}

impl Error {
    /// Create a new instance of `Error`.
    pub fn new<S: Into<String>>(message: S, data: String) -> Self {
        Self {
            message: message.into(),
            data,
            index: 0,
        }
    }
}