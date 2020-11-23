use super::instruction_parsers::{instruction_combined, AssemblerInstruction};
use nom::{multi::many1, IResult};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }
}

pub fn program(input: &str) -> IResult<&str, Program> {
    let (leftover, i) = many1(instruction_combined)(input)?;
    Ok((leftover, Program { instructions: i }))
}

mod tests {
    #![allow(unused_imports)]
    use super::*;
    use crate::assembler::Token::*;
    use crate::instruction::Opcode::*;
    #[test]
    fn test_parse_program() {
        let result = program("load $0 #100\nload $1 #200");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(p.instructions.len(), 2);
        assert_eq!(
            p.instructions,
            [
                AssemblerInstruction {
                    opcode: Some(Op { code: LOAD }),
                    label: None,
                    directive: None,
                    operand1: Some(Register { reg_num: 0 }),
                    operand2: Some(IntegerOperand { value: 100 }),
                    operand3: None
                },
                AssemblerInstruction {
                    opcode: Some(Op { code: LOAD }),
                    label: None,
                    directive: None,
                    operand1: Some(Register { reg_num: 1 }),
                    operand2: Some(IntegerOperand { value: 200 }),
                    operand3: None
                }
            ]
        );
    }
    #[test]
    fn test_program_to_bytes() {
        let result = program("load $0 #100\nload $1 #200\nadd $0 $1 $2\nhlt");
        assert_eq!(result.is_ok(), true);
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 16);
        assert_eq!(
            bytecode,
            [1, 0, 0, 100, 1, 1, 0, 200, 2, 0, 1, 2, 0, 0, 0, 0]
        );
    }
}
