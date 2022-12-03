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

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }   // Read next char, update positions

    fn read_identifier(&mut self) -> Token {
        let mut ident = Vec::new();
        loop {
            if self.ch.is_alphabetic() {
                // push to vector
                ident.push(self.ch);
                self.read_char();
            } else {
                break;
            }
        }
        return Token::IDENT(ident)
    }

    pub fn next_token(&mut self) -> Token {
        let tok;
        match self.ch {
            // "let" => {
            //     tok = Token::LET(self.ch);
            // },
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
            '\0' => {
                tok = Token::EOF(self.ch)
            },
            _ => {
                if self.ch.is_alphabetic() {
                    tok = self.read_identifier();
                } else {
                    // this needs to ignore and skip spaces
                    tok = Token::ILLEGAL(self.ch)
                }
            }
            // Other patterns
        }
        self.read_char();
        tok
    }
}
