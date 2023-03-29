use crate::token::{Reserved, Token};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimaryNoun {
    StringLiteral { literal: String },
    Ident { ident: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Noun {
    pub modifier: Vec<PrimaryNoun>,
    pub head: PrimaryNoun,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    EndOfFile,
    UnexpectedToken { expected: String, actual: Token },
}

pub struct State<'a> {
    tokens: &'a [Token],
}

impl<'a> State<'a> {
    pub const fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub const fn new(tokens: &'a [Token]) -> Self {
        Self { tokens }
    }

    pub fn next(&mut self) -> Result<Token, ParseError> {
        match self.tokens {
            [] => Err(ParseError::EndOfFile),
            [tok, ..] => {
                self.tokens = &self.tokens[1..];
                Ok(tok.clone())
            }
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        match self.tokens {
            [] => None,
            [tok, ..] => Some(tok.clone()),
        }
    }

    // primary_noun = ident | "<" character* ">"
    #[allow(clippy::match_wildcard_for_single_variants)]
    pub fn parse_primary_noun(&mut self) -> Result<PrimaryNoun, ParseError> {
        let next = self.next()?;
        match next {
            Token::NormalIdent { ident } => Ok(PrimaryNoun::Ident { ident }),
            Token::StringLiteral { literal } => Ok(PrimaryNoun::StringLiteral { literal }),
            _ => Err(ParseError::UnexpectedToken {
                expected: "（識別子か文字列リテラル）".to_string(),
                actual: next.clone(),
            }),
        }
    }

    // noun = (primary_noun "'d")* primary_noun
    pub fn parse_noun(&mut self) -> Result<Noun, ParseError> {
        let mut pns = vec![];
        pns.push(self.parse_primary_noun()?);
        while let [Token::Reserved(Reserved::ApostropheD), ..] = self.tokens {
            self.tokens = &self.tokens[1..];
            pns.push(self.parse_primary_noun()?);
        }
        let head = pns.pop().unwrap();
        Ok(Noun {
            modifier: pns,
            head,
        })
    }

    // noun_list = noun
    //   | noun "ad" noun
    //   | noun "adit" noun ("," noun)+
    pub fn parse_noun_list(&mut self) -> Result<Vec<Noun>, ParseError> {
        let mut nouns = vec![];
        nouns.push(self.parse_noun()?);
        match self.peek() {
            Some(Token::Reserved(Reserved::Ad)) => {
                // noun "ad" noun
                self.next()?;
                nouns.push(self.parse_noun()?);
                Ok(nouns)
            }
            Some(Token::Reserved(Reserved::Adit)) => {
                // noun "adit" noun ("," noun)+
                self.next()?;
                nouns.push(self.parse_noun()?);

                let expect_comma = self.next()?;
                if expect_comma != Token::Reserved(Reserved::PunctuationComma) {
                    return Err(ParseError::UnexpectedToken {
                        expected: "（カンマ）".to_string(),
                        actual: expect_comma,
                    });
                }
                nouns.push(self.parse_noun()?);

                while let [Token::Reserved(Reserved::PunctuationComma), ..] = self.tokens {
                    self.tokens = &self.tokens[1..];
                    nouns.push(self.parse_noun()?);
                }

                Ok(nouns)
            }
            _ => Ok(nouns),
        }
    }
}
