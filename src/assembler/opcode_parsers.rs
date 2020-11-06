use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::{
    character::complete::{alpha1, multispace0},
    sequence::delimited,
    IResult,
};

pub fn opcode(input: &str) -> IResult<&str, Token> {
    let (leftover, input) = delimited(multispace0, alpha1, multispace0)(input)?;
    Ok((
        leftover,
        Token::Op {
            code: Opcode::from(input),
        },
    ))
}

mod tests {
    #![allow(unused_imports)]
    use super::*;
    #[test]
    fn test_opcode_load() {
        // First tests that the opcode is detected and parsed correctly
        let result = opcode("load");
        assert_eq!(result.is_ok(), true);
        let (leftover, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(leftover, "");

        // Tests that an invalid opcode isn't recognized
        let result = opcode("aold");
        assert_eq!(result.is_ok(), true);
        let (leftover, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
        assert_eq!(leftover, "");
    }
}
