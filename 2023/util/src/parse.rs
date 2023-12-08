
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct InputPos(usize);

#[derive(PartialEq, Eq)]
pub enum ParseErr {
    EndOfInput       (InputPos),
    UnexpectedInput  (InputPos, char),
    ExpectedSingle   (InputPos, String),
    ExpectedMultiple (InputPos, Vec<String>),
}

impl ParseErr {
    fn pos(&self) -> InputPos {
        match self { 
            ParseErr::EndOfInput(pos) => *pos,
            ParseErr::UnexpectedInput(pos, _) => *pos,
            ParseErr::ExpectedSingle(pos, _) => *pos,
            ParseErr::ExpectedMultiple(pos, _) => *pos,
        }
    }
}

impl fmt::Debug for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErr::UnexpectedInput(pos, c) => {
                write!(f, "unexpected character `{}` at offset {}", c, pos.0)?;
            },
            ParseErr::EndOfInput(_) => {
                write!(f, "unexpected end of input")?;
            }
            ParseErr::ExpectedSingle(pos, exp) => {
                write!(f, "expected {} at offset {}", exp, pos.0)?
            },
            ParseErr::ExpectedMultiple(pos, expected) => {
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
                write!(f, " at offset {}", pos.0)?;
            },
        };
        Ok(())
    }
}

impl ParseErr {
    pub fn end_of_input(pos: InputPos) -> ParseErr {
        ParseErr::EndOfInput(pos)
    }
    
    pub fn unexpected_input(pos: InputPos, c: char) -> ParseErr {
        ParseErr::UnexpectedInput(pos, c)
    }
    
    pub fn expected_alpha(pos: InputPos) -> ParseErr {
        ParseErr::ExpectedSingle(pos, format!("alpha"))
    }

    pub fn expected_number(pos: InputPos) -> ParseErr {
        ParseErr::ExpectedSingle(pos, format!("number"))
    }

    pub fn expected_token(pos: InputPos, token: &str) -> ParseErr {
        ParseErr::ExpectedSingle(pos, format!("`{}`", token))
    }

    pub fn expected_end_of_input(pos: InputPos) -> ParseErr {
        ParseErr::ExpectedSingle(pos, format!("end of input"))
    }

    pub fn combine(a: ParseErr, b: ParseErr) -> ParseErr {
        let pos = a.pos();
        let mut errors = match a {
            ParseErr::ExpectedSingle(_, a) => vec![a],
            ParseErr::ExpectedMultiple(_, a) => a,
            e => return e,
        };
        match b {
            ParseErr::ExpectedSingle(_, b) => errors.push(b),
            ParseErr::ExpectedMultiple(_, b) => errors.extend(b),
            e => return e,
        };
        ParseErr::ExpectedMultiple(pos, errors)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Symbol,
    Alpha,
    Numeric,
}

pub type ParseResult<'a, T> = Result<(Input<'a>, T), ParseErr>;

pub trait ParseResultEx<'a, T> {
    fn or_try(self, op: impl FnOnce() -> ParseResult<'a, T>) -> ParseResult<'a, T>;
    fn map_val<V>(self, op: impl FnOnce(T) -> V) -> ParseResult<'a, V>;
    fn val<V>(self, val: V) -> ParseResult<'a, V>;
}

impl<'a, T> ParseResultEx<'a, T> for ParseResult<'a, T> {
    fn or_try(self, op: impl FnOnce() -> ParseResult<'a, T>) -> ParseResult<'a, T> {
        match self {
            Ok(r) => Ok(r),
            Err(a) => match op() {
                Ok(r) => Ok(r),
                Err(b) => Err(ParseErr::combine(a, b)),
            }
        }
    }
    
    fn map_val<V>(self, op: impl FnOnce(T) -> V) -> ParseResult<'a, V> {
        match self {
            Ok((input, v)) => Ok((input, op(v))),
            Err(e) => Err(e),
        }
    }
    
    fn val<V>(self, val: V) -> ParseResult<'a, V> {
        match self {
            Ok((input, _)) => Ok((input, val)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Input<'a> {
    source: &'a str,
    offset: usize,
}

impl<'a> fmt::Debug for Input<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Input[{}]", self.remaining())
    }
}

