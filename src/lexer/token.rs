use super::TokenType;

#[derive(Clone, Debug)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
}

impl Token {
    pub(crate) fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }

    /// Get a reference to the token's kind.
    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    /// Get a reference to the token's data.
    pub fn literal(&self) -> &str {
        self.literal.as_str()
    }
}