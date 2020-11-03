use crate::assembler::Token;
use nom::character::complete::digit1;
use nom::*;

named!(integer_operand<&str, Token>,
    do_parse!(
        tag!("#") >>
        reg_num: digit1 >>
        (
            Token::IntegerOperand{value: reg_num.parse::<i32>().unwrap()}
        )
    )
);

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
