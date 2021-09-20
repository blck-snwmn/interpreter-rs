use crate::token;

#[derive(Debug)]
pub(crate) enum Expression {
    Identifier(Identifier),
}
impl super::Node for Expression {
    fn token_literal(&self) -> &str {
        todo!()
    }

    fn string(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.string(),
        }
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
}
impl super::Node for Identifier {
    fn token_literal(&self) -> &str {
        std::str::from_utf8(&self.token.literal[..]).unwrap()
    }

    fn string(&self) -> String {
        // Bytes の clone は値自体をコピーしているわけではなさそうだけど、さすがにこの実装ではデータのコピー発生してそう（要検証）
        // debug 用途なので、一旦許容する
        String::from_utf8(self.token.literal.clone().to_vec()).unwrap()
    }
}
