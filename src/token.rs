// ILLEGAL = "ILLEGAL"
// EOF     = "EOF"

// // Identifiers + literals
// IDENT = "IDENT" // add, foobar, x, y, ...
// INT   = "INT"   // 1343456

// // Operators
// ASSIGN   = "="
// PLUS     = "+"

// // Delimiters
// COMMA     = ","
// SEMICOLON = ";"

// LPAREN = "("
// RPAREN = ")"
// LBRACE = "{"
// RBRACE = "}"

// // Keywords
// FUNCTION = "FUNCTION"
// LET      = "LET"
#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    ILLEGAL(char), 
    SPACE(char), 
    EOF(char),
    // Identifiers + literals
    IDENT(String), 
    INT(String),   

    // Operators
    ASSIGN(char), 
    PLUS(char),
    MINUS(char),
    BANG(char),
    ASTERISK(char),
    SLASH(char),
    LT(char),
    GT(char),


    COMMA(char),
    SEMICOLON(char),
    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),
    EQ(String),
    NOT_EQ(String),

    // Keywords
    FUNCTION(String),
    LET(String),
    TRUE(String),
    FALSE(String),
    IF(String),
    ELSE(String),
    RETURN(String)
}

