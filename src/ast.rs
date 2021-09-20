pub(crate) mod expression;
pub(crate) mod statement;

pub trait Node {
    fn token_literal(&self) -> &str;
    fn string(&self) -> String;
}

pub(crate) struct Program {
    pub(crate) statements: Vec<statement::Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        self.statements
            .first()
            .map_or_else(|| "", |s| s.token_literal())
    }

    fn string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.string())
            .collect::<String>()
    }
}
