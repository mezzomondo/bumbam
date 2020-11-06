use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::{
    bytes::complete::tag_no_case, character::complete::multispace0, sequence::delimited, IResult,
};

pub fn opcode_load(input: &str) -> IResult<&str, Token> {
    let (input, _) = delimited(multispace0, tag_no_case("load"), multispace0)(input)?;
    Ok((input, Token::Op { code: Opcode::LOAD }))
}

mod tests {
    #![allow(unused_imports)]
    use super::*;
    #[test]
    fn test_opcode_load() {
        // First tests that the opcode is detected and parsed correctly
        let result = opcode_load("load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");

        // Tests that an invalid opcode isn't recognized
        let result = opcode_load("aold");
        assert_eq!(result.is_ok(), false);
    }
}
