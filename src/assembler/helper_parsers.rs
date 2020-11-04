use nom::{
    character::complete::alphanumeric0, character::complete::multispace0, sequence::delimited,
    IResult,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws(inner: &str) -> IResult<&str, &str> {
    delimited(multispace0, alphanumeric0, multispace0)(inner)
}

mod tests {
    use super::*;
    #[test]
    fn test_ws() {
        let result = ws(" something");
        assert_eq!(result, Ok(("", "something")));
        let result = ws("something ");
        assert_eq!(result, Ok(("", "something")));
        let result = ws(" something ");
        assert_eq!(result, Ok(("", "something")));
        let result = ws("something");
        assert_eq!(result, Ok(("", "something")));
        let result = ws(" ");
        assert_eq!(result, Ok(("", "")));
        let result = ws("  ");
        assert_eq!(result, Ok(("", "")));
        let result = ws("   ");
        assert_eq!(result, Ok(("", "")));
    }
}
