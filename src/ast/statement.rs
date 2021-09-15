use crate::token;

use super::expression;

pub(crate) enum Statement {
    LetStatement(LetStatement),
}
impl Statement {
    pub(crate) fn token_literal(&self) -> &str {
        match self {
            Statement::LetStatement(let_statement) => let_statement.token_literal(),
        }
    }
}

pub(crate) struct LetStatement {
    pub(crate) token: token::Token,
    pub(crate) name: Identifier,
    pub(crate) value: expression::Expression,
}

impl LetStatement {
    pub(crate) fn token_literal(&self) -> &str {
        std::str::from_utf8(&self.token.literal[..]).unwrap()
    }
}

pub(crate) struct Identifier {
    token: token::Token,
}

impl Identifier {
    pub(crate) fn new(token: token::Token) -> Self {
        Self { token }
    }

    pub(crate) fn token_literal(&self) -> &str {
        std::str::from_utf8(&self.token.literal[..]).unwrap()
    }
}
