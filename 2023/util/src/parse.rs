
use std::fmt;

use crate::error::AppErr;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Symbol,
    Alpha,
    Numeric,
}

pub type ParseResult<'a, T> = Result<(Input<'a>, T), ParseErr<'a>>;

pub trait ParseResultEx<'a, T> {
    /// Applies the second parser if `self` represents a failed Parse.
    fn or_try(self, op: impl FnOnce() -> ParseResult<'a, T>) -> ParseResult<'a, T>;

    /// Maps the "value" component of a ParseResult. 
    fn map_val<V>(self, op: impl FnOnce(T) -> V) -> ParseResult<'a, V>;

    /// Replaces the "value" component of a ParseResult.
    /// Useful for assigning a value to a token.
    fn val<V>(self, val: V) -> ParseResult<'a, V>;

    /// Strips Input information from the ParseResult, converting it into an ordinary Result.
    fn complete(self) -> Result<T, AppErr>;
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

    fn complete(self) -> Result<T, AppErr> {
        self.map(|(_, v)| v)
            .map_err(|e| e.into())
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct InputSnapshot<'a> {
    source: &'a str,
    offset: usize,
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

    pub fn snapshot(self) -> InputSnapshot<'a> {
        InputSnapshot { source: self.source, offset: self.offset }
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
    /// - a contiguous sequence of alpha characters
    /// - a contiguous sequence of numeric characters
    /// - a single ascii punctuation character
    /// All whitespace is ignored.
    pub fn next_token(self) -> ParseResult<'a, (TokenKind, &'a str)> {
        let source = self.remaining();
        // Decide what to do based on the first input character
        match source.chars().next() {
            // Alpha sequences
            Some(c) if c.is_alphabetic() => {
                let (input, token) = self.consume(|c| c.is_alphabetic());
                Ok((input.consume_ws(), (TokenKind::Alpha, token)))
            },
            // Numeric sequences
            Some(c) if c.is_numeric() => {
                let (input, token) = self.consume(|c| c.is_numeric());
                Ok((input.consume_ws(), (TokenKind::Numeric, token)))
            },
            // Symbol tokens
            Some(c) if c.is_ascii_punctuation() => {
                let token = &source[..=0];
                let input = self.offset(1);
                Ok((input.consume_ws(), (TokenKind::Symbol, token)))
            },
            // Error cases
            Some(c) => Err(ParseErr::unexpected_input(self.snapshot(), c)),
            None    => Err(ParseErr::end_of_input(self.snapshot())),
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
                    .map_err(|_| ParseErr::expected_number(self.snapshot()))
            },
            _ => Err(ParseErr::expected_number(self.snapshot())),
        }?;
        Ok((next, num))
    }

    pub fn parse_alpha(self) -> ParseResult<'a, &'a str> {
        let (next, token) = self.next_token()?;
        match token {
            (TokenKind::Alpha, alpha) => Ok((next, alpha)),
            _ => Err(ParseErr::expected_alpha(self.snapshot())),
        }
    }

    pub fn parse_token(self, token: &str) -> ParseResult<'a, &'a str> {
        let (next, (_, actual)) = self.next_token()?;
        if actual != token {
            return Err(ParseErr::expected_token(self.snapshot(), token));
        }
        Ok((next, actual))
    }

    pub fn parse_end(self) -> ParseResult<'a, ()> {
        match self.next_token() {
            Err(ParseErr::EndOfInput(_)) => Ok((self, ())),
            _ => Err(ParseErr::expected_end_of_input(self.snapshot())),
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
        assert_eq!(err, ParseErr::expected_number(InputSnapshot { source: "xxx", offset: 0 }));
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
        assert_eq!(err, ParseErr::expected_alpha(InputSnapshot { source: "123", offset: 0 }));
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
        assert_eq!(err, ParseErr::expected_token(InputSnapshot { source: " xxx ", offset: 1 }, "yyy"));
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
        assert_eq!(err, ParseErr::expected_end_of_input(InputSnapshot { source: "a", offset: 0 }));
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

impl<'a> Input<'a> {
    pub fn parse_separated<I, Fi, S, Fs>(self, item: Fi, separator: Fs) -> ParseResult<'a, Vec<I>>
    where
        Fi: Fn(Input<'a>) -> ParseResult<'a, I>,
        Fs: Fn(Input<'a>) -> ParseResult<'a, S>
    {
        let mut input = self.clone();
        let mut results = Vec::new();

        loop {
            // Parse item
            let (next, val) = item(input)?;
            results.push(val);

            // Check for separator
            if let Ok((next, _)) = separator(next) {
                input = next;
                continue;
            }

            // Reached end of input
            input = next;
            break;
        }

        Ok((input, results))
    }

    pub fn parse_delimited<Ds, Fds, I, Fi, De, Fde>(self, start: Fds, item: Fi, end: Fde) -> ParseResult<'a, I>
    where
        Fds: Fn(Input<'a>) -> ParseResult<'a, Ds>,
        Fi: Fn(Input<'a>) -> ParseResult<'a, I>,
        Fde: Fn(Input<'a>) -> ParseResult<'a, De>
    {
        let (input, _) = start(self)?;
        let (input, item) = item(input)?;
        let (input, _) = end(input)?;
        Ok((input, item))
    }
}

#[cfg(test)]
mod combinator_tests {
    use super::*;

    #[test]
    fn parse_separated_1() {
        let input = Input::new("hello, my, name, is, alfred");
        let (input, results) = input.parse_separated(|next| next.parse_alpha(), |next| next.parse_token(",")).unwrap();
        assert_eq!("", input.remaining());
        assert_eq!(vec!["hello", "my", "name", "is", "alfred"], results);
    }

    #[test]
    fn parse_separated_2() {
        let input = Input::new("hello, my, name is john");
        let (input, results) = input.parse_separated(|next| next.parse_alpha(), |next| next.parse_token(",")).unwrap();
        assert_eq!("is john", input.remaining());
        assert_eq!(vec!["hello", "my", "name"], results);
    }

    #[test]
    fn parse_separated_error() {
        let input = Input::new("hello, 1, name");
        let err = input.parse_separated(|next| next.parse_alpha(), |next| next.parse_token(",")).unwrap_err();
        assert_eq!(format!("{:?}", err), "expected alpha at offset 7");
    }

    #[test]
    fn parse_delimited_1() {
        let input = Input::new(r#" [ hello ] "#);
        let (input, result) = input.parse_delimited(
            |next| next.parse_token("["),
            |next| next.parse_alpha(),
            |next| next.parse_token("]")
        ).unwrap();
        assert_eq!("", input.remaining());
        assert_eq!("hello", result);
    }

    #[test]
    fn parse_delimited_error() {
        let input = Input::new(r#" [ 1 ] "#);
        let err = input.parse_delimited(
            |next| next.parse_token("["),
            |next| next.parse_alpha(),
            |next| next.parse_token("]")
        ).unwrap_err();
        assert_eq!(format!("{:?}", err), "expected alpha at offset 3");
    }
}

#[derive(PartialEq, Eq)]
pub enum ParseErr<'a> {
    EndOfInput       (InputSnapshot<'a>),
    UnexpectedInput  (InputSnapshot<'a>, char),
    ExpectedSingle   (InputSnapshot<'a>, String),
    ExpectedMultiple (InputSnapshot<'a>, Vec<String>),
}

impl<'a> ParseErr<'a> {
    fn snapshot(&self) -> &InputSnapshot<'a> {
        match self { 
            ParseErr::EndOfInput(snap) => &snap,
            ParseErr::UnexpectedInput(snap, _) => &snap,
            ParseErr::ExpectedSingle(snap, _) => &snap,
            ParseErr::ExpectedMultiple(snap, _) => &snap,
        }
    }
}

impl<'a> ParseErr<'a> {
    pub fn end_of_input(snapshot: InputSnapshot<'a>) -> ParseErr<'a> {
        ParseErr::EndOfInput(snapshot)
    }
    
    pub fn unexpected_input(snapshot: InputSnapshot<'a>, c: char) -> ParseErr<'a> {
        ParseErr::UnexpectedInput(snapshot, c)
    }
    
    pub fn expected_alpha(snapshot: InputSnapshot<'a>) -> ParseErr<'a> {
        ParseErr::ExpectedSingle(snapshot, format!("alpha"))
    }

    pub fn expected_number(snapshot: InputSnapshot<'a>) -> ParseErr<'a> {
        ParseErr::ExpectedSingle(snapshot, format!("number"))
    }

    pub fn expected_token(snapshot: InputSnapshot<'a>, token: &str) -> ParseErr<'a> {
        ParseErr::ExpectedSingle(snapshot, format!("`{}`", token))
    }

    pub fn expected_end_of_input(snapshot: InputSnapshot<'a>) -> ParseErr<'a> {
        ParseErr::ExpectedSingle(snapshot, format!("end of input"))
    }

    pub fn combine(a: ParseErr<'a>, b: ParseErr<'a>) -> ParseErr<'a> {
        let snapshot = a.snapshot().clone();
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
        ParseErr::ExpectedMultiple(snapshot, errors)
    }
}

impl<'a> fmt::Debug for ParseErr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErr::UnexpectedInput(snap, c) => {
                write!(f, "unexpected character `{}` at offset {}", c, snap.offset)?;
            },
            ParseErr::EndOfInput(_) => {
                write!(f, "unexpected end of input")?;
            }
            ParseErr::ExpectedSingle(snap, exp) => {
                write!(f, "expected {} at offset {}", exp, snap.offset)?
            },
            ParseErr::ExpectedMultiple(snap, expected) => {
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
                write!(f, " at offset {}", snap.offset)?;
            },
        };
        Ok(())
    }
}

impl<'a> fmt::Display for ParseErr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // "Display" version includes input text
        writeln!(f, "{:?}", self)?;
        writeln!(f, "{}", input_snippet(&self.snapshot(), 20))?;
        write!(f, "{}", input_caret(&self.snapshot(), 20))?;
        Ok(())
    }
}

fn input_snippet(snap: &InputSnapshot, context: usize) -> String {
    let snip_start = if snap.offset > context  { snap.offset - context } else { 0 };
    let snip_end = (snap.offset + context).min(snap.source.len() - 1);
    format!(
        "{}{}{}",
        if snip_start == 0 { "" } else { "…" },
        &snap.source[snip_start..=snip_end],
        if snip_end == snap.source.len() - 1 { "" } else { "…" }
    )
}

fn input_caret(snap: &InputSnapshot, context: usize) -> String {
    let leading = if snap.offset > context { context + 1 } else { snap.offset };
    let mut s = " ".repeat(leading);
    s.push_str("^--- here");
    s
}

#[cfg(test)]
mod err_fmt_tests {
    use super::*;

    #[test]
    fn start_of_input() {
        let context = 5;
        let snap = InputSnapshot {
            source: "Hello world, Goodbye moon.",
            offset: 4,
        };
        assert_eq!("Hello worl…", input_snippet(&snap, context));
        assert_eq!("    ^--- here", input_caret(&snap, context));
    }

    #[test]
    fn middle_of_input() {
        let context = 5;
        let snap = InputSnapshot {
            source: "Hello world, Goodbye moon.",
            offset: 10,
        };
        assert_eq!("… world, Goo…", input_snippet(&snap, context));
        assert_eq!("      ^--- here", input_caret(&snap, context));
    }

    #[test]
    fn end_of_input() {
        let context = 5;
        let snap = InputSnapshot {
            source: "Hello world, Goodbye moon.",
            offset: 21,
        };
        assert_eq!("…dbye moon.", input_snippet(&snap, context));
        assert_eq!("      ^--- here", input_caret(&snap, context));
    }

    fn example_parser<'a>(input: Input<'a>) -> ParseResult<'a, ()> {
        let (input, _) = input.parse_token("Bar")?;
        let (input, _) = input.parse_i32()?;
        let (input, _) = input.parse_end()?;
        Ok((input, ()))
    }

    #[test]
    fn parser_error_debug_1() {
        let input = Input::new("Bar Baz");
        let err = example_parser(input).unwrap_err();
        let actual = format!("{:?}", err);
        let expected = r"expected number at offset 4";
        assert_eq!(expected, actual);
    }

    #[test]
    fn parser_error_display_1() {
        let input = Input::new("Foo 12");
        let err = example_parser(input).unwrap_err();
        let actual = format!("{}", err);
        let expected = r"expected `Bar` at offset 0
Foo 12
^--- here";
        assert_eq!(expected, actual);
    }

    #[test]
    fn parser_error_display_2() {
        let input = Input::new("Bar Baz");
        let err = example_parser(input).unwrap_err();
        let actual = format!("{}", err);
        let expected = r"expected number at offset 4
Bar Baz
    ^--- here";
        assert_eq!(expected, actual);
    }
}