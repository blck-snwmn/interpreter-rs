use crate::token;

use super::expression;

pub(super) enum Statement {
    LetStatement(LetStatement),
}
impl Statement {
    pub(super) fn token_literal(&self) -> &str {
        todo!()
    }
}

pub(super) struct LetStatement {
    token: token::Token,
    name: Identifier,
    value: expression::Expression,
}

impl LetStatement {
    pub(super) fn token_literal(&self) -> &str {
        std::str::from_utf8(&self.token.literal[..]).unwrap()
    }
}

struct Identifier {
    token: token::Token,
}

impl Identifier {
    pub(super) fn token_literal(&self) -> &str {
        std::str::from_utf8(&self.token.literal[..]).unwrap()
    }
}