impl<'a> Input<'a> {
    pub fn new(source: &'a str) -> Self {
        let input = Input { source, offset: 0 };
        input.consume_ws()
    }

    pub fn remaining(&self) -> &'a str {
        &self.source[self.offset..]
    }

    pub fn source(&self) -> &'a str {
        self.source
    }

    pub fn pos(&self) -> InputPos {
        InputPos(self.offset)
    }

    fn offset(&self, offset: usize) -> Self {
        Input { source: self.source, offset: self.offset + offset }
    }

    fn consume(self, mut pred: impl FnMut(&char) -> bool) -> (Self, &'a str) {
        let source = self.remaining();
        let last = source
            .char_indices()
            .find(|(_, c)| !pred(c))
            .map(|x| x.0)
            .unwrap_or(source.len());
    
        let input = self.offset(last);
        let consumed = &source[..last];
        (input, consumed)
    }
    
    fn consume_ws(self) -> Self {
        self.consume(|c| c.is_whitespace()).0
    }
}

#[cfg(test)]
mod consume_tests {
    use super::*;

    #[test]
    fn constructor() {
        let input = Input::new("   new ");
        assert_eq!(input.remaining(), "new ");
        assert_eq!(input.offset, 3);
    }

    #[test]
    fn offset() {
        let input = Input::new("abcdef");
        let input = input.offset(3);
        assert_eq!(input.remaining(), "def");
        assert_eq!(input.offset, 3);
    }

    #[test]
    fn consume_empty() {
        let input = Input::new("");
        let (input, consumed) = input.consume(|_| true);
        assert_eq!("", consumed);
        assert_eq!("", input.remaining());
    }

    #[test]
    fn consume_many() {
        let input = Input::new("aaa");
        let (input, consumed) = input.consume(|c| *c == 'a');
        assert_eq!("aaa", consumed);
        assert_eq!("", input.remaining());
    }

    #[test]
    fn consume_one() {
        let input = Input::new("abc");
        let (input, consumed) = input.consume(|c| *c == 'a');
        assert_eq!("a", consumed);
        assert_eq!("bc", input.remaining());
    }

    #[test]
    fn consume_whitespace() {
        let input = Input::new("   abc   ");
        let input = input.consume_ws();
        assert_eq!("abc   ", input.remaining());
    }
}

