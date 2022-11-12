#[cfg(test)]
mod tests {
    use rust_interpreter::lexer::Lexer;
    use rust_interpreter::token::Token;

    #[test]
    fn it_works() {
        let input = String::from("let a = 5;");
        let mut l = Lexer::new(input);
        l.read_char();
        loop {
            let token = l.next_token();
            //why no work?
            if token == Token::EOF {
                break;
            } else {
                println!("{:?}", token);
            }
        }
    }
}