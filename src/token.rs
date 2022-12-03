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
    EOF(char),
    // Identifiers + literals
    IDENT(Vec<char>), 
    INT(char),   

    // Operators
    ASSIGN(char), 
    PLUS(char),
    MINUS(char),


    COMMA(char),
    SEMICOLON(char),
    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),

    // Keywords
    FUNCTION(char),
    LET(String),
}

