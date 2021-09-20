use crate::token;

use super::expression;

#[derive(Debug)]
pub(crate) enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}
impl Statement {
    pub(crate) fn token_literal(&self) -> &str {
        match self {
            Statement::LetStatement(let_statement) => let_statement.token_literal(),
            Statement::ReturnStatement(return_statement) => return_statement.token_literal(),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) struct ReturnStatement {
    pub(crate) token: token::Token,
    pub(crate) return_value: expression::Expression,
}

impl ReturnStatement {
    pub(crate) fn token_literal(&self) -> &str {
        std::str::from_utf8(&self.token.literal[..]).unwrap()
    }
}

#[derive(Debug)]
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
