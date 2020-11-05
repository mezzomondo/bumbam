use super::opcode_parsers::*;
use super::operand_parsers::integer_operand;
use super::register_parsers::register;
use super::Token;
use nom::{sequence::tuple, IResult};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    label: Option<String>,
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

pub fn instruction_one(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (leftover, (o, r, i)) = tuple((opcode_load, register, integer_operand))(input)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Token;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_one("load $0 #100\n");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(None, p.label);
        assert_eq!(Token::Op { code: Opcode::LOAD }, p.opcode);
        assert_eq!(Some(Token::Register { reg_num: 0 }), p.operand1);
        assert_eq!(Some(Token::IntegerOperand { value: 100 }), p.operand2);
        assert_eq!(None, p.operand3);
    }
}
