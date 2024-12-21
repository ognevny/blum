use ast::{lexer::Lexer, parser::Parser};
use std::env::args;

pub mod ast;
pub mod error;

use error::Handler;

fn main() {
    let path = args().nth(1);

    match path {
        Some(path) => {
            Handler::set_source_file(path.clone());

            let file_contents = std::fs::read_to_string(path.clone())
                .inspect_err(|e| {
                    crate::error(1, format!("error opening the file at `{path}`, error: {e}"))
                })
                .unwrap();

            let mut lexer = Lexer::new(file_contents);
            let tokens = lexer.scan_tokens();
            //println!("{tokens:#?}");
            let mut parser = Parser::new(tokens);
            let ast = parser.parse();

            println!("{ast:#?}")
        }
        None => crate::error(1, "no source file given"),
    }
}

fn error(pos: usize, message: impl Into<String>) {
    Handler::error(pos, message);
}

fn error_at_token(token: &ast::Token, message: impl Into<String>) {
    let message: String = message.into();
    let pos = token.line;
    error(pos, message);
}
