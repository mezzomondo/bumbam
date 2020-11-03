use crate::assembler::Token;
use std::num::ParseIntError;

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, sequence::preceded,
    IResult,
};

fn from_str_register(reg_num: &str) -> Result<Token, ParseIntError> {
    Ok(Token::Register {
        reg_num: reg_num.parse::<u8>().unwrap(),
    })
}

pub fn register(input: &str) -> IResult<&str, Token> {
    map_res(preceded(tag("$"), digit1), |out: &str| {
        from_str_register(out)
    })(input)
}

mod tests {
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
