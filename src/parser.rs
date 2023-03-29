use crate::token::{Reserved, Token};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimaryNoun {
    StringLiteral { literal: String },
    Ident { ident: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    EndOfFile,
    UnexpectedToken { expected: String, actual: Token },
}

type ParserResult<'a, T> = Result<(T, &'a [Token]), ParseError>;

pub fn parse_primary_noun(tokens: &[Token]) -> ParserResult<PrimaryNoun> {
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Noun {
    pub modifier: Vec<PrimaryNoun>,
    pub head: PrimaryNoun,
}

// pub fn consume_or_die(expected_tok: Token, tokens: &[Token]) -> ParserResult<()> {
//     match tokens {
//         [] => Err(ParseError::EndOfFile),
//         [actual_tok, ..] if actual_tok == &expected_tok => Ok(((), &tokens[1..])),
//         [actual, ..] => Err(ParseError::UnexpectedToken {
//             expected: format!("{expected_tok:?}"),
//             actual: actual.clone(),
//         }),
//     }
// }

pub fn parse_noun(tokens: &[Token]) -> ParserResult<Noun> {
    let mut pns = vec![];
    let (pn, mut tokens) = parse_primary_noun(tokens)?;
    pns.push(pn);
    loop {
        let ((), tokens2) = match tokens {
            [Token::Reserved(Reserved::ApostropheD), ..] => Ok(((), &tokens[1..])),
            _ => break,
        }?;

        let (pn, tokens3) = parse_primary_noun(tokens2)?;
        pns.push(pn);
        tokens = tokens3;
    }
    let head = pns.pop().unwrap();
    Ok((
        Noun {
            modifier: pns,
            head,
        },
        tokens,
    ))
}
