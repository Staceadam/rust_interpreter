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
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0'
        };
        lexer.read_char();
        return lexer
    } 

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            return '\0'
        } else {
            return self.input[self.read_position]
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }   // Read next char, update positions
    

    fn read_number(&mut self) -> Token {
        let mut int: String = "".to_owned();
        while self.ch.is_numeric() {
            int.push_str(&String::from(self.ch));
            self.read_char();
        }     
        self.read_position -= 1;
        return Token::INT(int);
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident: String = "".to_owned();

        while self.ch.is_alphabetic() {
                ident.push_str(&self.ch.to_string());
                self.read_char();
        }        
        self.read_position -= 1;
        match ident.as_str() {
            "fn" => {
                return Token::FUNCTION(ident)
            },
            "let" => {
                return Token::LET(ident)
            },
            "true" => {
                return Token::TRUE(ident)
            },
            "false" => {
                return Token::FALSE(ident)
            },
            "if" => {
                return Token::IF(ident)
            },
            "else" => {
                return Token::ELSE(ident)
            },
            "return" => {
                return Token::RETURN(ident)
            },
            _ => {
                return Token::IDENT(ident)
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
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
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = Token::EQ("==".to_string())
                } else {
                    tok = Token::ASSIGN(self.ch);
                }
            },
            '+' => {
                tok = Token::PLUS(self.ch);
            },
            '-' => {
                tok = Token::MINUS(self.ch);
            },
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = Token::NOT_EQ("!=".to_string())
                } else {
                    tok = Token::BANG(self.ch);
                }
            },
            '*' => {
                tok = Token::ASTERISK(self.ch);
            },
            '/' => {
                tok = Token::SLASH(self.ch);
            },
            '<' => {
                tok = Token::LT(self.ch);
            },
            '>' => {
                tok = Token::GT(self.ch);
            },

            '\0' => {
                tok = Token::EOF(self.ch)
            },
            '\n' => {
                tok = Token::SPACE(self.ch)
            },
            ' ' => {
                tok = Token::SPACE(self.ch)
            },
            '\t' => {
                tok = Token::SPACE(self.ch)
            },
            '\r' => {
                tok = Token::SPACE(self.ch)
            },
            _ => {
                if self.ch.is_alphabetic() {
                    tok = self.read_identifier();
                } else if self.ch.is_numeric() {
                    tok = self.read_number();
                } else {
                    tok = Token::ILLEGAL(self.ch)
                }
            }
            // Other patterns
        }
        self.read_char();
        tok
    }
}
