#![warn(clippy::pedantic, clippy::nursery)]

use parser::parse_primary_noun;
use token::Token;
mod parser;
mod token;
mod tokenize;

pub fn foo(input: &str) -> Vec<Token> {
    token::tokenize(input)
}

pub fn bar(tokens: &[Token]) {
    parse_primary_noun(tokens);
}
