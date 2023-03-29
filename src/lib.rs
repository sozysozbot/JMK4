#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_panics_doc)]

use parser::NounsWithCase;

use crate::parser::{Noun, PrimaryNoun, State};
mod parser;
mod token;
mod tokenize;

#[must_use]
pub fn noun_from_ident(ident: &str) -> Noun {
    Noun {
        modifier: vec![],
        head: PrimaryNoun::Ident {
            ident: ident.to_string(),
        },
    }
}

#[must_use]
pub fn primary_noun_from_ident(ident: &str) -> PrimaryNoun {
    PrimaryNoun::Ident {
        ident: ident.to_string(),
    }
}

pub fn test_primary_noun() {
    let tokens = token::tokenize("xakant");
    let mut parser_state = State::new(&tokens);
    let noun = parser_state.parse_primary_noun().unwrap();
    assert!(parser_state.is_empty());
    assert_eq!(noun, primary_noun_from_ident("xakant"));
}

pub fn test_noun() {
    let tokens = token::tokenize("jerldir'd xakant");
    let mut parser_state = State::new(&tokens);
    let noun = parser_state.parse_noun().unwrap();
    assert!(parser_state.is_empty());
    assert_eq!(
        noun,
        Noun {
            modifier: vec![primary_noun_from_ident("jerldir")],
            head: primary_noun_from_ident("xakant")
        }
    );
}

pub fn test_noun_list() {
    let tokens = token::tokenize("jerldir'd xakant adit kernumesaxm, deln");
    let mut parser_state = State::new(&tokens);
    let noun = parser_state.parse_noun_list().unwrap();
    assert!(parser_state.is_empty());
    assert_eq!(
        noun,
        vec![
            Noun {
                modifier: vec![primary_noun_from_ident("jerldir")],
                head: primary_noun_from_ident("xakant")
            },
            noun_from_ident("kernumesaxm"),
            noun_from_ident("deln"),
        ]
    );
}

pub fn test_nouns_with_case() {
    let tokens = token::tokenize("lerj 10 ad 10");
    let mut parser_state = State::new(&tokens);
    let noun = parser_state.parse_nouns_with_case().unwrap();
    assert_eq!(
        noun,
        NounsWithCase {
            case: parser::Case::Preposition(token::Preposition::Lerj),
            nouns: vec![noun_from_ident("10"), noun_from_ident("10")]
        }
    );
}

#[test]
fn parsing_primary_noun() {
    test_primary_noun();
}

#[test]
fn parsing_noun() {
    test_noun();
}

#[test]
fn parsing_noun_list() {
    test_noun_list();
}

#[test]
fn parsing_nouns_with_case() {
    test_nouns_with_case();
}
