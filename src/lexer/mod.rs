mod token;
mod token_type;

pub use token::Token;
pub use token_type::TokenType;
use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    // this needs a lifetime so that it can be applied to the inputs string type as a lifetime reference
    input: &'a str,           // Source code
    position: usize,        // Reading position
    read_position: usize,   // Current moving reading position
    ch: char                // Current read character
}

pub(crate) const EOF_CHAR: char = '\0';
pub(crate) const SPACE_CHAR: char = ' ';

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0'
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        // turn the input into an iterable and use nth to grab the char at that position
        // unwrap the Option, if the it resolves as None then resolve to the EOF_CHAR, we are at the
        // end of the input...
        self.ch = self.input.chars().nth(self.read_position).unwrap_or(EOF_CHAR);
        self.position = self.read_position;
        self.read_position += 1
    }

    pub fn next_token(&mut self) -> Token {
        let tok;
        match self.ch {
            '=' => tok = Token::new(TokenType::ASSIGN, self.ch.to_string()),
            ';' => tok = Token::new(TokenType::SEMICOLON, self.ch.to_string()),
            '(' => tok = Token::new(TokenType::LPAREN, self.ch.to_string()),
            ')' => tok = Token::new(TokenType::RPAREN, self.ch.to_string()),
            ',' => tok = Token::new(TokenType::COMMA, self.ch.to_string()),
            '+' => tok = Token::new(TokenType::PLUS, self.ch.to_string()),
            '{' => tok = Token::new(TokenType::LBRACE, self.ch.to_string()),
            '}' => tok = Token::new(TokenType::RBRACE, self.ch.to_string()),
            _ => tok = Token::new(TokenType::EOF, '\0'.to_string()),
        }
        self.read_char();
        tok
    }
}

// implement the Iterator trait for our Lexer
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ch == TokenType::EOF {
            return None;
        }
        if self.input.is_empty() {
            let mut eof = Token::new(TokenKind::Eof, String::from("EOF"));
            eof.index = self.index;

            self.finished = true;
            return Some(Ok(eof));
        }

        if let Some(limit) = &mut self.limit {
            limit.consume();
            if limit.limited() {
                self.finished = true;
                return Some(Err(Error::limit(
                    "token limit reached, aborting lexing",
                    self.index,
                )));
            }
        }

        let mut c = Cursor::new(self.input);
        let r = c.advance();

        match r {
            Ok(mut token) => {
                token.index = self.index;
                self.index += token.data.len();

                self.input = &self.input[token.data.len()..];
                Some(Ok(token))
            }
            Err(mut err) => {
                err.index = self.index;
                self.index += err.data.len();

                self.input = &self.input[err.data.len()..];
                Some(Err(err))
            }
        }
    }
}


#[test]
fn special_characters() {
    let input = String::from("=+(){},;");
    let mut l = Lexer::new(&input);
    loop {
        let token = l.next_token();
        if token.token_type() == TokenType::EOF {
            break;
        // } else if token.token_type == TokenType::SP {
        //     break;
        } else {
            println!("{:?}", token);
        }
    }
}