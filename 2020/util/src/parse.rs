
use std::fmt;

#[derive(PartialEq, Eq)]
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
        ParseErr::ExpectedSingle(format!("alpha"))
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

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Special,
    Alpha,
    Numeric,
}

pub type ParseResult<'a, T> = Result<(&'a str, T), ParseErr>;

pub trait ParseResultEx<'a, T> {
    fn or_try(self, op: impl FnOnce() -> ParseResult<'a, T>) -> ParseResult<'a, T>;
    fn map_value<V>(self, op: impl FnOnce(T) -> V) -> ParseResult<'a, V>;
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
    
    fn map_value<V>(self, op: impl FnOnce(T) -> V) -> ParseResult<'a, V> {
        match self {
            Ok((input, v)) => Ok((input, op(v))),
            Err(e) => Err(e),
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
pub fn consume<'a>(s: &'a str) -> ParseResult<(TokenKind, &'a str)> {
    let s = s.trim();
    let first = s.chars().next()
        .ok_or(ParseErr::end_of_input())?;
    // Special character tokens
    if first == '.' || first == ',' {
        let (a, b) = s.split_at(1);
        return Ok((b.trim(), (TokenKind::Special, a.trim())));
    }
    // Tokens are made up of characters of the same type as the first character.
    let (allowed, kind) = match first {
        'a'..='z' => ('a'..='z', TokenKind::Alpha),
        '0'..='9' => ('0'..='9', TokenKind::Numeric),
        c => return Err(ParseErr::unexpected_input(c)),
    };
    let (last, _) = s.char_indices()
        .take_while(|(_, c)| allowed.contains(c))
        .last()
        .unwrap();
    let (a, b) = s.split_at(last + 1);
    Ok((b.trim(), (kind, a.trim())))
}

pub fn parse_i32(input: &str) -> ParseResult<i32> {
    let (input, token) = consume(input)?;
    let num = match token {
        (TokenKind::Numeric, num) => {
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
        (TokenKind::Alpha, alpha) => Ok((input, alpha)),
        _ => Err(ParseErr::expected_alpha()),
    }
}

pub fn parse_token<'a>(input: &'a str, token: &str) -> ParseResult<'a, &'a str> {
    let (input, (_, actual)) = consume(input)?;
    if actual != token {
        return Err(ParseErr::expected_token(token));
    }
    Ok((input, actual))
}

pub fn parse_end(input: &str) -> ParseResult<()> {
    match consume(input) {
        Err(ParseErr::EndOfInput) => Ok((input, ())),
        _ => Err(ParseErr::expected_end_of_input()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consume_special_characters() {
        let input = ",.";
        let (input, a) = consume(input).unwrap();
        assert_eq!(a, (TokenKind::Special, ","));
        let (input, a) = consume(input).unwrap();
        assert_eq!(a, (TokenKind::Special, "."));
        assert_eq!(input, "");
    }

    #[test]
    fn consume_numeric() {
        let input = "01234";
        let (input, a) = consume(input).unwrap();
        assert_eq!(a, (TokenKind::Numeric, "01234"));
        assert_eq!(input, "");
    }

    #[test]
    fn consume_alpha() {
        let input = "abcdef";
        let (input, a) = consume(input).unwrap();
        assert_eq!(a, (TokenKind::Alpha, "abcdef"));
        assert_eq!(input, "");
    }

    #[test]
    fn consume_ignores_whitespace() {
        let input = "abcdef 1 2aa, 3.";
        let (input, a) = consume(input).unwrap();
        assert_eq!(a, (TokenKind::Alpha, "abcdef"));
        let (input, a) = consume(input).unwrap();
        assert_eq!(a, (TokenKind::Numeric, "1"));
        let (input, a) = consume(input).unwrap();
        assert_eq!(a, (TokenKind::Numeric, "2"));
        let (input, a) = consume(input).unwrap();
        assert_eq!(a, (TokenKind::Alpha, "aa"));
        let (input, a) = consume(input).unwrap();
        assert_eq!(a, (TokenKind::Special, ","));
        let (input, a) = consume(input).unwrap();
        assert_eq!(a, (TokenKind::Numeric, "3"));
        let (input, a) = consume(input).unwrap();
        assert_eq!(a, (TokenKind::Special, "."));
        assert_eq!(input, "");
    }

    #[test]
    fn parse_i32_success() {
        let input = " 123 ";
        let (input, a) = parse_i32(input).unwrap();
        assert_eq!(a, 123);
        assert_eq!(input, "");
    }

    #[test]
    fn parse_i32_fail() {
        let input = " xxx ";
        let err = parse_i32(input).unwrap_err();
        assert_eq!(err, ParseErr::expected_number());
    }

    #[test]
    fn parse_alpha_success() {
        let input = " xxx ";
        let (input, a) = parse_alpha(input).unwrap();
        assert_eq!(a, "xxx");
        assert_eq!(input, "");
    }

    #[test]
    fn parse_alpha_fail() {
        let input = " 123 ";
        let err = parse_alpha(input).unwrap_err();
        assert_eq!(err, ParseErr::expected_alpha());
    }

    #[test]
    fn parse_token_success() {
        let input = " xxx ";
        let (input, a) = parse_token(input, "xxx").unwrap();
        assert_eq!(a, "xxx");
        assert_eq!(input, "");

        let input = " 123 ";
        let (input, a) = parse_token(input, "123").unwrap();
        assert_eq!(a, "123");
        assert_eq!(input, "");

        let input = " , ";
        let (input, a) = parse_token(input, ",").unwrap();
        assert_eq!(a, ",");
        assert_eq!(input, "");
    }

    #[test]
    fn parse_token_fail() {
        let input = " xxx ";
        let err = parse_token(input, "yyy").unwrap_err();
        assert_eq!(err, ParseErr::expected_token("yyy"));
    }

    #[test]
    fn parse_end_success() {
        let input = "a";
        let (input, _) = parse_alpha(input).unwrap();
        let (input, a) = parse_end(input).unwrap();
        assert_eq!(a, ());
        assert_eq!(input, "");
    }

    #[test]
    fn parse_end_fail() {
        let input = "a";
        let err = parse_end(input).unwrap_err();
        assert_eq!(err, ParseErr::expected_end_of_input());
    }

    #[test]
    fn parser_chaining() { 

        fn parse_2(input: &str) -> ParseResult<(&str, i32)> {
            let (input, a) = parse_alpha(input)?;
            let (input, b) = parse_i32(input)?;
            Ok((input, (a, b)))
        }

        fn parse_exact(input: &str) -> ParseResult<()> {
            let (input, _) = parse_token(input, "aaa")?;
            let (input, _) = parse_token(input, "bbb")?;
            let (input, _) = parse_token(input, "ccc")?;
            Ok((input, ()))
        }

        let input = "aaa bbb ccc hello 123";
        let (input, a) = parse_exact(input).unwrap();
        assert_eq!(a, ());
        let (input, a) = parse_2(input).unwrap();
        assert_eq!(a, ("hello", 123));
        assert_eq!(input, "");
    }

    #[test]
    fn parser_loops() {

        fn parse_foo(input: &str) -> ParseResult<()> {
            let (input, _) = parse_token(input, "aaa")?;
            let (input, _) = parse_token(input, "bbb")?;
            Ok((input, ()))
        }

        let input = "aaa bbb aaa bbb aaa bbb ccc";
        let mut input = input;
        while let Ok((input_, ())) = parse_foo(input) {
            input = input_;
        }
        assert_eq!(input, "ccc");
    }

    #[test]
    fn parser_loops_with_explicit_termination() {

        fn try_parse_next(input: &str) -> ParseResult<Option<&'static str>> {
            // check for terminator token
            if let Ok((input, _)) = parse_token(input, "ccc") {
                return Ok((input, None));
            }
            let (input, _) = parse_token(input, "aaa")?;
            let (input, _) = parse_token(input, "bbb")?;
            Ok((input, Some("foo bar")))
        }

        let input = "aaa bbb aaa bbb aaa bbb ccc";
        let mut input = input;
        while let Ok((input_, result)) = try_parse_next(input) {
            input = input_;
            if let Some(result) = result {
                assert_eq!(result, "foo bar");
            }
        }
        assert_eq!(input, "");
    }

    #[test]
    fn parser_combination_with_or_try() {

        fn parse_num(input: &str) -> ParseResult<i32> {
            parse_token(input, "zero").map_value(|_| 0)
                .or_try(|| parse_i32(input))
        }

        let input = "123";
        let (input, a) = parse_num(input).unwrap();
        assert_eq!(a, 123);
        assert_eq!(input, "");

        let input = "zero";
        let (input, a) = parse_num(input).unwrap();
        assert_eq!(a, 0);
        assert_eq!(input, "");
    }
}