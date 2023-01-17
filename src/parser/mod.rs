use crate::{
    error::Error,
    lexer::{Lexer, Token, TokenType},
};

#[derive(Debug)]
pub struct Node {
    token_literal: String,
}

impl Node {
    pub fn token_literal(self) -> String {
        self.token_literal
    }
}

#[derive(Debug)]
pub struct Statement {
    node: Node,
}

pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Expression,
}

impl LetStatement {
    pub fn statement_node() {}
    pub fn token_literal(self) -> String {
        self.token.data
    }
}

pub struct Identifier {
    token: Token,
    value: String,
}

pub struct Expression {
    node: Node,
}

#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn token_literal(self) -> String {
        if self.statements.len() > 0 {
            "Hello".to_string()
            // self.statements[0].node.token_literal()
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug)]
// you have to specify a lifetime here when another struct is being used
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Option<Result<Token, Error>>,
    peek_token: Option<Result<Token, Error>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);

        Self {
            lexer,
            cur_token: None,
            peek_token: None,
        }
    }

    pub fn next_token(mut self) {
        self.cur_token = self.peek_token;
        self.peek_token = self.lexer.next()
    }

    pub fn parse(mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while let Some(result) = self.lexer.next() {
            let statement = Statement {
                node: Node {
                    token_literal: result.unwrap().data,
                },
            };
            program.statements.push(statement);
        }
        program
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn special_characters() {
        let input = r#"let x = 5;
        let y = 10;
        let foobar = 838383;"#;

        let p = Parser::new(&input);
        let statements = p.parse();

        println!("{:?}", statements);

        // assert_eq!(tokens.len(), 9);
    }
}
