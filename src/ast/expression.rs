#[derive(Debug)]
pub(crate) enum Expression {
    Nop,
}
impl Expression {
    pub(crate) fn token_literal(&self) -> &str {
        todo!()
    }
}
