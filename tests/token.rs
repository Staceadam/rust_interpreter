#[cfg(test)]
mod tests {
    use rust_interpreter::lexer::Lexer;
    use rust_interpreter::token::Token;

    #[test]
    fn special_characters() {
        let input = String::from("=+(){},;");
        let mut l = Lexer::new(input);
        l.read_char();
        loop {
            let token = l.next_token();
            if token == Token::EOF('\0') {
                break;
            } else {
                println!("{:?}", token);
            }
        }
    }


    #[test]
    fn full_input() {
        let input_str = "let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
          x + y;
        };
        
        let result = add(five, ten);
        "; 
        let input = String::from(input_str);
        let mut l = Lexer::new(input);
        l.read_char();
        loop {
            let token = l.next_token();
            if token == Token::EOF('\0') {
                break;
            } else {
                println!("{:?}", token);
            }
        }
    }
}