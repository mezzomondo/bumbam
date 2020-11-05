use crate::assembler::Token;
use std::num::ParseIntError;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    sequence::{delimited, preceded},
    IResult,
};

fn from_str_operand(reg_num: &str) -> Result<Token, ParseIntError> {
    Ok(Token::IntegerOperand {
        value: reg_num.parse::<i32>().unwrap(),
    })
}

pub fn integer_operand(input: &str) -> IResult<&str, Token> {
    map_res(
        delimited(multispace0, preceded(tag("#"), digit1), multispace0),
        |out: &str| from_str_operand(out),
    )(input)
}

mod tests {
    use super::*;
    #[test]
    fn test_parse_integer_operand() {
        // Test a valid integer operand
        let result = integer_operand("#10");
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::IntegerOperand { value: 10 });

        // Test an invalid one (missing the #)
        let result = integer_operand("10");
        assert_eq!(result.is_ok(), false);
    }
}
