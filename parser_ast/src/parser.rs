use combine::primitives::{ Stream, State };
use combine::{ Parser, ParserExt, ParseResult };
use combine::{ spaces, many1, digit, lower, string, choice, try, parser };

use ast;

fn p_label<I>(input: State<I>) -> ParseResult<ast::Label, I>
    where I: Stream<Item=char>
{
    let label = many1(lower()).message("Label");
    
    spaces().with(label).parse_state(input)
}

fn p_signal<I>(input: State<I>) -> ParseResult<ast::Signal, I>
    where I: Stream<Item=char>
{
    let signal =
        many1(digit())
            .map(|val: String| val.parse::<ast::Signal>().unwrap())
            .message("Raw Signal");
    
    spaces().with(signal).parse_state(input)
}

fn p_source<I>(input: State<I>) -> ParseResult<ast::Source, I>
    where I: Stream<Item=char>
{
    let label = parser(p_label).map(|s| ast::Source::Wire(s));
    let source = parser(p_signal).map(|s| ast::Source::Const(s));
    
    try(label).or(source).parse_state(input)
}

fn p_gate1<I>(input: State<I>) -> ParseResult<ast::Gate1, I>
    where I: Stream<Item=char>
{
    let choice =
        string("NOT").map(|_| ast::Gate1::NOT);
        
    spaces().with(choice).parse_state(input)
}

fn p_gate2<I>(input: State<I>) -> ParseResult<ast::Gate2, I>
    where I: Stream<Item=char>
{
    let choice =
        choice([
            string("AND"),
            string("OR"),
            string("LSHIFT"),
            string("RSHIFT"),
        ])
        .map(|label| match label {
            "AND"    => ast::Gate2::AND,
            "OR"     => ast::Gate2::OR,
            "LSHIFT" => ast::Gate2::LSHIFT,
            "RSHIFT" => ast::Gate2::RSHIFT,
            _        => panic!("p_gate2")
        });
        
    spaces().with(choice).parse_state(input)
}

fn p_expr<I>(input: State<I>) -> ParseResult<ast::Expr, I>
    where I: Stream<Item=char>
{
    let gate2  =
        parser(p_source).and(parser(p_gate2)).and(parser(p_source))
            .map(|((source1, gate), source2)| ast::Expr::Gate2(gate, source1, source2))
            .message("Two-input Gate");
            
    let gate1 =
        parser(p_gate1).and(parser(p_source))
            .map(|(gate, source)| ast::Expr::Gate1(gate, source))
            .message("One-input Gate");
            
    let source =
        parser(p_source)
            .map(|source| ast::Expr::Input(source));
    
    let match_one = try(gate2).or(try(gate1)).or(source);
    
    spaces().with(match_one).parse_state(input)
}

fn p_inst<I>(input: State<I>) -> ParseResult<ast::Instruction, I>
    where I: Stream<Item=char>
{
    let arrow = spaces().with(string("->"));
    let target = spaces().with(parser(p_label));
    
    let mut expr =
        parser(p_expr).skip(arrow).and(target)
            .map(|(e, t)| ast::Instruction { expr: e, target: t });
            
    expr.parse_state(input)
}

pub fn parse_instruction(s: &str) -> Result<ast::Instruction, String> {
    match parser(p_inst).parse(s) {
        Ok((result, _)) => Ok(result),
        Err(err)        => Err(format!("{}", err))
    }
}
