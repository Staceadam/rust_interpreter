#[derive(Debug)]
pub enum Token {
    ILLEGAL(char), 
    EOF(char),
    // Identifiers + literals
    IDENT(char), 
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
    LET(char),
    IDENTIFIER(Vec<char>)
}

