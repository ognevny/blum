use super::ParserException;
use crate::{
    ast::{expr, statement as stmt, token as tk, Lexer},
    throw,
};

pub struct Parser {
    input: Vec<tk::Token>,
    ast: Vec<stmt::Statement>,
    progress: usize,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let input = lexer.lex();
        Self {
            input,
            ast: vec![],
            progress: 0,
        }
    }

    pub fn advance(&mut self) -> Option<tk::Token> {
        self.progress += 1;
        self.input.get(self.progress - 1).cloned()
    }

    pub fn expect(&mut self, ttype: tk::TokenType) -> Option<tk::Token> {
        match self.advance() {
            Some(t) if t.ttype == ttype => Some(t),
            Some(t) => {
                let found = t.ttype.clone();
                let exception = ParserException::expected_error(ttype, found, (t.line, t.pos));
                throw!(exception);
                None
            }
            _ => None,
        }
    }

    pub fn finish(self) -> Vec<stmt::Statement> {
        self.ast
    }

    pub fn read_expression(&mut self) -> Option<expr::Expr> {
        let next = self.advance()?;

        // leave it like this for now
        match next.ttype {
            tk::TokenType::Number => {
                let src = &next.literal.unwrap();
                if let Ok(num) = i128::from_str_radix(src, 10) {
                    return Some(expr::Expr::Value(expr::Value::Int(num)));
                } else if let Ok(num) = src.parse::<f64>() {
                    return Some(expr::Expr::Value(expr::Value::Float(num)));
                }

                unreachable!()
            }
            tk::TokenType::String => {
                return Some(expr::Expr::Value(expr::Value::String(
                    next.literal.unwrap(),
                )));
            }
            _ => todo!(),
        }
    }

    pub fn try_let_statement(&mut self) -> Option<stmt::Statement> {
        let _let_kw = self.expect(tk::TokenType::Let);
        let name = self.expect(tk::TokenType::Identifier)?;
        let name = name.lexeme;
        let _eq = self.expect(tk::TokenType::Equal);
        let value = self.read_expression()?;

        let ret_let = stmt::Let { value, name };

        Some(stmt::Statement::Let(ret_let))
    }
}

#[cfg(test)]
mod tests {
    use super::{expr, stmt, Lexer, Parser};

    #[test]
    fn basic_let() {
        let source = "let a = 10";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);

        let let_stmt = parser.try_let_statement().unwrap();
        let rhand = stmt::Let {
            name: "a".to_owned(),
            value: expr::Expr::Value(expr::Value::Int(10)),
        };
        assert_eq!(let_stmt, stmt::Statement::Let(rhand))
    }
}
