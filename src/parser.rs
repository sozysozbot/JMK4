use crate::token::{Preposition, Reserved, Token};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimaryNoun {
    StringLiteral { literal: String },
    Ident { ident: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Verb(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Noun {
    pub modifier: Vec<PrimaryNoun>,
    pub head: PrimaryNoun,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Case {
    Preposition(Preposition),
    ApostropheC,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NounsWithCase {
    pub nouns: Vec<Noun>,
    pub case: Case,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sentence {
    VarDecl(Noun, Noun),
    PredicateDecl {
        noun_list: Vec<Noun>,
        verb: Verb,
        cond: Cond,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventCond {
    noun: Noun,
    verb: Verb,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CondElem {
    pub noun: Noun,
    pub verb: Verb,
    pub nouns_with_case: Option<NounsWithCase>,
}

// noun verb nouns_with_case* ("mal" noun verb nouns_with_case*)*
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cond(pub Vec<CondElem>);

// import = "lus" (module "'d")+ (
//        ident | ident "ad" ident | ident "adit" ident ("," ident)+
//      )
pub struct Import {
    module_path: Vec<Module>,
    idents: Vec<String>,
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

    // preposition = "el" | "lerj" | "fal" | "cecioj"
    // nouns_with_case = preposition noun_list | noun_list "'c"
    pub fn parse_nouns_with_case(&mut self) -> Result<NounsWithCase, ParseError> {
        if let Some(Token::Reserved(Reserved::Preposition(p))) = self.peek() {
            self.next()?;
            let nouns = self.parse_noun_list()?;
            Ok(NounsWithCase {
                nouns,
                case: Case::Preposition(p),
            })
        } else {
            let nouns = self.parse_noun_list()?;
            let next = self.next()?;
            match next {
                Token::Reserved(Reserved::ApostropheC) => Ok(NounsWithCase {
                    nouns,
                    case: Case::ApostropheC,
                }),
                _ => Err(ParseError::UnexpectedToken {
                    expected: "'c".to_string(),
                    actual: next,
                }),
            }
        }
    }

    // var_decl = noun "es" noun
    pub fn parse_var_decl(&mut self) -> Result<Sentence, ParseError> {
        let noun1 = self.parse_noun()?;
        self.consume_or_die(Reserved::Es, "es")?;
        let noun2 = self.parse_noun()?;
        Ok(Sentence::VarDecl(noun1, noun2))
    }

    // verb = ident
    pub fn parse_verb(&mut self) -> Result<Verb, ParseError> {
        let next = self.next()?;
        match next {
            Token::NormalIdent { ident } => Ok(Verb(ident)),
            _ => Err(ParseError::UnexpectedToken {
                expected: "（識別子）".to_string(),
                actual: next.clone(),
            }),
        }
    }

    pub fn consume_or_die(&mut self, reserved: Reserved, msg: &str) -> Result<(), ParseError> {
        let next = self.next()?;
        if next == Token::Reserved(reserved) {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: msg.to_string(),
                actual: next,
            })
        }
    }

    // event_cond = noun "'st" verb "-il" "io"
    pub fn parse_event_cond(&mut self) -> Result<EventCond, ParseError> {
        let noun = self.parse_noun()?;
        self.consume_or_die(Reserved::ApostropheSt, "'st")?;
        let verb = self.parse_verb()?;
        self.consume_or_die(Reserved::HyphenIl, "-il")?;
        self.consume_or_die(Reserved::Io, "io")?;
        Ok(EventCond { noun, verb })
    }

    pub fn parse_cond_elem(&mut self) -> Result<CondElem, ParseError> {
        let noun = self.parse_noun()?;
        let verb = self.parse_verb()?;
        // When `felx`, `mal`, `.` comes, the parsing stops
        let nouns_with_case = if let Some(Token::Reserved(
            Reserved::Felx | Reserved::Mal | Reserved::PunctuationPeriod,
        )) = self.peek()
        {
            None
        } else {
            let nouns_with_case = self.parse_nouns_with_case()?;
            Some(nouns_with_case)
        };
        Ok(CondElem {
            noun,
            verb,
            nouns_with_case,
        })
    }

    // cond = noun verb nouns_with_case* ("mal" noun verb nouns_with_case*)*
    pub fn parse_cond(&mut self) -> Result<Cond, ParseError> {
        let mut cond_elems = vec![self.parse_cond_elem()?];
        while let [Token::Reserved(Reserved::Mal), ..] = self.tokens {
            self.tokens = &self.tokens[1..];
            cond_elems.push(self.parse_cond_elem()?);
        }
        Ok(Cond(cond_elems))
    }

    // predicate_decl = noun_list "'st" verb "-o" ":" cond
    pub fn parse_predicate_decl(&mut self) -> Result<Sentence, ParseError> {
        let noun_list = self.parse_noun_list()?;
        self.consume_or_die(Reserved::ApostropheSt, "'st")?;
        let verb = self.parse_verb()?;
        self.consume_or_die(Reserved::HyphenO, "-o")?;
        self.consume_or_die(Reserved::PunctuationColon, ":")?;
        let cond = self.parse_cond()?;
        Ok(Sentence::PredicateDecl {
            noun_list,
            verb,
            cond,
        })
    }

    // module = ident
    pub fn parse_module(&mut self) -> Result<Module, ParseError> {
        let next = self.next()?;
        match next {
            Token::NormalIdent { ident } => Ok(Module(ident)),
            _ => Err(ParseError::UnexpectedToken {
                expected: "（識別子）".to_string(),
                actual: next.clone(),
            }),
        }
    }

    // import = "lus" (module "'d")+ (
    //        ident | ident "ad" ident | ident "adit" ident ("," ident)+
    //      )
    pub fn parse_import(&mut self) -> Result<Import, ParseError> {
        self.consume_or_die(Reserved::Lus, "lus")?;
        let mut module_path = vec![self.parse_module()?];
        self.consume_or_die(Reserved::ApostropheD, "'d")?;

        // needs to read two tokens ahead.
        // If `'d`, it's `module 'd`
        // If period, it's `ident .`
        // If `ad`, it's `ident ad`
        // If `adit`, it's `ident adit`
        loop {
            match self.tokens {
                [] | [Token::NormalIdent { .. }] => return Err(ParseError::EndOfFile),
                [Token::NormalIdent { ident }, Token::Reserved(Reserved::ApostropheD), ..] => {
                    module_path.push(Module(ident.to_string()));
                    self.tokens = &self.tokens[2..];
                    continue;
                }
                [Token::NormalIdent { ident }, Token::Reserved(Reserved::PunctuationPeriod), ..] => {
                    let import = Import {
                        module_path,
                        idents: vec![ident.to_string()],
                    };
                    self.tokens = &self.tokens[1..];
                    return Ok(import);
                }
                [Token::NormalIdent { ident }, Token::Reserved(Reserved::Ad), Token::NormalIdent { ident: ident2 }] =>
                {
                    let import = Import {
                        module_path,
                        idents: vec![ident.to_string(), ident2.to_string()],
                    };
                    self.tokens = &self.tokens[3..];
                    return Ok(import);
                }
                [Token::NormalIdent { ident }, Token::Reserved(Reserved::Adit), Token::NormalIdent { ident: ident2 }, Token::Reserved(Reserved::PunctuationComma), Token::NormalIdent { ident: ident3 }, ..] =>
                {
                    let mut ident_list =
                        vec![ident.to_string(), ident2.to_string(), ident3.to_string()];
                    self.tokens = &self.tokens[5..];

                    while let [Token::Reserved(Reserved::PunctuationComma), Token::NormalIdent { ident }, ..] =
                        self.tokens
                    {
                        self.tokens = &self.tokens[2..];
                        ident_list.push(ident.to_string());
                    }

                    return Ok(Import {
                        module_path,
                        idents: ident_list,
                    });
                }
                [Token::NormalIdent { .. }, tok, ..] => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "'d / . / ad / adit".to_string(),
                        actual: tok.clone(),
                    })
                }

                [tok, ..] => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "（識別子）".to_string(),
                        actual: tok.clone(),
                    })
                }
            }
        }
    }
}
