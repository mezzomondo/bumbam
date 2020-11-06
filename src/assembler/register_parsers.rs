use crate::assembler::Token;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    sequence::{delimited, preceded},
    IResult,
};

pub fn register(input: &str) -> IResult<&str, Token> {
    let (leftover, register) =
        delimited(multispace0, preceded(tag("$"), digit1), multispace0)(input)?;
    Ok((
        leftover,
        Token::Register {
            reg_num: register.parse::<u8>().unwrap(),
        },
    ))
}

mod tests {
    #![allow(unused_imports)]
    use super::*;
    #[test]
    fn test_parse_register() {
        let result = register("$0");
        assert_eq!(result.is_ok(), true);
        let result = register("0");
        assert_eq!(result.is_ok(), false);
        let result = register("$a");
        assert_eq!(result.is_ok(), false);
    }
}
