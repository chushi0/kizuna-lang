use super::ast;
use lalrpop_util::{lalrpop_mod, lexer::Token, ParseError};

lalrpop_mod!(grammar, "/compiler/grammar.rs");

pub fn parse(input: &str) -> Result<ast::Script, ParseError<usize, Token<'_>, &'static str>> {
    grammar::FileParser::new().parse(input)
}
