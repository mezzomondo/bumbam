use crate::assembler::Token;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric0, multispace0},
    sequence::{preceded, terminated},
    IResult,
};

pub fn label_declaration(input: &str) -> IResult<&str, Token> {
    let (leftover, name) = terminated(terminated(alphanumeric0, tag(":")), multispace0)(input)?;
    Ok((
        leftover,
        Token::LabelDeclaration {
            name: name.to_string(),
        },
    ))
}

pub fn label_usage(input: &str) -> IResult<&str, Token> {
    let (leftover, name) = terminated(preceded(tag("@"), alphanumeric0), multispace0)(input)?;
    Ok((
        leftover,
        Token::LabelUsage {
            name: name.to_string(),
        },
    ))
}

#[test]
fn test_parse_label_declaration() {
    let result = label_declaration("test:");
    assert_eq!(result.is_ok(), true);
    let (_, token) = result.unwrap();
    assert_eq!(
        token,
        Token::LabelDeclaration {
            name: "test".to_string()
        }
    );
    let result = label_declaration("test");
    assert_eq!(result.is_ok(), false);
}

#[test]
fn test_parse_label_usage() {
    let result = label_usage("@test");
    assert_eq!(result.is_ok(), true);
    let (_, token) = result.unwrap();
    assert_eq!(
        token,
        Token::LabelUsage {
            name: "test".to_string()
        }
    );
    let result = label_usage("test");
    assert_eq!(result.is_ok(), false);
}
