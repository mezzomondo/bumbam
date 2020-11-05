use super::opcode_parsers::*;
use super::operand_parsers::integer_operand;
use super::register_parsers::register;
use super::Token;
use nom::{
    error::{ErrorKind, ParseError},
    sequence::tuple,
    Err, IResult,
};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    label: Option<String>,
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

pub fn instruction_one(input: &str) -> IResult<&str, AssemblerInstruction> {
    let pattern = tuple((opcode_load, register, integer_operand))(input);
    match pattern {
        Ok(("", (o, r, i))) => Ok((
            "",
            AssemblerInstruction {
                label: None,
                opcode: o,
                operand1: Some(r),
                operand2: Some(i),
                operand3: None,
            },
        )),
        _ => Err(Err::Error(ParseError::from_error_kind(
            input,
            ErrorKind::MultiSpace,
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Token;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_one("load $0 #100\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    label: None,
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            ))
        );
    }
}
