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
    pub fn parse_primary_noun(&mut self) -> Result<PrimaryNoun, ParseError> {
        match self.tokens {
            [] => Err(ParseError::EndOfFile),
            [Token::NormalIdent { ident }, ..] => {
                self.tokens = &self.tokens[1..];
                Ok(PrimaryNoun::Ident {
                    ident: ident.clone(),
                })
            }
            [Token::StringLiteral { literal }, ..] => {
                self.tokens = &self.tokens[1..];
                Ok(PrimaryNoun::StringLiteral {
                    literal: literal.clone(),
                })
            }
            [tok, ..] => Err(ParseError::UnexpectedToken {
                expected: "（識別子か文字列リテラル）".to_string(),
                actual: tok.clone(),
            }),
        }
    }
    pub fn parse_noun(&mut self) -> Result<Noun, ParseError> {
        let mut pns = vec![];
        let pn = self.parse_primary_noun()?;
        pns.push(pn);
        loop {
            let ((), tokens2) = match self.tokens {
                [Token::Reserved(Reserved::ApostropheD), ..] => Ok(((), &self.tokens[1..])),
                _ => break,
            }?;

            self.tokens = tokens2;

            let pn = self.parse_primary_noun()?;
            pns.push(pn);
        }
        let head = pns.pop().unwrap();
        Ok(Noun {
            modifier: pns,
            head,
        })
    }
}
