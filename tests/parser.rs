#[cfg(test)]
mod tests {
    use rust_interpreter::lexer::Lexer;
    use rust_interpreter::ast;
    use rust_interpreter::parser::Parser;
    use rust_interpreter::token;

    #[test]
    fn let_statement() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let l = Lexer::new(input.to_string());
        let p =  Parser::new(l);
        p.next_token();
        p.next_token();

        let program = p.parse_program();
    
        if program.statements.len() != 3 {
            println!("program.Statements does not contain 3 statements. {:?}", program.statements.len());
        }
    }

}