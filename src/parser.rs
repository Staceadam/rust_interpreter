use crate::{token::Token, lexer::Lexer, ast::{Program, Statement, LetStatement, Identifier, Node}};

#[derive(Clone)]
pub struct Parser {
    pub l: Lexer,
    pub cur_token:  Option<Token>,
    pub peek_token: Option<Token>
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        Self { l: l, cur_token: None, peek_token: None }
    } 

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = Some(self.l.next_token())
    }

    pub fn parse_program(&mut self) -> Program {
        let p = Program { statements: vec![] };
        while self.cur_token != Some(Token::EOF('\0')) {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                p.statements.push(stmt.unwrap());
                self.next_token()
            }
        }
        p
    }

    pub fn parse_statement(self) -> Option<Statement> {
        match self.cur_token {
          Some(Token::LET(_)) => {
            let let_stmt = self.parse_let_statement();
            return Some(Statement{ node: Node { token_literal: "idk".to_string() }, statement_node: Node { token_literal: "idk".to_string() } }) 
          },
          _ => {
            None
          }
        }
    }

    fn peek_token_is(&mut self, t: Token) -> bool {
        return self.peek_token == Some(t)
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true
        } else {
            return false
        }
    }
    
    fn cur_token_is(self, t: Token) -> bool {
        return self.cur_token == Some(t);
    } 

    pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let mut stmt = LetStatement{ token: self.cur_token, name: None, value: None };
        
        if self.expect_peek(Token::IDENT(" ".to_owned())) {
            return None
        }

        stmt.name = Some(Identifier{ token: self.cur_token, value: stringify!(self.cur_token).to_string() });
    
        if !self.expect_peek(Token::ASSIGN('=')) {
            return None
        }

        while self.cur_token_is(Token::SEMICOLON(';')) {
            self.next_token()
        }
    
        return Some(stmt)
    }
}