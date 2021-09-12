use bytes::Bytes;

use crate::token;
struct Lexer {
    data: Bytes,
}

impl Lexer {
    fn new<'a>(input: String) -> Self {
        let data = Bytes::from(input);
        Self { data }
    }

    // fn read_char(&mut self) {
    //     self.data.get_u8()
    // }
    fn next_token(&mut self) -> token::Token {
        let mut b = self.data.clone();
        let literal = b.split_to(1);
        let c = self.data.get_u8();
        match c {
            b'=' => token::Token::new(token::TokenType::Assign, literal),
            b';' => token::Token::new(token::TokenType::Semicolon, literal),
            b'(' => token::Token::new(token::TokenType::Lparne, literal),
            b')' => token::Token::new(token::TokenType::Rparne, literal),
            b',' => token::Token::new(token::TokenType::Comma, literal),
            b'+' => token::Token::new(token::TokenType::Plus, literal),
            b'{' => token::Token::new(token::TokenType::Lbrace, literal),
            b'}' => token::Token::new(token::TokenType::Rbrace, literal),
            _ => token::Token::new(token::TokenType::Illegal, Bytes::new()),
        }
    }
}

#[cfg(test)]
mod test {
    use bytes::Bytes;

    use crate::token::{Token, TokenType};

    use super::Lexer;
    #[test]
    fn test_next_token() {
        let input = "=+(){},;".to_string();
        let mut l = Lexer::new(input);
        for expect in vec![
            Token::new(TokenType::Assign, Bytes::from(&b"="[..])),
            Token::new(TokenType::Plus, Bytes::from(&b"+"[..])),
            Token::new(TokenType::Lparne, Bytes::from(&b"("[..])),
            Token::new(TokenType::Rparne, Bytes::from(&b")"[..])),
            Token::new(TokenType::Lbrace, Bytes::from(&b"{"[..])),
            Token::new(TokenType::Rbrace, Bytes::from(&b"}"[..])),
            Token::new(TokenType::Comma, Bytes::from(&b","[..])),
            Token::new(TokenType::Semicolon, Bytes::from(&b";"[..])),
        ] {
            let token = l.next_token();
            assert_eq!(
                token.typ, expect.typ,
                "got={:?}, want={:?}",
                token.typ, expect.typ
            );
            assert_eq!(&token.literal[..], &expect.literal[..]);
        }
        assert!(l.data.is_empty())
    }
}
