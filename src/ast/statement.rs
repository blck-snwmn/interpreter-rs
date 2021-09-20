use crate::token;

use super::expression;

#[derive(Debug)]
pub(crate) enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}
impl super::Node for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Statement::LetStatement(let_statement) => let_statement.token_literal(),
            Statement::ReturnStatement(return_statement) => return_statement.token_literal(),
        }
    }
    fn string(&self) -> String {
        match self {
            Statement::LetStatement(let_statement) => let_statement.string(),
            Statement::ReturnStatement(return_statement) => return_statement.string(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LetStatement {
    pub(crate) token: token::Token,
    pub(crate) name: expression::Identifier,
    pub(crate) value: Option<expression::Expression>,
}

impl super::Node for LetStatement {
    fn token_literal(&self) -> &str {
        std::str::from_utf8(&self.token.literal[..]).unwrap()
    }
    fn string(&self) -> String {
        format!(
            "{} {} = {};",
            self.token_literal(),
            self.name.string(),
            self.value
                .as_ref()
                .map_or_else(|| "".to_string(), |v| v.string())
        )
    }
}

#[derive(Debug)]
pub(crate) struct ReturnStatement {
    pub(crate) token: token::Token,
    pub(crate) return_value: Option<expression::Expression>,
}

impl super::Node for ReturnStatement {
    fn token_literal(&self) -> &str {
        std::str::from_utf8(&self.token.literal[..]).unwrap()
    }
    fn string(&self) -> String {
        format!(
            "{} {}",
            self.token_literal(),
            self.return_value
                .as_ref()
                .map_or_else(|| "".to_string(), |v| v.string())
        )
    }
}

#[derive(Debug)]
pub(crate) struct ExpresstionStatement {
    pub(crate) token: token::Token,
    pub(crate) exresstion: Option<expression::Expression>,
}

impl super::Node for ExpresstionStatement {
    fn token_literal(&self) -> &str {
        std::str::from_utf8(&self.token.literal[..]).unwrap()
    }

    fn string(&self) -> String {
        self.exresstion
            .as_ref()
            .map_or_else(|| "".to_string(), |v| v.string())
    }
}
