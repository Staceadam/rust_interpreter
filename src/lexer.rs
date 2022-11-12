use crate::token::{Token};

//https://mohitkarekar.com/posts/pl/lexer/

pub struct Lexer {
    input: Vec<char>,           // Source code
    pub position: usize,        // Reading position
    pub read_position: usize,   // Current moving reading position
    pub ch: char                // Current read character
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0'
        }
    }             // Create a new Lexer instance

    pub fn read_char(mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }   // Read next char, update positions
    
    pub fn next_token(self) -> Token {
        let tok;
        match self.ch {
            ',' => {
                tok = Token::COMMA(self.ch);
            },
            ';' => {
                tok = Token::SEMICOLON(self.ch);
            },
            '(' => {
                tok = Token::LPAREN(self.ch);
            },
            ')' => {
                tok = Token::RPAREN(self.ch);
            },
            '{' => {
                tok = Token::LBRACE(self.ch);
            },
            '}' => {
                tok = Token::RBRACE(self.ch);
            },
            '=' => {
                tok = Token::ASSIGN(self.ch);
            },
            '+' => {
                tok = Token::PLUS(self.ch);
            },
            '-' => {
                tok = Token::MINUS(self.ch);
            },
            ' ' => {
                tok = Token::EOF(self.ch)
            },
            _ => panic!()
            // Other patterns
        }
        self.read_char();
        tok
    }
}
