use crate::lexer::token::Token;

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes()
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => Token::Assign,
            b'+' => Token::Plus,
            b';' => Token::Semicolon,
            b',' => Token::Comma,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return match ident.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    _ => Token::Ident(ident)
                }
            }
            b'0'..=b'9' => return Token::Int(self.read_number()),
            0 => Token::Eof,
            _ => Token::Illegal
        };

        self.read_char();
        tok
    }

    fn read_ident(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() ||self.ch == b'_' {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use crate::lexer::lexer::Lexer;
    use crate::lexer::token::Token;

    #[test]
    fn get_next_token() -> Result<()> {
        let input = r#"let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        "#;
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::Lparen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::Rparen,
            Token::Semicolon,
            Token::Eof
        ];

        for token in tokens {
            let next_token = lexer.next_token();
            println!("type: {:?}, literal: {}", token, token);
            assert_eq!(token, next_token);
        }

        Ok(())
    }
}