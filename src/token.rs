#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Reserved {
    Preposition(Preposition),
    ApostropheD,
    ApostropheC,
    ApostropheI,
    ApostropheSt,
    Adit,
    Ad,
    PunctuationComma,
    PunctuationPeriod,
    PunctuationColon,
    Elx,
    Shrlo,
    Melx,
    Felx,
    Mea,
    Mal,
    Es,
    Lus,
    Io,
    HyphenO,
    HyphenIl,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Preposition {
    El,
    Lerj,
    Fal,
    Cecioj,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]

pub enum Token {
    StringLiteral { literal: String },
    NormalIdent { ident: String },
    Reserved(Reserved),
}

impl Token {
    pub fn from(tok: &str) -> Self {
        match tok {
            "el" => Self::Reserved(Reserved::Preposition(Preposition::El)),
            "lerj" => Self::Reserved(Reserved::Preposition(Preposition::Lerj)),
            "fal" => Self::Reserved(Reserved::Preposition(Preposition::Fal)),
            "cecioj" => Self::Reserved(Reserved::Preposition(Preposition::Cecioj)),
            "'d" => Self::Reserved(Reserved::ApostropheD),
            "'c" => Self::Reserved(Reserved::ApostropheC),
            "'i" => Self::Reserved(Reserved::ApostropheI),
            "'st" => Self::Reserved(Reserved::ApostropheSt),
            "adit" => Self::Reserved(Reserved::Adit),
            "ad" => Self::Reserved(Reserved::Ad),
            "," => Self::Reserved(Reserved::PunctuationComma),
            "." => Self::Reserved(Reserved::PunctuationPeriod),
            ":" => Self::Reserved(Reserved::PunctuationColon),
            "elx" => Self::Reserved(Reserved::Elx),
            "shrlo" => Self::Reserved(Reserved::Shrlo),
            "melx" => Self::Reserved(Reserved::Melx),
            "felx" => Self::Reserved(Reserved::Felx),
            "mea" => Self::Reserved(Reserved::Mea),
            "mal" => Self::Reserved(Reserved::Mal),
            "es" => Self::Reserved(Reserved::Es),
            "lus" => Self::Reserved(Reserved::Lus),
            "io" => Self::Reserved(Reserved::Io),
            "-o" => Self::Reserved(Reserved::HyphenO),
            "-il" => Self::Reserved(Reserved::HyphenIl),
            tok if tok.starts_with('<') && tok.ends_with('>') => Self::StringLiteral {
                literal: tok.to_string(),
            },
            tok => Self::NormalIdent {
                ident: tok.to_string(),
            },
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    super::tokenize::tokenize(input)
        .into_iter()
        .map(|s| Token::from(&s))
        .collect()
}

#[test]
fn parsing_primary_noun() {
    let tokens = tokenize("xakant");
    assert_eq!(
        tokens,
        vec![Token::NormalIdent {
            ident: "xakant".to_string()
        }]
    );
}
