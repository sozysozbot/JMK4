use crate::token::Token;

pub enum PrimaryNoun {
    StringLiteral { literal: String },
    Ident { ident: String },
}

pub enum ParseError {
    EndOfFile,
    UnexpectedToken { expected: String, actual: Token },
}

type Parser<'a, T> = Result<(T, &'a [Token]), ParseError>;

pub fn parse_primary_noun(tokens: &[Token]) -> Parser<PrimaryNoun> {
    match tokens {
        [] => Err(ParseError::EndOfFile),
        [Token::NormalIdent { ident }, ..] => Ok((
            PrimaryNoun::Ident {
                ident: ident.clone(),
            },
            &tokens[1..],
        )),
        [Token::StringLiteral { literal }, ..] => Ok((
            PrimaryNoun::StringLiteral {
                literal: literal.clone(),
            },
            &tokens[1..],
        )),
        [tok, ..] => Err(ParseError::UnexpectedToken {
            expected: "（識別子か文字列リテラル）".to_string(),
            actual: tok.clone(),
        }),
    }
}
