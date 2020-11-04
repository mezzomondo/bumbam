use nom::{character::complete::multispace0, error::ParseError, sequence::delimited, IResult};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
// pub fn ws(inner: F) -> IResult<&str, &str> {
//     delimited(multispace0, alphanumeric0, multispace0)(inner)
// }

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F, i: &'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)(i)
}

// mod tests {
//     use super::*;
//     #[test]
//     fn test_ws() {
//         let result = ws(" something");
//         assert_eq!(result, Ok(("", "something")));
//         let result = ws("something ");
//         assert_eq!(result, Ok(("", "something")));
//         let result = ws(" something ");
//         assert_eq!(result, Ok(("", "something")));
//         let result = ws("something");
//         assert_eq!(result, Ok(("", "something")));
//         let result = ws(" ");
//         assert_eq!(result, Ok(("", "")));
//         let result = ws("  ");
//         assert_eq!(result, Ok(("", "")));
//         let result = ws("   ");
//         assert_eq!(result, Ok(("", "")));
//     }
// }
