use super::instruction_parsers::AssemblerInstruction;
use super::label_parsers::label_declaration;
use super::operand_parsers::operand;
use super::Token;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric0, multispace0},
    combinator::opt,
    sequence::tuple,
    sequence::{preceded, terminated},
    IResult,
};

fn directive_declaration(input: &str) -> IResult<&str, Token> {
    let (leftover, dir) = terminated(preceded(tag("."), alphanumeric0), multispace0)(input)?;
    Ok((
        leftover,
        Token::Directive {
            name: dir.to_string(),
        },
    ))
}

fn directive_combined(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (leftover, (l, dir, o1, o2, o3)) = tuple((
        opt(label_declaration),
        directive_declaration,
        opt(operand),
        opt(operand),
        opt(operand),
    ))(input)?;
    Ok((
        leftover,
        AssemblerInstruction {
            label: l,
            directive: Some(dir),
            opcode: None,
            operand1: o1,
            operand2: o2,
            operand3: o3,
        },
    ))
}

pub fn directive(input: &str) -> IResult<&str, AssemblerInstruction> {
    directive_combined(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Token;

    #[test]
    fn parse_label_directive() {
        let result = directive_combined("label: .asciiz 'Something'");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(
            Some(Token::LabelDeclaration {
                name: "label".to_string()
            }),
            p.label
        );
        assert_eq!(
            Some(Token::Directive {
                name: "asciiz".to_string()
            }),
            p.directive
        );
        assert_eq!(None, p.opcode);
        assert_eq!(
            Some(Token::IrString {
                name: "Something".to_string()
            }),
            p.operand1
        );
        assert_eq!(None, p.operand2);
        assert_eq!(None, p.operand3);
    }
    #[test]
    fn test_parse_directive() {
        let result = directive_combined(".data\n");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(None, p.label);
        assert_eq!(
            Some(Token::Directive {
                name: "data".to_string()
            }),
            p.directive
        );
        assert_eq!(None, p.opcode);
        assert_eq!(None, p.operand1);
        assert_eq!(None, p.operand2);
        assert_eq!(None, p.operand3);
    }
}
