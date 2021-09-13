use bytes::{Buf, Bytes};

pub(crate) struct Lexer {
    data: Bytes,
}

impl Lexer {
    pub(crate) fn new<'a>(input: String) -> Self {
        let data = Bytes::from(input);
        Self { data }
    }

    // fn read_char(&mut self) {
    //     self.data.get_u8()
    // }
    pub(crate) fn next_token(&mut self) -> token::Token {
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
        let mut peek = self.data.clone();
        let token = match peek.get_u8() {
            b'=' => {
                if !peek.is_empty() && peek.get_u8() == b'=' {
                    return token::Token::new(token::TokenType::Eq, self.data.split_to(2));
                }
                token::Token::new(token::TokenType::Assign, literal)
            }
            b';' => token::Token::new(token::TokenType::Semicolon, literal),
            b'(' => token::Token::new(token::TokenType::Lparne, literal),
            b')' => token::Token::new(token::TokenType::Rparne, literal),
            b',' => token::Token::new(token::TokenType::Comma, literal),
            b'+' => token::Token::new(token::TokenType::Plus, literal),
            b'-' => token::Token::new(token::TokenType::Minus, literal),
            b'!' => {
                if !peek.is_empty() && peek.get_u8() == b'=' {
                    return token::Token::new(token::TokenType::NotEq, self.data.split_to(2));
                }
                token::Token::new(token::TokenType::Bang, literal)
            }
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
                let token_type = Lexer::literal_to_token_type(&literal[..]);

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
    const fn literal_to_token_type(literal: &[u8]) -> token::TokenType {
        match literal {
            b"fn" => token::TokenType::Function,
            b"let" => token::TokenType::Let,
            b"true" => token::TokenType::True,
            b"false" => token::TokenType::False,
            b"if" => token::TokenType::If,
            b"else" => token::TokenType::Else,
            b"return" => token::TokenType::Retrun,
            _ => token::TokenType::Ident,
        }
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
        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
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
            Token::new(TokenType::If, Bytes::from(&b"if"[..])),
            Token::new(TokenType::Lparne, Bytes::from(&b"("[..])),
            Token::new(TokenType::Int, Bytes::from(&b"5"[..])),
            Token::new(TokenType::Lt, Bytes::from(&b"<"[..])),
            Token::new(TokenType::Int, Bytes::from(&b"10"[..])),
            Token::new(TokenType::Rparne, Bytes::from(&b")"[..])),
            Token::new(TokenType::Lbrace, Bytes::from(&b"{"[..])),
            Token::new(TokenType::Retrun, Bytes::from(&b"return"[..])),
            Token::new(TokenType::True, Bytes::from(&b"true"[..])),
            Token::new(TokenType::Semicolon, Bytes::from(&b";"[..])),
            Token::new(TokenType::Rbrace, Bytes::from(&b"}"[..])),
            Token::new(TokenType::Else, Bytes::from(&b"else"[..])),
            Token::new(TokenType::Lbrace, Bytes::from(&b"{"[..])),
            Token::new(TokenType::Retrun, Bytes::from(&b"return"[..])),
            Token::new(TokenType::False, Bytes::from(&b"false"[..])),
            Token::new(TokenType::Semicolon, Bytes::from(&b";"[..])),
            Token::new(TokenType::Rbrace, Bytes::from(&b"}"[..])),
            Token::new(TokenType::Int, Bytes::from(&b"10"[..])),
            Token::new(TokenType::Eq, Bytes::from(&b"=="[..])),
            Token::new(TokenType::Int, Bytes::from(&b"10"[..])),
            Token::new(TokenType::Semicolon, Bytes::from(&b";"[..])),
            Token::new(TokenType::Int, Bytes::from(&b"10"[..])),
            Token::new(TokenType::NotEq, Bytes::from(&b"!="[..])),
            Token::new(TokenType::Int, Bytes::from(&b"9"[..])),
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
