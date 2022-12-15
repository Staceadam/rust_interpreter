mod token;
mod token_type;

pub use token::Token;
pub use token_type::TokenType;
use crate::error::Error;
use std::str::Chars;

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    // this needs a lifetime so that it can be applied to the inputs string type as a lifetime reference
    input: &'a str,           // Source code
    index: usize,        // Reading position
    finished: bool,         // At the end of the input
}

pub(crate) const EOF_CHAR: char = '\0';
pub(crate) const SPACE_CHAR: char = ' ';

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            index: 0,
            finished: false,
        }
    }

    /// Lex the full source text, consuming the lexer.
    pub fn lex(self) -> Vec<Token> {
        let mut tokens = vec![];

        for item in self {
            match item {
                Ok(token) => tokens.push(token),
                Err(error) => println!("{:?}", error),
            }
        }

        tokens
    }
}

// implement the Iterator trait for our Lexer
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None
        }
        if self.input.is_empty() {
            let eof = Token::new(TokenType::EOF, String::from("EOF"));
            self.finished = true;
            return Some(Ok(eof));
        }

        // create a new cursor
        let mut c = Cursor::new(self.input);
        // advance it to the next char
        let r = c.advance();

        match r {
            // if we get an Ok from the advance then update the lexers index and return some to the iterator
            Ok(mut token) => {
                token.index = self.index;
                self.index += token.data.len();

                self.input = &self.input[token.data.len()..];
                Some(Ok(token))
            }
            // if we get an Error from the advance then update the lexers index and return an Err
            Err(mut err) => {
                err.index = self.index;
                self.index += err.data.len();

                self.input = &self.input[err.data.len()..];
                Some(Err(err))
            }
        }
    }
}

pub(crate) struct Cursor<'a> {
    chars: Chars<'a>,
    pub(crate) err: Option<Error>,
}

fn is_digit_char(c: char) -> bool {
    matches!(c, '0'..='9')
}