impl<'a> Input<'a> {
    /// Consume a single token from the input, returning (the remainder input, the token).
    /// A token is one of:
    /// - . or , or + or -
    /// - a contiguous sequence of alpha characters
    /// - a contiguous sequence of numeric characters
    /// All whitespace is ignored.
    pub fn next_token(self) -> ParseResult<'a, (TokenKind, &'a str)> {
        let symbols = ['.', ',', '+', '-'];
        let source = self.remaining();
        // Decide what to do based on the first input character
        match source.chars().next() {
            // Symbol tokens
            Some(c) if symbols.contains(&c) => {
                let token = &source[..=0];
                let input = self.offset(1);
                Ok((input.consume_ws(), (TokenKind::Symbol, token)))
            },
            // Alpha sequences
            Some(c) if c.is_alphabetic() => {
                let lower = 'a'..='z';
                let upper = 'A'..='Z';
                let (input, token) = self.consume(|c| lower.contains(c) || upper.contains(c));
                Ok((input.consume_ws(), (TokenKind::Alpha, token)))
            },
            // Numeric sequences
            Some(c) if c.is_numeric() => {
                let numeric = '0'..='9';
                let (input, token) = self.consume(|c| numeric.contains(c));
                Ok((input.consume_ws(), (TokenKind::Numeric, token)))
            },
            // Error cases
            Some(c) => Err(ParseErr::unexpected_input(self.pos(), c)),
            None    => Err(ParseErr::end_of_input(self.pos())),
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn consume_symbol_characters() {
        let input = Input::new(",.");
        let (input, a) = input.next_token().unwrap();
        assert_eq!(a, (TokenKind::Symbol, ","));
        let (input, a) = input.next_token().unwrap();
        assert_eq!(a, (TokenKind::Symbol, "."));
        assert_eq!(input.remaining(), "");
    }

    #[test]
    fn consume_numeric() {
        let input = Input::new("01234");
        let (input, a) = input.next_token().unwrap();
        assert_eq!(a, (TokenKind::Numeric, "01234"));
        assert_eq!(input.remaining(), "");
    }

    #[test]
    fn consume_alpha() {
        let input = Input::new("abcdef");
        let (input, a) = input.next_token().unwrap();
        assert_eq!(a, (TokenKind::Alpha, "abcdef"));
        assert_eq!(input.remaining(), "");
    }

    #[test]
    fn consume_ignores_whitespace() {
        let input = Input::new("abcdef 1 2aa, 3333..");
        let (input, a) = input.next_token().unwrap();
        assert_eq!(a, (TokenKind::Alpha, "abcdef"));
        let (input, a) = input.next_token().unwrap();
        assert_eq!(a, (TokenKind::Numeric, "1"));
        let (input, a) = input.next_token().unwrap();
        assert_eq!(a, (TokenKind::Numeric, "2"));
        let (input, a) = input.next_token().unwrap();
        assert_eq!(a, (TokenKind::Alpha, "aa"));
        let (input, a) = input.next_token().unwrap();
        assert_eq!(a, (TokenKind::Symbol, ","));
        let (input, a) = input.next_token().unwrap();
        assert_eq!(a, (TokenKind::Numeric, "3333"));
        let (input, a) = input.next_token().unwrap();
        assert_eq!(a, (TokenKind::Symbol, "."));
        let (input, a) = input.next_token().unwrap();
        assert_eq!(a, (TokenKind::Symbol, "."));
        assert_eq!(input.remaining(), "");
    }
}

impl<'a> Input<'a> {
    pub fn parse_i32(self) -> ParseResult<'a, i32> {
        let (next, token) = self.next_token()?;
        let num = match token {
            (TokenKind::Numeric, num) => {
                num.parse::<i32>()
                    .map_err(|_| ParseErr::expected_number(self.pos()))
            },
            _ => Err(ParseErr::expected_number(self.pos())),
        }?;
        Ok((next, num))
    }

    pub fn parse_alpha(self) -> ParseResult<'a, &'a str> {
        let (next, token) = self.next_token()?;
        match token {
            (TokenKind::Alpha, alpha) => Ok((next, alpha)),
            _ => Err(ParseErr::expected_alpha(self.pos())),
        }
    }

    pub fn parse_token(self, token: &str) -> ParseResult<'a, &'a str> {
        let (next, (_, actual)) = self.next_token()?;
        if actual != token {
            return Err(ParseErr::expected_token(self.pos(), token));
        }
        Ok((next, actual))
    }

    pub fn parse_end(self) -> ParseResult<'a, ()> {
        match self.next_token() {
            Err(ParseErr::EndOfInput(_)) => Ok((self, ())),
            _ => Err(ParseErr::expected_end_of_input(self.pos())),
        }
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn parse_i32_success() {
        let input = Input::new("123");
        let (input, a) = input.parse_i32().unwrap();
        assert_eq!(a, 123);
        assert_eq!(input.remaining(), "");
    }

    #[test]
    fn parse_i32_fail() {
        let input = Input::new("xxx");
        let err = input.parse_i32().unwrap_err();
        assert_eq!(err, ParseErr::expected_number(InputPos(0)));
    }

    #[test]
    fn parse_alpha_success() {
        let input = Input::new("xxx");
        let (input, a) = input.parse_alpha().unwrap();
        assert_eq!(a, "xxx");
        assert_eq!(input.remaining(), "");
    }

    #[test]
    fn parse_alpha_fail() {
        let input = Input::new("123");
        let err = input.parse_alpha().unwrap_err();
        assert_eq!(err, ParseErr::expected_alpha(InputPos(0)));
    }

    #[test]
    fn parse_token_success() {
        let input = Input::new(" xxx ");
        let (input, a) = input.parse_token("xxx").unwrap();
        assert_eq!(a, "xxx");
        assert_eq!(input.remaining(), "");

        let input = Input::new(" 123 ");
        let (input, a) = input.parse_token("123").unwrap();
        assert_eq!(a, "123");
        assert_eq!(input.remaining(), "");

        let input = Input::new(" , ");
        let (input, a) = input.parse_token(",").unwrap();
        assert_eq!(a, ",");
        assert_eq!(input.remaining(), "");
    }

    #[test]
    fn parse_token_fail() {
        let input = Input::new(" xxx ");
        let err = input.parse_token("yyy").unwrap_err();
        assert_eq!(err, ParseErr::expected_token(InputPos(1), "yyy"));
    }

    #[test]
    fn parse_end_success() {
        let input = Input::new("a");
        let (input, _) = input.parse_alpha().unwrap();
        let (input, a) = input.parse_end().unwrap();
        assert_eq!(a, ());
        assert_eq!(input.remaining(), "");
    }

    #[test]
    fn parse_end_fail() {
        let input = Input::new("a");
        let err = input.parse_end().unwrap_err();
        assert_eq!(err, ParseErr::expected_end_of_input(InputPos(0)));
    }

    #[test]
    fn parser_chaining() { 

        fn parse_2(input: Input) -> ParseResult<(&str, i32)> {
            let (input, a) = input.parse_alpha()?;
            let (input, b) = input.parse_i32()?;
            Ok((input, (a, b)))
        }

        fn parse_exact(input: Input) -> ParseResult<()> {
            let (input, _) = input.parse_token("aaa")?;
            let (input, _) = input.parse_token("bbb")?;
            let (input, _) = input.parse_token("ccc")?;
            Ok((input, ()))
        }

        let input = Input::new("aaa bbb ccc hello 123");
        let (input, a) = parse_exact(input).unwrap();
        assert_eq!(a, ());
        let (input, a) = parse_2(input).unwrap();
        assert_eq!(a, ("hello", 123));
        assert_eq!(input.remaining(), "");
    }

    #[test]
    fn parser_loops() {

        fn parse_foo(input: Input) -> ParseResult<()> {
            let (input, _) = input.parse_token("aaa")?;
            let (input, _) = input.parse_token("bbb")?;
            Ok((input, ()))
        }

        let input = Input::new("aaa bbb aaa bbb aaa bbb ccc");
        let mut input = input;
        while let Ok((input_, ())) = parse_foo(input) {
            input = input_;
        }
        assert_eq!(input.remaining(), "ccc");
    }

    #[test]
    fn parser_loops_with_explicit_termination() {

        fn try_parse_next(input: Input) -> ParseResult<Option<&'static str>> {
            // check for terminator token
            if let Ok((input, _)) = input.parse_token("ccc") {
                return Ok((input, None));
            }
            let (input, _) = input.parse_token("aaa")?;
            let (input, _) = input.parse_token("bbb")?;
            Ok((input, Some("foo bar")))
        }

        let input = Input::new("aaa bbb aaa bbb aaa bbb ccc");
        let mut input = input;
        while let Ok((input_, result)) = try_parse_next(input) {
            input = input_;
            if let Some(result) = result {
                assert_eq!(result, "foo bar");
            }
        }
        assert_eq!(input.remaining(), "");
    }

    #[test]
    fn parser_combination_with_or_try() {

        fn parse_num(input: Input) -> ParseResult<i32> {
            input.parse_token("zero").val(0)
                .or_try(|| input.parse_i32())
        }

        let input = Input::new("123");
        let (input, a) = parse_num(input).unwrap();
        assert_eq!(a, 123);
        assert_eq!(input.remaining(), "");

        let input = Input::new("zero");
        let (input, a) = parse_num(input).unwrap();
        assert_eq!(a, 0);
        assert_eq!(input.remaining(), "");
    }
}