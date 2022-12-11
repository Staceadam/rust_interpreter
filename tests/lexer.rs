#[cfg(test)]
mod tests {
    use rust_interpreter::lexer::Lexer;
    use rust_interpreter::token::Token;

    #[test]
    fn special_characters() {
        let input = String::from("=+(){},;");
        let mut l = Lexer::new(input);
        loop {
            let token = l.next_token();
            if token == Token::EOF('\0') {
                break;
            } else if token == Token::SPACE(' ') {
                break;
            } else {
                println!("{:?}", token);
            }
        }
    }


    #[test]
    fn next_token() {
        let input_str = "let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
          x + y;
        };
        
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        
        10 == 10;
        10 != 9;        
        "; 
        let input = String::from(input_str);
        let mut l = Lexer::new(input);
        loop {
            let token = l.next_token();
            if token == Token::EOF('\0') {
                break;
            } else if token == Token::SPACE(' ') || token == Token::SPACE('\n') || token == Token::SPACE('\t') || token == Token::SPACE('\r'){
                continue;
            } else {
                println!("{:?}", token);
            }
        }
    }

    #[test]
    fn alpha() {
        assert!('a'.is_alphabetic());
    }
}