fn is_ident_char(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_whitespace(c: char) -> bool {
    // from rust's lexer:
    matches!(
        c,
        // ASCII
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // Unicode BOM (Byte Order Mark)
        | '\u{FEFF}'

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            chars: input.chars(),
            err: None,
        }
    }

    fn advance(&mut self) -> Result<Token, Error> {
        let first_char = self.bump().unwrap();

        match first_char {
            c if is_digit_char(c) => self.number(c),
            c if is_whitespace(c) => self.whitespace(c),
            c if is_ident_char(c) => self.ident(c),
            '=' => Ok(Token::new(TokenType::ASSIGN, first_char.into())),
            '-' => Ok(Token::new(TokenType::MINUS, first_char.into())),
            '!' => Ok(Token::new(TokenType::BANG, first_char.into())),
            '*' => Ok(Token::new(TokenType::ASTERISK, first_char.into())),
            '/' => Ok(Token::new(TokenType::SLASH, first_char.into())),
            '<' => Ok(Token::new(TokenType::LT, first_char.into())),
            '>' => Ok(Token::new(TokenType::GT, first_char.into())),
            ';' => Ok(Token::new(TokenType::SEMICOLON, first_char.into())),
            '(' => Ok(Token::new(TokenType::LPAREN, first_char.into())),
            ')' => Ok(Token::new(TokenType::RPAREN, first_char.into())),
            ',' => Ok(Token::new(TokenType::COMMA, first_char.into())),
            '+' => Ok(Token::new(TokenType::PLUS, first_char.into())),
            '{' => Ok(Token::new(TokenType::LBRACE, first_char.into())),
            '}' => Ok(Token::new(TokenType::RBRACE, first_char.into())),
            c => Err(Error::new("Unexpected character", c.to_string())),
        }
    }

    /// Returns nth character relative to the current cursor position.
    fn nth_char(&mut self, n: usize) -> char {
        self.chars.nth(n).unwrap_or(EOF_CHAR)
    }

    /// Peeks the next char in input without consuming.
    pub(crate) fn first(&mut self) -> char {
        self.nth_char(0)
    }

    /// Peeks the second char in input without consuming.
    pub(crate) fn second(&mut self) -> char {
        self.nth_char(1)
    }

    /// Checks if there are chars to consume.
    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Moves to the next character.
    pub(crate) fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;

        Some(c)
    }

    fn whitespace(&mut self, first_char: char) -> Result<Token, Error> {
        let mut buf = String::new();
        buf.push(first_char);

        while !self.is_eof() {
            let first = self.bump().unwrap();
            if is_whitespace(first) {
                buf.push(first);
            } else {
                break;
            }
        }

        Ok(Token::new(TokenType::WHITESPACE, buf))
    }

    fn ident(&mut self, first_char: char) -> Result<Token, Error> {
        let mut buf = String::new();
        buf.push(first_char);

        while !self.is_eof() {
            let first = self.first();
            if is_ident_char(first) || is_digit_char(first) {
                buf.push(first);
                // self.bump();
            } else {
                break;
            }
        }

        match buf.as_str() {
            "let" => {              
                 Ok(Token::new(TokenType::LET, buf)) 
            }
            "fn" => {              
                 Ok(Token::new(TokenType::FUNCTION, buf)) 
            }
            _ => Ok(Token::new(TokenType::IDENT, buf)) 

        }
    }

    fn number(&mut self, first_digit: char) -> Result<Token, Error> {
        let mut buf = String::new();
        
        println!("{:?}", buf);
        buf.push(first_digit);

        let mut has_fractional = false;
        let mut has_digit = is_digit_char(first_digit);

        while !self.is_eof() {
            let first = self.first();

            println!("{:?}", first);
            match first {
                '.' => {
                    buf.push(first);
                    // self.bump();

                    if !has_digit {
                        self.err = Some(Error::new(
                            format!("Unexpected character `{}` before a digit", first),
                            first.to_string(),
                        ));
                    }

                    if has_fractional {
                        self.err = Some(Error::new(
                            format!("Unexpected character `{}`", first),
                            first.to_string(),
                        ));
                    }

                    has_fractional = true;
                }
                c if is_digit_char(c) => {
                    buf.push(c);
                    // self.bump();
                    has_digit = true;
                }
                _ => break     
            }
        }

        if has_fractional {
            Ok(Token::new(TokenType::FLOAT, buf))
        } else {
            Ok(Token::new(TokenType::INT, buf))
        }
    }

}

#[test]
fn special_characters() {
    let input = String::from("=+(){},;");
    let mut l = Lexer::new(&input);
    let tokens = l.lex();
    
    println!("{:?}", tokens);

    assert_eq!(tokens.len(), 9);
}

#[test]
fn white_space() {
    let input = String::from("=   +    (){},;");
    let mut l = Lexer::new(&input);
    let tokens = l.lex();
    
    println!("{:?}", tokens);

    assert_eq!(tokens.len(), 11);
}

#[test]
fn number() {
    //TODO: needs to handle floats that start with .
    let input = String::from("1256 34 53.3");
    let mut l = Lexer::new(&input);
    let tokens = l.lex();
    
    println!("{:?}", tokens);

    assert_eq!(tokens.len(), 6);
}

#[test]
fn ident() {
    let input = String::from("ident let fn special");
    let mut l = Lexer::new(&input);
    let tokens = l.lex();
    
    println!("{:?}", tokens);

    assert_eq!(tokens.len(), 8);
}


#[test]
fn complex() {
    let input = "let five = 5; 
    let ten = 10;
    let add = fn(x, y) {
        x + y;
    };

    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;";

    let mut l = Lexer::new(&input);
    let tokens = l.lex();
    
    println!("{:?}", tokens);

    // assert_eq!(tokens.len(), 8);
}
