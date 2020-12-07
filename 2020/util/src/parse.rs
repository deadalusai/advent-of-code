
use std::fmt;

pub enum ParseErr {
    EndOfInput,
    UnexpectedInput(char),
    ExpectedSingle(String),
    ExpectedMultiple(Vec<String>),
}

impl fmt::Debug for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErr::UnexpectedInput(c) => {
                write!(f, "unexpected character `{}`", c)?;
            },
            ParseErr::EndOfInput => {
                write!(f, "unexpected end of input")?;
            }
            ParseErr::ExpectedSingle(exp) => {
                write!(f, "expected {}", exp)?
            },
            ParseErr::ExpectedMultiple(expected) => {
                write!(f, "expected ")?;
                for (i, exp) in expected.iter().enumerate() {
                    if i == expected.len() - 1 {
                        write!(f, " or ")?;
                    }
                    else if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", exp)?
                }
            },
        };
        Ok(())
    }
}

impl ParseErr {
    pub fn end_of_input() -> ParseErr {
        ParseErr::EndOfInput
    }
    
    pub fn unexpected_input(c: char) -> ParseErr {
        ParseErr::UnexpectedInput(c)
    }
    
    pub fn expected_alpha() -> ParseErr {
        ParseErr::ExpectedSingle(format!("ident"))
    }

    pub fn expected_number() -> ParseErr {
        ParseErr::ExpectedSingle(format!("number"))
    }

    pub fn expected_token(token: &str) -> ParseErr {
        ParseErr::ExpectedSingle(format!("`{}`", token))
    }

    pub fn expected_end_of_input() -> ParseErr {
        ParseErr::ExpectedSingle(format!("end of input"))
    }

    pub fn combine(a: ParseErr, b: ParseErr) -> ParseErr {
        let mut a = match a {
            ParseErr::ExpectedSingle(a) => vec![a],
            ParseErr::ExpectedMultiple(a) => a,
            e => return e,
        };
        match b {
            ParseErr::ExpectedSingle(b) => a.push(b),
            ParseErr::ExpectedMultiple(b) => a.extend(b),
            e => return e,
        };
        ParseErr::ExpectedMultiple(a)
    }
}

#[derive(Debug)]
pub enum ParseToken<'a> {
    Special(&'a str),
    Alpha(&'a str),
    Numeric(&'a str),
}

impl<'a> ParseToken<'a> {
    fn raw(&self) -> &'a str {
        match self {
            ParseToken::Special(s) => s,
            ParseToken::Alpha(s) => s,
            ParseToken::Numeric(s) => s,
        }
    }
}

pub type ParseResult<'a, T> = Result<(&'a str, T), ParseErr>;

pub trait ParseResultEx<'a, T> {
    fn or_try(self, op: impl FnOnce() -> ParseResult<'a, T>) -> ParseResult<'a, T>;
}

impl<'a, T> ParseResultEx<'a, T> for ParseResult<'a, T> {
    fn or_try(self, op: impl FnOnce() -> ParseResult<'a, T>) -> ParseResult<'a, T> {
        match self {
            Ok(r) => Ok(r),
            Err(a) => match op() {
                Ok(r) => Ok(r),
                Err(b) => Err(ParseErr::combine(a, b))
            }
        }
    }
}

/// Consume a single token from the input string, returning (the remainder string, the token).
/// A token is one of:
/// - .
/// - ,
/// - a contiguous sequence of alpha characters
/// - a contiguous sequence of numeric characters
/// All whitespace is ignored.
pub fn consume<'a>(s: &'a str) -> ParseResult<ParseToken<'a>> {
    let first = s.chars().next()
        .ok_or(ParseErr::end_of_input())?;
    // Special character tokens
    if first == '.' || first == ',' {
        let (a, b) = s.split_at(1);
        return Ok((b.trim(), ParseToken::Numeric(a.trim())));
    }
    // Tokens are made up of characters of the same type as the first character.
    let (allowed, is_alpha) = match first {
        'a'..='z' => ('a'..='z', true),
        '0'..='9' => ('0'..='9', false),
        c => return Err(ParseErr::unexpected_input(c)),
    };
    let factory = if is_alpha { ParseToken::Alpha } else { ParseToken::Numeric };
    let (last, _) = s.char_indices()
        .take_while(|(_, c)| allowed.contains(c))
        .last()
        .unwrap();
    let (a, b) = s.split_at(last + 1);
    Ok((b.trim(), factory(a.trim())))
}

pub fn parse_i32(input: &str) -> ParseResult<i32> {
    let (input, token) = consume(input)?;
    let num = match token {
        ParseToken::Numeric(num) => {
            num.parse::<i32>()
               .map_err(|_| ParseErr::expected_number())
        },
        _ => Err(ParseErr::expected_number()),
    }?;
    Ok((input, num))
}

pub fn parse_alpha(input: &str) -> ParseResult<&str> {
    let (input, token) = consume(input)?;
    match token {
        ParseToken::Alpha(alpha) => Ok((input, alpha)),
        _ => Err(ParseErr::expected_alpha()),
    }
}

pub fn parse_token<'a>(input: &'a str, token: &str) -> ParseResult<'a, &'a str> {
    let (input, actual) = consume(input)?;
    if actual.raw() != token {
        return Err(ParseErr::expected_token(token));
    }
    Ok((input, actual.raw()))
}

pub fn parse_end(input: &str) -> ParseResult<()> {
    match consume(input) {
        Err(ParseErr::EndOfInput) => Ok((input, ())),
        _ => Err(ParseErr::expected_end_of_input()),
    }
}