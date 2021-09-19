use std::fmt::Display;

use bytes::Bytes;

#[derive(Debug, PartialEq, Eq, Clone)]
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

    Eq,    // ==
    NotEq, // !=

    // keyword
    Function, // FUNCTION
    Let,      // LET
    True,     // true
    False,    // false
    If,       // if
    Else,     // else
    Retrun,   // return
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            TokenType::Illegal => "Illegal",
            TokenType::Eof => "Eof",
            TokenType::Ident => "Ident",
            TokenType::Int => "Int",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Bang => "!",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",
            TokenType::Lt => "<",
            TokenType::Gt => ">",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::Lparne => "(",
            TokenType::Rparne => ")",
            TokenType::Lbrace => "{",
            TokenType::Rbrace => "}",
            TokenType::Eq => "==",
            TokenType::NotEq => "!=",
            TokenType::Function => "Function",
            TokenType::Let => "Let",
            TokenType::True => "True",
            TokenType::False => "False",
            TokenType::If => "If",
            TokenType::Else => "Else",
            TokenType::Retrun => "Return",
        };
        f.write_str(s)
    }
}

// TODO to enum
#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) typ: TokenType,
    pub(crate) literal: Bytes,
}

impl Token {
    pub(crate) fn new(typ: TokenType, literal: Bytes) -> Self {
        Self { typ, literal }
    }
}
