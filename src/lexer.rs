use bytes::{Buf, Bytes};

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
        self.consume_white_space();
        // FIXME is_emptyの判定まわりをもう少し最適化したい
        if self.data.is_empty() {
            return token::Token::new(token::TokenType::Eof, Bytes::new());
        }

        let mut b = self.data.clone();
        // 保持するためのdata
        let literal = b.split_to(1);
        // １文字消費しつつ、 `u8` を取得
        // これをもとに判定する
        let c = self.data.clone().get_u8();
        let token = match c {
            b'=' => token::Token::new(token::TokenType::Assign, literal),
            b';' => token::Token::new(token::TokenType::Semicolon, literal),
            b'(' => token::Token::new(token::TokenType::Lparne, literal),
            b')' => token::Token::new(token::TokenType::Rparne, literal),
            b',' => token::Token::new(token::TokenType::Comma, literal),
            b'+' => token::Token::new(token::TokenType::Plus, literal),
            b'-' => token::Token::new(token::TokenType::Minus, literal),
            b'!' => token::Token::new(token::TokenType::Bang, literal),
            b'/' => token::Token::new(token::TokenType::Slash, literal),
            b'*' => token::Token::new(token::TokenType::Asterisk, literal),
            b'<' => token::Token::new(token::TokenType::Lt, literal),
            b'>' => token::Token::new(token::TokenType::Gt, literal),
            b'{' => token::Token::new(token::TokenType::Lbrace, literal),
            b'}' => token::Token::new(token::TokenType::Rbrace, literal),
            s if Lexer::is_letter(s) => {
                // is_letterを満たさない最初の位置を取得
                let mut by = self.data.split(|s| !Lexer::is_letter(*s));
                let x = by.next().unwrap().len();
                // 取得したis_letterを満たさない位置でsplitし、literalとする
                let literal = self.data.split_to(x);

                // identifierかどうか
                let x = &literal[..];
                let token_type = match x {
                    b"fn" => token::TokenType::Function,
                    b"let" => token::TokenType::Let,
                    _ => token::TokenType::Ident,
                };

                // FIXME returnしているのを直す
                return token::Token::new(token_type, literal);
            }
            s if Lexer::is_digit(s) => {
                // is_letterを満たさない最初の位置を取得
                let mut by = self.data.split(|s| !Lexer::is_digit(*s));
                let x = by.next().unwrap().len();
                // 取得したis_letterを満たさない位置でsplitし、literalとする
                let literal = self.data.split_to(x);
                // FIXME returnしているのを直す
                return token::Token::new(token::TokenType::Int, literal);
            }
            _ => token::Token::new(token::TokenType::Illegal, Bytes::new()),
        };
        // １文字分すすめる
        self.data.get_u8();
        token
    }
    const fn is_letter(s: u8) -> bool {
        matches!(s, b'a'..=b'z' | b'A'..=b'Z' | b'_')
    }
    const fn is_digit(s: u8) -> bool {
        matches!(s, b'0'..=b'9')
    }
    const fn is_whitespace(s: u8) -> bool {
        matches!(s, b' ' | b'\t' | b'\n' | b'\r')
    }
    fn consume_white_space(&mut self) {
        // 消費するスペース等を満たさない最初の位置を取得
        let mut by = self.data.split(|s| !Lexer::is_whitespace(*s));
        let x = by.next().unwrap().len();
        // 消費
        let _ = self.data.split_to(x);
    }
}

#[cfg(test)]
mod test {
    use bytes::Bytes;

    use crate::token::{Token, TokenType};

