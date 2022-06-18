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
    Number(u16),
}

fn ensure_tokens_available(tokens: &[Token], n: usize) -> Result<(), &'static str> {
    if tokens.len() < n {
        Err("Unexpected end of program")
    } else {
        Ok(())
    }
}

fn munch_label<'a>(tokens: &'a [Token]) -> Result<(&'a [Token<'a>], &'a str), &'static str> {
    ensure_tokens_available(tokens, 2)?;

    match tokens {
        [Token {name: "Label", ..}, Token {name: "Colon", ..}] =>
            Ok((&tokens[2..], tokens[0].text)),
        _ =>
            Err("Didn't find a label"),
    }
}

fn munch_operation<'a>(tokens: &'a [Token]) -> Result<(&'a [Token<'a>], Operation), &'static str> {
    ensure_tokens_available(tokens, 1)?;
    Ok((&tokens[1..], Operation::from_str(&tokens[0].text.to_uppercase())?))
}


fn parse_decimal_number<'a>(tokens: &'a [Token]) -> Result<(&'a [Token<'a>], u16), &'static str> {
    ensure_tokens_available(tokens, 1);

    if let Ok(value) = tokens[0].text.parse::<u16>() {
        Ok((&tokens[1..], value))
    } else {
        Err("Expected decimal number")
    }
}

fn parse_hex_number<'a>(tokens: &'a [Token]) -> Result<(&'a [Token<'a>], u16), &'static str> {
    ensure_tokens_available(tokens, 2);

    if tokens[0].name == "Hex" {
        if let Ok(value) = u16::from_str_radix(tokens[1].text, 16) {
            Ok((&tokens[2..], value))
        } else {
            Err("Expected decimal number")
        }
    } else {
        Err("Couldn't find hex number")
    }
}

fn munch_expression<'a>(tokens: &'a [Token]) -> Result<(&'a [Token<'a>], Expression), &'static str> {
    if let Ok((new_tokens, value)) = parse_decimal_number(tokens) {
        Ok((new_tokens, Expression::Number(value)))
    } else if let Ok((new_tokens, value)) = parse_hex_number(tokens) {
        Ok((new_tokens, Expression::Number(value)))
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
            expression: Expression::Number(0),
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
    use std::fmt::Debug;

    use super::*;

    /// Asserts that `parser(tokens)` returns `expected_value` and consumes
    /// `expected_consumed_tokens` values from `tokens`
    fn assert_parsing_tokens_with_func_gives_and_consumes<'a, T: PartialEq + Debug, E: Debug>(
        tokens: &'a [Token],
        parser: fn (&'a [Token]) -> Result<(&'a [Token<'a>], T), E>,
        expected_value: T,
        expected_consumed_tokens: usize
    ) {
        let (returned_tokens, returned_val) = parser(&tokens).unwrap();
        assert_eq!(returned_val, expected_value);
        assert_eq!(returned_tokens.len(), tokens.len() - expected_consumed_tokens)
    }

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

            assert_parsing_tokens_with_func_gives_and_consumes(&mock_tokens, munch_label, "GotoLabel", 2);
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

            assert_parsing_tokens_with_func_gives_and_consumes(&mock_tokens, munch_operation, Operation::ADC, 1);
        }
    }

    mod describe_munch_expression {
        use super::*;

        mod describe_parse_decimal_number {
            use super::*;

            #[test]
            fn it_works_correctly() {
                let mock_tokens = vec![
                    Token {
                        name: "Number",
                        text: "5312",
                    },
                ];

                assert_parsing_tokens_with_func_gives_and_consumes(&mock_tokens, parse_decimal_number, 5312, 1);
            }
        }


        #[test]
        fn it_works_correctly_for_decimal_numbers() {
            let mock_tokens = vec![
                Token {
                    name: "Number",
                    text: "5312",
                },
            ];

            assert_parsing_tokens_with_func_gives_and_consumes(&mock_tokens, munch_expression, Expression::Number(5312), 1);
        }
    }
}
