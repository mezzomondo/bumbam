use super::opcode_parsers::*;
use super::operand_parsers::integer_operand;
use super::register_parsers::register;
use super::Token;
use nom::{
    branch::alt,
    character::complete::multispace0,
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub label: Option<String>,
    pub opcode: Token,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op { code } => match code {
                _ => results.push(code as u8),
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        }

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results)
            }
        }

        return results;
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }
}

// _zero etc. is the arity of the instuction parsed
fn instruction_zero(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (leftover, o) = terminated(opcode, multispace0)(input)?;
    Ok((
        leftover,
        AssemblerInstruction {
            label: None,
            opcode: o,
            operand1: None,
            operand2: None,
            operand3: None,
        },
    ))
}

fn instruction_two(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (leftover, (o, r, i)) = tuple((opcode, register, integer_operand))(input)?;
    Ok((
        leftover,
        AssemblerInstruction {
            label: None,
            opcode: o,
            operand1: Some(r),
            operand2: Some(i),
            operand3: None,
        },
    ))
}

fn instruction_three(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (leftover, (o, r1, r2, r3)) = tuple((opcode, register, register, register))(input)?;
    Ok((
        leftover,
        AssemblerInstruction {
            label: None,
            opcode: o,
            operand1: Some(r1),
            operand2: Some(r2),
            operand3: Some(r3),
        },
    ))
}

pub fn instruction(input: &str) -> IResult<&str, AssemblerInstruction> {
    alt((instruction_three, instruction_two, instruction_zero))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Token;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_zero() {
        let result = instruction_zero("HLT\n");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(None, p.label);
        assert_eq!(Token::Op { code: Opcode::HLT }, p.opcode);
        assert_eq!(None, p.operand1);
        assert_eq!(None, p.operand2);
        assert_eq!(None, p.operand3);
    }
    #[test]
    fn test_parse_instruction_two() {
        let result = instruction_two("load $0 #100\n");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(None, p.label);
        assert_eq!(Token::Op { code: Opcode::LOAD }, p.opcode);
        assert_eq!(Some(Token::Register { reg_num: 0 }), p.operand1);
        assert_eq!(Some(Token::IntegerOperand { value: 100 }), p.operand2);
        assert_eq!(None, p.operand3);
    }
    #[test]
    fn test_parse_instruction_three() {
        let result = instruction_three("add $0 $1 $2\n");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(None, p.label);
        assert_eq!(Token::Op { code: Opcode::ADD }, p.opcode);
        assert_eq!(Some(Token::Register { reg_num: 0 }), p.operand1);
        assert_eq!(Some(Token::Register { reg_num: 1 }), p.operand2);
        assert_eq!(Some(Token::Register { reg_num: 2 }), p.operand3);
    }
}
