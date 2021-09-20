use crate::{
    ast::{self, statement},
    lexer, token,
};
use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("expected next token to be {want}, got {got} instead")]
    UnexpectedPeekToken {
        want: token::TokenType,
        got: token::TokenType,
    },
}

struct Parser {
    l: lexer::Lexer,
    cur_token: Option<token::Token>,
    peek_token: Option<token::Token>,
    // TODO エラーをResultで統一したほうがいい気がしている
    errors: Vec<ParseError>,
}

impl Parser {
    fn new(l: lexer::Lexer) -> Self {
        let mut p = Self {
            l,
            cur_token: None,
            peek_token: None,
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        p
    }

    fn errors(&mut self) -> Vec<ParseError> {
        // TODO 一旦コピーせず、持っているものを返す。
        std::mem::take(&mut self.errors)
    }

    fn next_token(&mut self) {
        self.cur_token = std::mem::replace(&mut self.peek_token, Some(self.l.next_token()));
    }

    fn cur_token_is(&self, target: &token::TokenType) -> bool {
        matches!(&self.cur_token, Some(token::Token { typ, literal: _ }) if typ == target)
    }
    fn peek_token_is(&self, target: &token::TokenType) -> bool {
        matches!(&self.peek_token, Some(token::Token { typ, literal: _ }) if typ == target)
    }

    fn peek_error(&mut self, token: &token::TokenType) {
        self.errors.push(ParseError::UnexpectedPeekToken {
            want: token.clone(),
            got: self.peek_token.as_ref().unwrap().typ.clone(),
        })
    }

    // expect_peek check peek token. this method call next_token if own token's type match target type
    fn expect_peek(&mut self, target: &token::TokenType) -> bool {
        if self.peek_token_is(target) {
            self.next_token();
            true
        } else {
            self.peek_error(target);
            false
        }
    }

    fn parse_let_statemet(&mut self) -> Option<statement::Statement> {
        let token = std::mem::replace(&mut self.cur_token, None).unwrap();
        if !self.expect_peek(&token::TokenType::Ident) {
            return None;
        }
        let identifier_token = std::mem::replace(&mut self.cur_token, None).unwrap();
        let name = ast::expression::Identifier::new(identifier_token);

        if !self.expect_peek(&token::TokenType::Assign) {
            return None;
        }
        // TODO 一旦セミコロンまで読み飛ばす
        while !self.cur_token_is(&token::TokenType::Semicolon) {
            self.next_token()
        }
        Some(statement::Statement::LetStatement(
            statement::LetStatement {
                name,
                token,
                value: None,
            },
        ))
    }
    fn parse_return_statemet(&mut self) -> Option<statement::Statement> {
        let token = std::mem::replace(&mut self.cur_token, None).unwrap();

        self.next_token();

        // TODO 一旦セミコロンまで読み飛ばす
        while !self.cur_token_is(&token::TokenType::Semicolon) {
            self.next_token()
        }
        Some(statement::Statement::ReturnStatement(
            statement::ReturnStatement {
                token,
                return_value: None,
            },
        ))
    }

    fn parse_statement(&mut self) -> Option<statement::Statement> {
        match self.cur_token {
            Some(token::Token {
                typ: token::TokenType::Let,
                literal: _,
            }) => self.parse_let_statemet(),
            Some(token::Token {
                typ: token::TokenType::Retrun,
                literal: _,
            }) => self.parse_return_statemet(),
            _ => None,
        }
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut p = ast::Program {
            statements: Vec::new(),
        };
        while !self.cur_token_is(&token::TokenType::Eof) {
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

    use bytes::Bytes;

    use super::Parser;
    use crate::ast::Node;
    use crate::token;
    use crate::{ast, lexer::Lexer};

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
        check_parser_error(&mut p);

        assert_eq!(program.statements.len(), 3);

        {
            let s = program.statements.get(0).unwrap();
            assert_let_statement(s, "x")
        }
        {
            let s = program.statements.get(1).unwrap();
            assert_let_statement(s, "y")
        }
        {
            let s = program.statements.get(2).unwrap();
            assert_let_statement(s, "foobar")
        }
    }

    #[test]
    #[ignore = "failed test"]
    fn test_error() {
        let input = r"
        let x 5;
        let = 10;
        let 838383;
        "
        .to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let _ = p.parse_program();
        check_parser_error(&mut p);
    }

    #[test]
    fn test_return_statement() {
        let input = r"
        return 5;
        return 10;
        return 993322;
        "
        .to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_error(&mut p);

        assert_eq!(program.statements.len(), 3);

        for s in program.statements {
            let rs = match &s {
                ast::statement::Statement::ReturnStatement(rs) => rs,
                other => panic!("unexpected statement {:?}", other),
            };
            assert_eq!(rs.token_literal(), "return")
        }
    }

    #[test]
    fn test_string() {
        let program = ast::Program {
            statements: vec![ast::statement::Statement::LetStatement(
                ast::statement::LetStatement {
                    token: token::Token::new(token::TokenType::Ident, Bytes::from("let")),
                    name: ast::expression::Identifier::new(token::Token::new(
                        token::TokenType::Ident,
                        Bytes::from("myVar"),
                    )),
                    value: Some(ast::expression::Expression::Identifier(
                        ast::expression::Identifier::new(token::Token::new(
                            token::TokenType::Ident,
                            Bytes::from("anotherVar"),
                        )),
                    )),
                },
            )],
        };

        assert_eq!(program.string(), "let myVar = anotherVar;".to_string())
    }

    fn assert_let_statement(s: &ast::statement::Statement, expected_name: &str) {
        assert_eq!(s.token_literal(), "let");

        let ls = match s {
            ast::statement::Statement::LetStatement(let_statement) => let_statement,
            _ => panic!("unexpected statement"),
        };
        assert_eq!(ls.name.token_literal(), expected_name);
    }

    fn check_parser_error(p: &mut Parser) {
        let errors = p.errors();
        if errors.is_empty() {
            return;
        }
        errors.iter().for_each(|pe| println!("{}", pe));
        panic!("failed");
    }
}
