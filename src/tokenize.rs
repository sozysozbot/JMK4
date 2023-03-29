#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum CharKind {
    WordConstituent,
    Space,
    SimplePunctuation,
    StartsStringLiteral,
    EndsStringLiteral,
}

fn classify_char(c: char) -> CharKind {
    match c {
        'a'..='z' | 'φ' | 'β' | 'ж' | '0'..='9' | '\'' | '-' => CharKind::WordConstituent,
        c if c.is_whitespace() => CharKind::Space,
        '.' | ',' | ':' => CharKind::SimplePunctuation,
        '<' => CharKind::StartsStringLiteral,
        '>' => CharKind::EndsStringLiteral,
        _ => panic!("unknown character {c}"),
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
    let tokens = to_words(input);
    tokens
        .into_iter()
        .flat_map(|pre_token| split_off_reserved(&pre_token))
        .collect()
}

const RESERVED_ENDING: [&str; 6] = ["'d", "'c", "'st", "-il", "-o", "'i"];

pub fn split_off_reserved(pre_token: &str) -> Vec<String> {
    for ending in RESERVED_ENDING {
        if let Some(remaining) = pre_token.strip_suffix(ending) {
            if !remaining.is_empty() {
                return vec![remaining.to_string(), ending.to_string()];
            }
        }
    }
    vec![pre_token.to_string()]
}

pub fn to_words(input: &str) -> Vec<String> {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    enum State {
        ExpectingWordInitial,
        WordInternal,
        StringLiteralInternal,
    }
    let mut words = vec![];
    let mut state = State::ExpectingWordInitial;

    let mut partial_word = String::new();

    for c in input.chars() {
        use CharKind::{
            EndsStringLiteral, SimplePunctuation, Space, StartsStringLiteral, WordConstituent,
        };
        use State::{ExpectingWordInitial, StringLiteralInternal, WordInternal};
        match (classify_char(c), state) {
            (WordConstituent, ExpectingWordInitial) => {
                partial_word.push(c);
                state = State::WordInternal;
            }
            (WordConstituent, WordInternal) => {
                partial_word.push(c);
            }
            (Space, ExpectingWordInitial) => { /* nothing is needed */ }
            (Space, WordInternal) => {
                words.push(partial_word.clone());
                partial_word = String::new();
                state = ExpectingWordInitial;
            }
            (SimplePunctuation, ExpectingWordInitial) => {
                words.push(c.to_string());
            }
            (SimplePunctuation, WordInternal) => {
                words.push(partial_word.clone());
                words.push(c.to_string());
                partial_word = String::new();
                state = ExpectingWordInitial;
            }
            (StartsStringLiteral, ExpectingWordInitial) => {
                partial_word.push(c);
                state = State::StringLiteralInternal;
            }
            (StartsStringLiteral, WordInternal) => {
                words.push(partial_word.clone());
                partial_word = c.to_string();
                state = State::StringLiteralInternal;
            }
            (EndsStringLiteral, StringLiteralInternal) => {
                partial_word.push(c);
                words.push(partial_word.clone());
                partial_word = String::new();
                state = ExpectingWordInitial;
            }
            (_, StringLiteralInternal) => {
                partial_word.push(c);
            }
            (EndsStringLiteral, _) => {
                panic!("Unmatched > encountered");
            }
        }
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplest() {
        assert_eq!(
            tokenize("selsurle es iu."),
            vec!["selsurle", "es", "iu", "."]
        );
    }

    #[test]
    fn number() {
        assert_eq!(
            tokenize("laozia jerldir lerj 10 ad 10 el 168 ad 218."),
            vec!["laozia", "jerldir", "lerj", "10", "ad", "10", "el", "168", "ad", "218", "."]
        );
    }

    #[test]
    fn case_ending() {
        assert_eq!(
            tokenize("kernumesaxm'st sides-il io elx shrlo is selsurle iu'c."),
            vec![
                "kernumesaxm",
                "'st",
                "sides",
                "-il",
                "io",
                "elx",
                "shrlo",
                "is",
                "selsurle",
                "iu",
                "'c",
                "."
            ]
        );
    }

    #[test]
    fn space_string_literal_nospace() {
        assert_eq!(
            tokenize("is jerldir'd xakant <selsurle>'c."),
            vec!["is", "jerldir", "'d", "xakant", "<selsurle>", "'c", "."]
        );
    }

    #[test]
    fn nospace_string_literal_nospace() {
        assert_eq!(
            tokenize("is jerldir'd xakant<selsurle>'c."),
            vec!["is", "jerldir", "'d", "xakant", "<selsurle>", "'c", "."]
        );
    }

    #[test]
    fn nospace_string_literal_space() {
        assert_eq!(
            tokenize("is jerldir'd xakant<selsurle> 'c."),
            vec!["is", "jerldir", "'d", "xakant", "<selsurle>", "'c", "."]
        );
    }

    #[test]
    fn space_string_literal_space() {
        assert_eq!(
            tokenize("is jerldir'd xakant <selsurle> 'c."),
            vec!["is", "jerldir", "'d", "xakant", "<selsurle>", "'c", "."]
        );
    }
}
