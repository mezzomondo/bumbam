use crate::assembler::Token;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
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
