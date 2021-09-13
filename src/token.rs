use bytes::Bytes;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TokenType {
    Illegal, // ILLEGAL
    Eof,     // EOF

    // identifier, literal
    Ident, // IDENT
    Int,   // INT

    // operator
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Bang,     // !
    Asterisk, // *
    Slash,    // /

    Lt, // <
    Gt, // >

    Comma,     // ,
    Semicolon, // ;

    Lparne, // (
    Rparne, //)

    Lbrace, //{
    Rbrace, // }
    // keyword
    Function, // FUNCTION
    Let,      // LET
    True,     // true
    False,    // false
    If,       // if
    Else,     // else
    Retrun,   // return
}

// TODO to enum
pub(crate) struct Token {
    pub(crate) typ: TokenType,
    pub(crate) literal: Bytes,
}

impl Token {
    pub(crate) fn new(typ: TokenType, literal: Bytes) -> Self {
        Self { typ, literal }
    }
}
