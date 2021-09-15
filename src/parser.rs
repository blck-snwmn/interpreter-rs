use bytes::Bytes;

use crate::{
    ast::{self, statement},
    lexer, token,
};

struct Parser {
    l: lexer::Lexer,
    // TODO やはりOptionのほうがいい気がする。
    cur_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    fn new(mut l: lexer::Lexer) -> Self {
        let cur_token = l.next_token();
        let peek_token = l.next_token();
        Self {
            l,
            cur_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = std::mem::replace(&mut self.peek_token, self.l.next_token());
    }

    fn cur_token_is(&self, target: &token::TokenType) -> bool {
        match &self.cur_token.typ {
            a if a == target => true,
            _ => false,
        }
    }
    fn expect_peek(&mut self, target: &token::TokenType) -> bool {
        match &self.peek_token.typ {
            a if a == target => {
                self.next_token();
                true
            }
            _ => false,
        }
    }

    fn parse_let_statemet(&mut self) -> Option<statement::Statement> {
        let token = std::mem::replace(&mut self.cur_token, token::empty);
        if !self.expect_peek(&token::TokenType::Ident) {
            return None;
        }
        let identifier_token = std::mem::replace(&mut self.cur_token, token::empty);
        let name = ast::statement::Identifier::new(identifier_token);
        while !self.cur_token_is(&token::TokenType::Semicolon) {
            self.next_token()
        }
        Some(statement::Statement::LetStatement(
            statement::LetStatement {
                name,
                token,
                value: ast::expression::Expression::Nop,
            },
        ))
    }

    fn parse_statement(&mut self) -> Option<statement::Statement> {
        match self.cur_token.typ {
            token::TokenType::Let => self.parse_let_statemet(),
            _ => None,
        }
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut p = ast::Program {
            statements: Vec::new(),
        };
        while self.cur_token.typ != token::TokenType::Eof {
            let stmt = self.parse_statement();
            if let Some(stmt) = stmt {
                p.statements.push(stmt);
            }
            self.next_token();
        }
        p
    }
}

#[cfg(test)]
mod test {
    use crate::{ast, lexer::Lexer};

    use super::Parser;

    #[test]
    fn test_let_statements() {
        let input = r"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "
        .to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();

        assert_eq!(program.statements.len(), 3);

        {
            let s = program.statements.get(0).unwrap();
            test_let_statement(s, "x")
        }
        {
            let s = program.statements.get(1).unwrap();
            test_let_statement(s, "y")
        }
        {
            let s = program.statements.get(2).unwrap();
            test_let_statement(s, "foobar")
        }
    }

    fn test_let_statement(s: &ast::statement::Statement, expected_name: &str) {
        assert_eq!(s.token_literal(), "let");

        let ls = match s {
            ast::statement::Statement::LetStatement(let_statement) => let_statement,
            _ => panic!("unexpected statement"),
        };
        assert_eq!(ls.name.token_literal(), expected_name);
    }
}
