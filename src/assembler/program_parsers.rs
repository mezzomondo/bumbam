use super::instruction_parsers::{instruction_one, AssemblerInstruction};
use nom::{multi::many1, IResult};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

pub fn program(input: &str) -> IResult<&str, Program> {
    let (_, i) = many1(instruction_one)(input)?;
    Ok(("", Program { instructions: i }))
}

#[test]
fn test_parse_program() {
    let result = program("load $0 #100\nload $1 #100\n");
    assert_eq!(result.is_ok(), true);
    let (leftover, p) = result.unwrap();
    assert_eq!(leftover, "");
    assert_eq!(2, p.instructions.len());
    // TODO: Figure out an ergonomic way to test the AssemblerInstruction returned
}
