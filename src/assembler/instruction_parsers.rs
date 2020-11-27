use super::label_parsers::label_declaration;
use super::opcode_parsers::*;
use super::operand_parsers::operand;
use super::{SymbolTable, Token};
use byteorder::{ByteOrder, LittleEndian};
use nom::{combinator::opt, sequence::tuple, IResult};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub opcode: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut results = vec![];
        if let Some(ref token) = self.opcode {
            match token {
                Token::Op { code } => match code {
                    _ => {
                        let b: u8 = (*code).into();
                        results.push(b);
                    }
                },
                _ => {
                    println!("Non-opcode found in opcode field");
                    std::process::exit(1);
                }
            }
        }

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results, symbols)
            }
        }

        while results.len() < 4 {
            results.push(0);
        }

        return results;
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>, symbols: &SymbolTable) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            Token::LabelUsage { name } => {
                if let Some(value) = symbols.symbol_value(name) {
                    let mut wtr = vec![];
                    LittleEndian::write_u32(&mut wtr, value);
                    results.push(wtr[1]);
                    results.push(wtr[0]);
                } else {
                    println!("No value found for {:?}", name);
                }
            }

            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }
    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    pub fn get_label_name(&self) -> Option<String> {
        match &self.label {
            Some(l) => match l {
                Token::LabelDeclaration { name } => Some(name.clone()),
                _ => None,
            },
            None => None,
        }
    }
}

fn instruction_combined(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (leftover, (l, op, o1, o2, o3)) = tuple((
        opt(label_declaration),
        opcode,
        opt(operand),
        opt(operand),
        opt(operand),
    ))(input)?;
    Ok((
        leftover,
        AssemblerInstruction {
            label: l,
            directive: None,
            opcode: Some(op),
            operand1: o1,
            operand2: o2,
            operand3: o3,
        },
    ))
}

pub fn instruction(input: &str) -> IResult<&str, AssemblerInstruction> {
    instruction_combined(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Token;
    use crate::instruction::Opcode;

    #[test]
    fn parse_label_instruction_zero() {
        let result = instruction_combined("label: HLT");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(
            Some(Token::LabelDeclaration {
                name: "label".to_string()
            }),
            p.label
        );
        assert_eq!(None, p.directive);
        assert_eq!(Some(Token::Op { code: Opcode::HLT }), p.opcode);
        assert_eq!(None, p.operand1);
        assert_eq!(None, p.operand2);
        assert_eq!(None, p.operand3);
    }
    #[test]
    fn test_parse_instruction_zero() {
        let result = instruction_combined("HLT\n");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(None, p.label);
        assert_eq!(None, p.directive);
        assert_eq!(Some(Token::Op { code: Opcode::HLT }), p.opcode);
        assert_eq!(None, p.operand1);
        assert_eq!(None, p.operand2);
        assert_eq!(None, p.operand3);
    }
    #[test]
    fn test_parse_label_instruction_one() {
        let result = instruction_combined("label: ALOC $0");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(
            Some(Token::LabelDeclaration {
                name: "label".to_string()
            }),
            p.label
        );
        assert_eq!(None, p.directive);
        assert_eq!(Some(Token::Op { code: Opcode::ALOC }), p.opcode);
        assert_eq!(Some(Token::Register { reg_num: 0 }), p.operand1);
        assert_eq!(None, p.operand2);
        assert_eq!(None, p.operand3);
    }
    #[test]
    fn test_parse_instruction_one() {
        let result = instruction_combined("ALOC $0\n");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(None, p.label);
        assert_eq!(None, p.directive);
        assert_eq!(Some(Token::Op { code: Opcode::ALOC }), p.opcode);
        assert_eq!(Some(Token::Register { reg_num: 0 }), p.operand1);
        assert_eq!(None, p.operand2);
        assert_eq!(None, p.operand3);
    }
    #[test]
    fn test_parse_label_instruction_two() {
        let result = instruction_combined("label: load $0 #100\n");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(
            Some(Token::LabelDeclaration {
                name: "label".to_string()
            }),
            p.label
        );
        assert_eq!(None, p.directive);
        assert_eq!(Some(Token::Op { code: Opcode::LOAD }), p.opcode);
        assert_eq!(Some(Token::Register { reg_num: 0 }), p.operand1);
        assert_eq!(Some(Token::IntegerOperand { value: 100 }), p.operand2);
        assert_eq!(None, p.operand3);
    }
    #[test]
    fn test_parse_instruction_two() {
        let result = instruction_combined("load $0 #100\n");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(None, p.label);
        assert_eq!(None, p.directive);
        assert_eq!(Some(Token::Op { code: Opcode::LOAD }), p.opcode);
        assert_eq!(Some(Token::Register { reg_num: 0 }), p.operand1);
        assert_eq!(Some(Token::IntegerOperand { value: 100 }), p.operand2);
        assert_eq!(None, p.operand3);
    }
    #[test]
    fn test_parse_label_instruction_three() {
        let result = instruction_combined("label: add $0 $1 $2\n");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(
            Some(Token::LabelDeclaration {
                name: "label".to_string()
            }),
            p.label
        );
        assert_eq!(None, p.directive);
        assert_eq!(Some(Token::Op { code: Opcode::ADD }), p.opcode);
        assert_eq!(Some(Token::Register { reg_num: 0 }), p.operand1);
        assert_eq!(Some(Token::Register { reg_num: 1 }), p.operand2);
        assert_eq!(Some(Token::Register { reg_num: 2 }), p.operand3);
    }
    #[test]
    fn test_parse_instruction_three() {
        let result = instruction_combined("add $0 $1 $2\n");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(None, p.label);
        assert_eq!(None, p.directive);
        assert_eq!(Some(Token::Op { code: Opcode::ADD }), p.opcode);
        assert_eq!(Some(Token::Register { reg_num: 0 }), p.operand1);
        assert_eq!(Some(Token::Register { reg_num: 1 }), p.operand2);
        assert_eq!(Some(Token::Register { reg_num: 2 }), p.operand3);
    }
}
