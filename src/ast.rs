use crate::{token::{Token}, parser::Parser};

pub struct Node {
    token_literal: String
}

pub struct Statement {
    node: Node,
    statement_node: Node
}

pub struct Expression {
    node: Node,
    expression_node: Node
}

#[derive(Default)]
pub struct LetStatement {
    pub token: Option<Token>,
    pub name:  Option<Identifier>,
    pub value: Option<Expression>
}

impl LetStatement {
    fn statement_node() {

    }
    fn token_literal(self) -> Option<Token> {
        return self.token
    }
}


pub struct Identifier {
    token: Option<Token>, // the token.IDENT token
    value: String
}

impl Identifier {
    fn expression_node() {

    }
    fn token_literal(self) -> String {
        return stringify!(self.token).to_string()
    }
}

pub struct Program  {
    pub statements: Vec<Statement> 
}

impl Program {
    pub fn token_literal(&mut self) -> String {
        if self.statements.len() > 0 {
            // return self.statements[0].node.token_literal
            return self.statements[0].node.token_literal.to_owned();
        } else {
            return "".to_owned()
        }
    }
}