    use super::Lexer;
    #[test]
    fn test_next_token() {
        let input = r"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        "
        .to_string();

        let mut l = Lexer::new(input);
        for (i, expect) in vec![
            Token::new(TokenType::Let, Bytes::from(&b"let"[..])),
            Token::new(TokenType::Ident, Bytes::from(&b"five"[..])),
            Token::new(TokenType::Assign, Bytes::from(&b"="[..])),
            Token::new(TokenType::Int, Bytes::from(&b"5"[..])),
            Token::new(TokenType::Semicolon, Bytes::from(&b";"[..])),
            Token::new(TokenType::Let, Bytes::from(&b"let"[..])),
            Token::new(TokenType::Ident, Bytes::from(&b"ten"[..])),
            Token::new(TokenType::Assign, Bytes::from(&b"="[..])),
            Token::new(TokenType::Int, Bytes::from(&b"10"[..])),
            Token::new(TokenType::Semicolon, Bytes::from(&b";"[..])),
            Token::new(TokenType::Let, Bytes::from(&b"let"[..])),
            Token::new(TokenType::Ident, Bytes::from(&b"add"[..])),
            Token::new(TokenType::Assign, Bytes::from(&b"="[..])),
            Token::new(TokenType::Function, Bytes::from(&b"fn"[..])),
            Token::new(TokenType::Lparne, Bytes::from(&b"("[..])),
            Token::new(TokenType::Ident, Bytes::from(&b"x"[..])),
            Token::new(TokenType::Comma, Bytes::from(&b","[..])),
            Token::new(TokenType::Ident, Bytes::from(&b"y"[..])),
            Token::new(TokenType::Rparne, Bytes::from(&b")"[..])),
            Token::new(TokenType::Lbrace, Bytes::from(&b"{"[..])),
            Token::new(TokenType::Ident, Bytes::from(&b"x"[..])),
            Token::new(TokenType::Plus, Bytes::from(&b"+"[..])),
            Token::new(TokenType::Ident, Bytes::from(&b"y"[..])),
            Token::new(TokenType::Semicolon, Bytes::from(&b";"[..])),
            Token::new(TokenType::Rbrace, Bytes::from(&b"}"[..])),
            Token::new(TokenType::Semicolon, Bytes::from(&b";"[..])),
            Token::new(TokenType::Let, Bytes::from(&b"let"[..])),
            Token::new(TokenType::Ident, Bytes::from(&b"result"[..])),
            Token::new(TokenType::Assign, Bytes::from(&b"="[..])),
            Token::new(TokenType::Ident, Bytes::from(&b"add"[..])),
            Token::new(TokenType::Lparne, Bytes::from(&b"("[..])),
            Token::new(TokenType::Ident, Bytes::from(&b"five"[..])),
            Token::new(TokenType::Comma, Bytes::from(&b","[..])),
            Token::new(TokenType::Ident, Bytes::from(&b"ten"[..])),
            Token::new(TokenType::Rparne, Bytes::from(&b")"[..])),
            Token::new(TokenType::Semicolon, Bytes::from(&b";"[..])),
            Token::new(TokenType::Bang, Bytes::from(&b"!"[..])),
            Token::new(TokenType::Minus, Bytes::from(&b"-"[..])),
            Token::new(TokenType::Slash, Bytes::from(&b"/"[..])),
            Token::new(TokenType::Asterisk, Bytes::from(&b"*"[..])),
            Token::new(TokenType::Int, Bytes::from(&b"5"[..])),
            Token::new(TokenType::Semicolon, Bytes::from(&b";"[..])),
            Token::new(TokenType::Int, Bytes::from(&b"5"[..])),
            Token::new(TokenType::Lt, Bytes::from(&b"<"[..])),
            Token::new(TokenType::Int, Bytes::from(&b"10"[..])),
            Token::new(TokenType::Gt, Bytes::from(&b">"[..])),
            Token::new(TokenType::Int, Bytes::from(&b"5"[..])),
            Token::new(TokenType::Semicolon, Bytes::from(&b";"[..])),
            Token::new(TokenType::Eof, Bytes::new()),
        ]
        .iter()
        .enumerate()
        {
            let token = l.next_token();
            assert_eq!(
                token.typ, expect.typ,
                "[{}]got={:?}, want={:?}",
                i, token.typ, expect.typ
            );
            assert_eq!(
                &token.literal[..],
                &expect.literal[..],
                "[{}]got={:?}, want={:?}",
                i,
                std::str::from_utf8(&token.literal[..]).unwrap(),
                std::str::from_utf8(&expect.literal[..]).unwrap(),
            );
        }
        // すべてが消費されている or is_whitespaceを満たすものだけで構成されているか確認
        assert!(
            l.data.iter().all(|s: &u8| Lexer::is_whitespace(*s)),
            "{:?}",
            l.data.to_ascii_lowercase()
        )
    }

    #[test]
    fn explain() {
        let input = r"ab sb ab".to_string();
        let mut input = Bytes::from(input);
        let mut by = input.split(|s| !Lexer::is_letter(*s));
        println!("{:?}", input.to_ascii_lowercase());
        let x = by.next().unwrap().len();
        let _ = input.split_to(x);
        println!("{:?}", x);
        println!("{:?}", input.to_ascii_lowercase());
    }
}
