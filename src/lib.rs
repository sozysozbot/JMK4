#![warn(clippy::pedantic, clippy::nursery)]

use crate::parser::{Noun, State, PrimaryNoun};
mod parser;
mod token;
mod tokenize;

pub fn foo() {
    let tokens = token::tokenize("xakant");
    let mut parser_state = State::new(&tokens);
    let noun = parser_state.parse_primary_noun().unwrap();
    assert_eq!(
        noun,
        PrimaryNoun::Ident {
            ident: "xakant".to_string()
        }
    );
    assert!(parser_state.is_empty());
}

pub fn bar() {
    let tokens = token::tokenize("jerldir'd xakant");
    let mut parser_state = State::new(&tokens);
    let noun = parser_state.parse_noun().unwrap();
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
    assert!(parser_state.is_empty());
}

#[test]
fn parsing_primary_noun() {
    foo();
}

#[test]
fn parsing_noun() {
    bar();
}
