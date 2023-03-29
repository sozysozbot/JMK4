#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum CharKind {
    WordConstituent,
    Space,
    SimplePunctuation,
}

fn classify_char(c: char) -> CharKind {
    match c {
        'a'..='z' | 'φ' | 'β' | 'ж' | '0'..='9' | '\'' | '-' => CharKind::WordConstituent,
        c if c.is_whitespace() => CharKind::Space,
        '.' | ',' => CharKind::SimplePunctuation,
        _ => panic!("unknown character {c}"),
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
    let tokens = tokenize1(input);
    tokens
        .into_iter()
        .flat_map(|pre_token| split_off_reserved(&pre_token))
        .collect()
}

const RESERVED_ENDING: [&str; 6] = ["'d", "'c", "'st", "-il", "-o", "'i"];

pub fn split_off_reserved(pre_token: &str) -> Vec<String> {
    for ending in RESERVED_ENDING {
        if let Some(remaining) = pre_token.strip_suffix(ending) {
            return vec![remaining.to_string(), ending.to_string()];
        }
    }
    vec![pre_token.to_string()]
}

pub fn tokenize1(input: &str) -> Vec<String> {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    enum State {
        ExpectingWordInitial,
        WordInternal,
    }
    let mut ans = vec![];
    let mut state = State::ExpectingWordInitial;

    let mut partial_word = String::new();

    for c in input.chars() {
        use CharKind::*;
        use State::*;
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
                ans.push(partial_word.clone());
                partial_word = String::new();
                state = ExpectingWordInitial;
            }
            (SimplePunctuation, ExpectingWordInitial) => {
                ans.push(c.to_string());
            }
            (SimplePunctuation, WordInternal) => {
                ans.push(partial_word.clone());
                ans.push(c.to_string());
                partial_word = String::new();
                state = ExpectingWordInitial;
            }
        }
    }
    ans
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
        )
    }
}
