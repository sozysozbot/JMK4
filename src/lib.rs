#![warn(clippy::pedantic, clippy::nursery)]

use parser::parse_noun;
use token::Token;

use crate::parser::{parse_primary_noun, Noun, ParserState, PrimaryNoun};
mod parser;
mod token;
mod tokenize;

pub fn foo(input: &str) -> Vec<Token> {
    token::tokenize(input)
}

pub fn bar(tokens: &[Token]) {
    parse_noun(tokens);
}

#[test]
fn parsing_primary_noun() {
    let tokens = token::tokenize("xakant");
    let mut parser_state = ParserState::new(&tokens);
    println!("{tokens:?}");
    let noun = parser_state.parse_primary_noun().unwrap();
    assert_eq!(
        noun,
        PrimaryNoun::Ident {
            ident: "xakant".to_string()
        }
    );
    assert!(parser_state.is_empty());
}

#[test]
fn parsing_noun() {
    let tokens = token::tokenize("jerldir'd xakant");
    let (noun, tokens) = parse_noun(&tokens).unwrap();
    assert_eq!(
        noun,
        Noun {
            modifier: vec![PrimaryNoun::Ident {
                ident: "jerldir".to_string()
            }],
            head: PrimaryNoun::Ident {
                ident: "xakant".to_string()
            }
        }
    );
    assert_eq!(tokens, vec![]);
}
