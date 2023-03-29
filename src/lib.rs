#![warn(clippy::pedantic, clippy::nursery)]

use token::Token;
mod token;
mod tokenize;

pub fn foo(input: &str) -> Vec<Token> {
    token::tokenize(input)
}
