use crate::{token::Token, instruction::{operation::Operation, operand_mode::OperandMode}};
use std::str::FromStr;


pub struct Program<'a> {
    statements: Vec<Statement<'a>>,
}

pub struct Statement<'a> {
    label: Option<&'a str>,
    operation: Operation,
    expression: Expression,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Decimal(u16),
}

fn munch_label<'a>(tokens: &'a [Token]) -> Result<(&'a [Token<'a>], &'a str), &'static str> {
    if tokens.len() >= 2 && tokens[0].name == "Label" && tokens[1].name == "Colon" {
        Ok((&tokens[2..], tokens[0].text))
    } else {
        Err("Didn't find a label")
    }
}

fn munch_operation<'a>(tokens: &'a [Token]) -> Result<(&'a [Token<'a>], Operation), &'static str> {
    if tokens.len() == 0 {
        Err("Unexpected end of program")
    } else {
        Ok((&tokens[1..], Operation::from_str(&tokens[0].text.to_uppercase())?))
    }
}

fn munch_expression<'a>(tokens: &'a [Token]) -> Result<(&'a [Token<'a>], Expression), &'static str> {
    if tokens.len() >= 1 && tokens[0].name == "Number" {
        if let Ok(value) = tokens[0].text.parse::<u16>() {
            Ok((
                &tokens[1..],
                Expression::Decimal(value)
            ))
        } else {
                Err("Expected decimal number")
            }
    } else if tokens.len() == 0 {
        Err("Unexpected end of program")
    } else {
        Err("Didn't find an expression")
    }
}

fn parse_statement<'a>(tokens: &'a [Token<'a>]) -> Result<(&'a [Token<'a>], Statement<'a>), &'static str> {
    let mut curren_tokens = tokens;

    let mut label = None;
    if let Ok(maybe_label) = munch_label(tokens) {
        label = Some(maybe_label.1);
        curren_tokens = maybe_label.0;
    }

    let (curren_tokens, operation) = munch_operation(curren_tokens)?;

    Ok((
        curren_tokens,
        Statement {
            label,
            operation,
            expression: Expression::Decimal(0),
        }
    ))
}

pub fn parse<'a>(tokens: &'a [Token<'a>]) -> Result<Program<'a>, &'static str> {
    let mut mut_tokens = tokens;
    let mut statements = vec![];

    while mut_tokens.len() > 0 {
        let parsed_statement = parse_statement(mut_tokens)?;
        mut_tokens = parsed_statement.0;
        statements.push(parsed_statement.1);
    }

    Ok(Program { statements })
}

#[cfg(test)]
mod test {
    use super::*;

    mod describe_munch_label {
        use super::*;

        #[test]
        fn it_works_correctly() {
            let mock_tokens = vec![
                Token {
                    name: "Label",
                    text: "GotoLabel",
                }, Token {
                    name: "Colon",
                    text: ":",
                },
            ];

            let (_, label) = munch_label(&mock_tokens).unwrap();
            assert_eq!(label, "GotoLabel");
        }
    }

    mod describe_munch_operation {
        use super::*;

        #[test]
        fn it_works_correctly() {
            let mock_tokens = vec![
                Token {
                    name: "Text",
                    text: "adc",
                },
            ];

            let (_, op) = munch_operation(&mock_tokens).unwrap();
            assert_eq!(op, Operation::ADC);
        }
    }

    mod describe_munch_expression {
        use super::*;

        #[test]
        fn it_works_correctly_for_decimals() {
            let mock_tokens = vec![
                Token {
                    name: "Number",
                    text: "5312",
                },
            ];

            let (_, expr) = munch_expression(&mock_tokens).unwrap();
            assert_eq!(expr, Expression::Decimal(5312));
        }
    }
}
