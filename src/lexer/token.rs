use super::TokenType;

#[derive(Clone, Debug)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) data: String,
    pub(crate) index: usize,
}

impl Token {
    pub(crate) fn new(token_type: TokenType, data: String) -> Self {
        Self {
            token_type,
            data,
            index: 0
        }
    }

    /// Get a reference to the token's kind.
    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    /// Get a reference to the token's data.
    pub fn data(&self) -> &str {
        self.data.as_str()
    }
}