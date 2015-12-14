use combine::{ spaces, many1, digit, lower, string, choice, try, Parser, ParserExt, ParseError };

use ast::*;

pub type PResult<'a, T> = Result<(T, &'a str), ParseError<&'a str>>;

pub fn parse_instruction(s: &str) -> PResult<Instruction> {
    
    // A wire "label" is a sequence of lowercase letters
    let p_wire_label = || many1(lower()).message("Label");

    // A raw signal is an integer                         
    let p_signal = || many1(digit()).map(|val: String| val.parse::<Signal>().unwrap()).message("Raw Signal");
    
    let p_source = || {
        let signal = p_signal().map(|l| Source::Const(l));
        let wire = p_wire_label().map(|s| Source::Wire(s));
        spaces().with(
            signal.or(wire)
        )
    };
        
    let p_gate_2 = || {
        let choice =
            choice([
                string("AND"),
                string("OR"),
                string("LSHIFT"),
                string("RSHIFT"),
            ])
            .map(|label| match label {
                "AND"    => Gate2::AND,
                "OR"     => Gate2::OR,
                "LSHIFT" => Gate2::LSHIFT,
                "RSHIFT" => Gate2::RSHIFT,
                _        => panic!("p_gate_2")
            });
            
        spaces().with(
            choice
        )
    };
        
    let p_gate_1 = || {
        let choice = string("NOT").map(|_| Gate1::NOT);
        
        spaces().with(
            choice
        )
    };
        

    let p_expr = || {
        let gate2 = p_source().and(p_gate_2()).and(p_source()).map(|((source1, gate), source2)| Expr::Gate2(gate, source1, source2)).message("Two-input Gate");
        let gate1 = p_gate_1().and(p_source())                .map(|(gate, source)|             Expr::Gate1(gate, source))          .message("One-input Gate");
        let input = p_source()                                .map(|source|                     Expr::Input(source));
        
        spaces().with(
            try(gate2).or(try(gate1)).or(input)
        )
    };
    
    let p_inst = || {
        let arrow = spaces().with(string("->"));
        let target = spaces().with(p_wire_label());
        
        p_expr().skip(arrow).and(target)
            .map(|(expr, target)| Instruction { expr: expr, target: target })
    };
    
    p_inst().parse(s)
}