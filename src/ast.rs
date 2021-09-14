mod expression;
mod statement;
enum Node {
    Statement(statement::Statement),
    Expression(expression::Expression),
    Program(Program),
}

impl Node {
    fn token_literal(&self) -> &str {
        match self {
            Node::Statement(statement) => statement.token_literal(),
            Node::Expression(expression) => expression.token_literal(),
            Node::Program(program) => program.token_literal(),
        }
    }
}

pub(crate) struct Program {
    statements: Vec<statement::Statement>,
}

impl Program {
    fn token_literal(&self) -> &str {
        self.statements
            .first()
            .map_or_else(|| "", |s| s.token_literal())
    }
}
