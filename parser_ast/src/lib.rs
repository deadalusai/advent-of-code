extern crate combine;

pub mod ast;
pub mod parser;

use combine::{ parser, Parser };
use parser::{ p_inst };

pub fn parse_instruction(s: &str) -> Result<ast::Instruction, String> {
    match parser(p_inst).parse(s) {
        Ok((result, _)) => Ok(result),
        Err(err)        => Err(format!("{}", err))
    }
}