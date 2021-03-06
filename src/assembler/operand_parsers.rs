use crate::assembler::register_parsers::register;
use crate::assembler::Token;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, multispace0},
    sequence::{delimited, preceded},
    IResult,
};

pub fn integer_operand(input: &str) -> IResult<&str, Token> {
    let (leftover, operand) =
        delimited(multispace0, preceded(tag("#"), digit1), multispace0)(input)?;
    Ok((
        leftover,
        Token::IntegerOperand {
            value: operand.parse::<i32>().unwrap(),
        },
    ))
}

pub fn irstring(input: &str) -> IResult<&str, Token> {
    let (leftover, content) = delimited(tag("'"), alphanumeric1, tag("'"))(input)?;
    Ok((
        leftover,
        Token::IrString {
            name: content.to_string(),
        },
    ))
}

pub fn operand(input: &str) -> IResult<&str, Token> {
    alt((integer_operand, register, irstring))(input)
}

mod tests {
    #![allow(unused_imports)]
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
    #[test]
    fn test_parse_irstring() {
        // Test a valid integer operand
        let result = irstring("'Ciaone'");
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::IrString { name: "Ciaone".to_string() });
    }
}
