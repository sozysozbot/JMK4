#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum CharKind {
    AlphabetOrNumeral,
    Space,
    SimplePunctuation,
    // PossiblyTokenDivider,
}

fn classify_char(c: char) -> CharKind {
    match c {
        'a'..='z' | 'φ' | 'β' | 'ж' | '0'..='9' => CharKind::AlphabetOrNumeral,
        c if c.is_whitespace() => CharKind::Space,
        '.' | ',' => CharKind::SimplePunctuation,
        // '\'' | '-' => CharKind::PossiblyTokenDivider,
        _ => panic!("unknown character {c}"),
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
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
            (AlphabetOrNumeral, ExpectingWordInitial) => {
                partial_word.push(c);
                state = State::WordInternal;
            }
            (AlphabetOrNumeral, WordInternal) => {
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
}
