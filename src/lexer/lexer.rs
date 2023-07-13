#[derive(Debug, PartialEq)]
pub enum Token<>{
    Illegal,
    EOF,
    Identifier(Vec<u8>),
    Integer(usize),
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    LSquirly,
    RSquirly,
    Function,
    Let,
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer{
            read_position:0,
            ch: 0,
            input: input.into_bytes(),
            position: 0,
        };
        
        lexer.read_char();
        return lexer;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'\0';
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn skip_whitespace(&mut self) -> () {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            b'=' => Token::Assign,
            b'+' => Token::Plus,
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'\0' => Token::EOF,
            _ => {
                if(self.is_letter()) {
                    let word = self.read_identifier();
                    return match word.as_str(){
                        "let" => Token::Let,
                        "fn" => Token::Function,
                        _ => Token::Identifier(word.into())
                    }
                } else if self.is_number() {
                    let number = self.read_number();
                    return Token::Integer(number);
                }
                else {
                    return Token::Illegal
                }
            }
        };

        self.read_char();
        return token;
    }

    pub fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.is_letter() {
            self.read_char();
        }

        return String::from_utf8(self.input[pos..self.position].into()).unwrap();
    }

    pub fn read_number(&mut self) -> usize {
        let pos = self.position;
        while self.is_number() {
            self.read_char();
        }
        
       let number_str = String::from_utf8(self.input[pos..self.position].into()).unwrap();
       return  number_str.parse().unwrap();
    }

    pub fn is_letter(&self) -> bool {
        return self.ch >= b'a' && self.ch <= b'z'
            || self.ch >= b'A' && self.ch <= b'Z'
            || self.ch == b'_';
    }

    pub fn is_number(&self) -> bool {
        return self.ch >= b'0' && self.ch <= b'9';
    }

}


#[cfg(test)]
mod tests {
    use super::{Token, Lexer};
    #[test]
    fn test_next_token() {
        let input = String::from("=let+             192Batu{},;Umut Damla");
        let mut lexer = Lexer::new(input);
        let expected = vec![
            Token::Assign,
            Token::Let,
            Token::Plus,
            Token::Integer(192),
            Token::Identifier("Batu".into()),
            Token::LSquirly,
            Token::RSquirly,
            Token::Comma,
            Token::Semicolon,
            Token::Identifier("Umut".into()),
            Token::Identifier("Damla".into()),
            Token::EOF,
        ];
        let mut res = vec![];
        while(lexer.ch != 0){
            res.push(lexer.next_token());
        }
        res.push(lexer.next_token());
        assert_eq!(res, expected);
    }

    #[test]
    fn test_expression() {
        let input = String::from("let x = y + 15");
        let mut lexer = Lexer::new(input);
        let expected = vec![
            Token::Let,
            Token::Identifier("x".into()),
            Token::Assign,
            Token::Identifier("y".into()),
            Token::Plus,
            Token::Integer(15),
            Token::EOF,
        ];
        let mut res = vec![];
        while(lexer.ch != 0){
            res.push(lexer.next_token());
        }
        res.push(lexer.next_token());
        assert_eq!(res, expected);
    }
}

