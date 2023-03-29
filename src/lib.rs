#![warn(clippy::pedantic, clippy::nursery)]
mod tokenize;

pub fn foo(input: &str) -> Vec<String> {
    tokenize::tokenize(input)
}
