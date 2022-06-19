use crate::{token::Token, instruction::operation::Operation};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum AssemblyTokensType {
    CloseBrackets,
    Colon,
    Comma,
    Hash,
    Hex,
    Number,
    OpenBrackets,
    Text,
    X,
    Y,
}

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
    Immediate(u16),
    AbsoluteIndirect(u16),
    AbsoluteIndexedWithX(u16),
    AbsoluteIndexedWithY(u16),
    ZeroPageIndexedWithX(u8),
    ZeroPageIndexedWithY(u8),
    ZeroPageIndexedIndirect(u8),
    ZeroPageIndirectIndexedWithY(u8),
    ZeroPage(u8),
}

pub type TokenList<'a> = &'a [Token<'a, AssemblyTokensType>];
pub type ParserResult<'a, ParsedType> = Result<(TokenList<'a>, ParsedType), &'static str>;

fn ensure_tokens_available<'a, T>(tokens: &'a [Token<'a, T>], n: usize) -> Result<(), &'static str>{
    if tokens.len() < n {
        Err("Unexpected end of program")
    } else {
        Ok(())
    }
}

fn parse_label<'a>(tokens: TokenList<'a>) -> ParserResult<'a, &'a str> {
    ensure_tokens_available(tokens, 2)?;

    match tokens {
        [Token {token_type: AssemblyTokensType::Text, ..}, Token {token_type: AssemblyTokensType::Colon, ..}] =>
            Ok((&tokens[2..], tokens[0].text)),
        _ =>
            Err("Didn't find a label"),
    }
}

fn parse_operation<'a>(tokens: TokenList<'a>) -> ParserResult<'a, Operation> {
    ensure_tokens_available(tokens, 1)?;
    Ok((&tokens[1..], Operation::from_str(&tokens[0].text.to_uppercase())?))
}


fn parse_decimal_number<'a>(tokens: TokenList<'a>) -> ParserResult<'a, u16>{
    ensure_tokens_available(tokens, 1);

    if let Ok(value) = tokens[0].text.parse::<u16>() {
        Ok((&tokens[1..], value))
    } else {
        Err("Expected decimal number")
    }
}

fn parse_hex_number<'a>(tokens: TokenList<'a>) -> ParserResult<'a, u16> {
    ensure_tokens_available(tokens, 2);

    if tokens[0].token_type == AssemblyTokensType::Hex {
        if let Ok(value) = u16::from_str_radix(tokens[1].text, 16) {
            Ok((&tokens[2..], value))
        } else {
            Err("Expected decimal number")
        }
    } else {
        Err("Couldn't find hex number")
    }
}

fn parse_hash<'a>(tokens: TokenList<'a>) -> ParserResult<()> {
    ensure_tokens_available(tokens, 1);

    match tokens {
        [Token {token_type: AssemblyTokensType::Hash, ..}] =>
            Ok((&tokens[1..], ())),
        _ =>
            Err("Didn't find #"),
    }
}

fn parse_y<'a>(tokens: TokenList<'a>) -> ParserResult<()> {
    ensure_tokens_available(tokens, 1);

    match tokens {
        [Token {token_type: AssemblyTokensType::Y, ..}] =>
            Ok((&tokens[1..], ())),
        _ =>
            Err("Didn't find Y"),
    }
}

fn parse_x<'a>(tokens: TokenList<'a>) -> ParserResult<()> {
    ensure_tokens_available(tokens, 1);

    match tokens {
        [Token {token_type: AssemblyTokensType::X, ..}] =>
            Ok((&tokens[1..], ())),
        _ =>
            Err("Didn't find X"),
    }
}

fn parse_comma<'a>(tokens: TokenList<'a>) -> ParserResult<()> {
    ensure_tokens_available(tokens, 1);

    match tokens {
        [Token {token_type: AssemblyTokensType::Comma, ..}] =>
            Ok((&tokens[1..], ())),
        _ =>
            Err("Didn't find X"),
    }
}

fn parse_close_brackets<'a>(tokens: TokenList<'a>) -> ParserResult<'a, ()> {
    ensure_tokens_available(tokens, 1);

    if tokens[0].token_type == AssemblyTokensType::CloseBrackets {
        Ok((&tokens[1..], ()))
    } else {
        Err("Couldn't find close brackets")
    }
}

fn parse_open_brackets<'a>(tokens: TokenList<'a>) -> ParserResult<'a, ()> {
    ensure_tokens_available(tokens, 1);

    if tokens[0].token_type == AssemblyTokensType::OpenBrackets {
        Ok((&tokens[1..], ()))
    } else {
        Err("Couldn't find open brackets")
    }
}

fn parse_word<'a>(tokens: TokenList<'a>) -> ParserResult<u16> {
    if let Ok((new_tokens, value)) = parse_decimal_number(tokens) {
        Ok((new_tokens, value))
    } else if let Ok((new_tokens, value)) = parse_hex_number(tokens) {
        Ok((new_tokens, value))
    } else {
        Err("Didn't find a number")
    }
}

fn parse_byte<'a>(tokens: TokenList<'a>) -> ParserResult<u8> {
    let (parsed_tokens, word) = parse_word(tokens)?;

    if word < u8::max_value().into() {
        Ok((parsed_tokens, word as u8))
    } else {
        Err("Given number doesn't fit in a byte")
    }
}

fn parse_absolute_indirect_addressing<'a>(tokens: TokenList<'a>) -> ParserResult<u16> {
    let (parsed_tokens, _) = parse_open_brackets(tokens)?;
    let (parsed_tokens, num) = parse_word(parsed_tokens)?;
    let (parsed_tokens, _) = parse_close_brackets(parsed_tokens)?;

    Ok((parsed_tokens, num))
}

fn parse_immediate_addressing<'a>(tokens: TokenList<'a>) -> ParserResult<u16> {
    let (parsed_tokens, _) = parse_hash(tokens)?;
    let (parsed_tokens, num) = parse_word(parsed_tokens)?;

    Ok((parsed_tokens, num))
}

fn parse_absolute_with_x_addressing<'a>(tokens: TokenList<'a>) -> ParserResult<u16> {
    let (parsed_tokens, num) = parse_word(tokens)?;
    let (parsed_tokens, _) = parse_comma(parsed_tokens)?;
    let (parsed_tokens, _) = parse_x(parsed_tokens)?;

    Ok((parsed_tokens, num))
}

fn parse_absolute_with_y_addressing<'a>(tokens: TokenList<'a>) -> ParserResult<u16> {
    let (parsed_tokens, num) = parse_word(tokens)?;
    let (parsed_tokens, _) = parse_comma(parsed_tokens)?;
    let (parsed_tokens, _) = parse_y(parsed_tokens)?;

    Ok((parsed_tokens, num))
}

fn parse_zero_page<'a>(tokens: TokenList<'a>) -> ParserResult<u8> {
    let (parsed_tokens, num) = parse_byte(tokens)?;

    Ok((parsed_tokens, num))
}

fn parse_zero_page_indexed_with_x<'a>(tokens: TokenList<'a>) -> ParserResult<u8> {
    let (parsed_tokens, num) = parse_byte(tokens)?;
    let (parsed_tokens, _) = parse_comma(parsed_tokens)?;
    let (parsed_tokens, _) = parse_x(parsed_tokens)?;

    Ok((parsed_tokens, num))
}

fn parse_zero_page_indexed_with_y<'a>(tokens: TokenList<'a>) -> ParserResult<u8> {
    let (parsed_tokens, num) = parse_byte(tokens)?;
    let (parsed_tokens, _) = parse_comma(parsed_tokens)?;
    let (parsed_tokens, _) = parse_y(parsed_tokens)?;

    Ok((parsed_tokens, num))
}

fn parse_zero_page_indexed_indirect<'a>(tokens: TokenList<'a>) -> ParserResult<u8> {
    let (parsed_tokens, _) = parse_open_brackets(tokens)?;
    let (parsed_tokens, num) = parse_byte(parsed_tokens)?;
    let (parsed_tokens, _) = parse_close_brackets(parsed_tokens)?;
    let (parsed_tokens, _) = parse_comma(parsed_tokens)?;
    let (parsed_tokens, _) = parse_y(parsed_tokens)?;

    Ok((parsed_tokens, num))
}

fn parse_zero_page_indirect_indexed_with_y<'a>(tokens: TokenList<'a>) -> ParserResult<u8> {
    let (parsed_tokens, _) = parse_open_brackets(tokens)?;
    let (parsed_tokens, num) = parse_byte(parsed_tokens)?;
    let (parsed_tokens, _) = parse_close_brackets(parsed_tokens)?;
    let (parsed_tokens, _) = parse_comma(parsed_tokens)?;
    let (parsed_tokens, _) = parse_y(parsed_tokens)?;

    Ok((parsed_tokens, num))
}

fn parse_expression<'a>(tokens: TokenList<'a>) -> Result<(TokenList<'a>, Expression), &'static str> {
    if let Ok((new_tokens, value)) = parse_word(tokens) {
        Ok((new_tokens, Expression::Number(value)))
    } else if let Ok((new_tokens, value)) = parse_immediate_addressing(tokens) {
        Ok((new_tokens, Expression::Immediate(value)))
    } else if let Ok((new_tokens, value)) = parse_byte(tokens) {
        Ok((new_tokens, Expression::ZeroPage(value)))
    } else if let Ok((new_tokens, value)) = parse_immediate_addressing(tokens) {
        Ok((new_tokens, Expression::Immediate(value)))
    } else if let Ok((new_tokens, value)) = parse_zero_page_indexed_indirect(tokens) {
        Ok((new_tokens, Expression::ZeroPageIndexedIndirect(value)))
    } else if let Ok((new_tokens, value)) = parse_zero_page_indexed_indirect(tokens) {
        Ok((new_tokens, Expression::ZeroPageIndexedIndirect(value)))
    } else if let Ok((new_tokens, value)) = parse_zero_page_indexed_with_x(tokens) {
        Ok((new_tokens, Expression::ZeroPageIndexedWithX(value)))
    } else if let Ok((new_tokens, value)) = parse_zero_page_indexed_with_y (tokens) {
        Ok((new_tokens, Expression::ZeroPageIndexedWithY(value)))
    } else if let Ok((new_tokens, value)) = parse_zero_page_indexed_indirect(tokens) {
        Ok((new_tokens, Expression::ZeroPageIndexedIndirect(value)))
    } else if let Ok((new_tokens, value)) = parse_zero_page_indirect_indexed_with_y(tokens) {
        Ok((new_tokens, Expression::ZeroPageIndirectIndexedWithY(value)))
    } else {
        Err("Didn't find an expression")
    }
}

fn parse_statement<'a>(tokens: TokenList<'a>) -> Result<(TokenList<'a>, Statement<'a>), &'static str> {
    let mut curren_tokens = tokens;

    let mut label = None;
    if let Ok(maybe_label) = parse_label(tokens) {
        label = Some(maybe_label.1);
        curren_tokens = maybe_label.0;
    }

    let (curren_tokens, operation) = parse_operation(curren_tokens)?;

    Ok((
        curren_tokens,
        Statement {
            label,
            operation,
            expression: Expression::Number(0),
        }
    ))
}

pub fn parse<'a>(tokens: TokenList<'a>) -> Result<Program<'a>, &'static str> {
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
        tokens: TokenList<'a>,
        parser: fn (TokenList<'a>) -> Result<(TokenList<'a>, T), E>,
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
                    token_type: AssemblyTokensType::Text,
                    text: "GotoLabel",
                }, Token {
                    token_type: AssemblyTokensType::Colon,
                    text: ":",
                },
            ];

            assert_parsing_tokens_with_func_gives_and_consumes(&mock_tokens, parse_label, "GotoLabel", 2);
        }
    }

    mod describe_munch_operation {
        use super::*;

        #[test]
        fn it_works_correctly() {
            let mock_tokens = vec![
                Token {
                    token_type: AssemblyTokensType::Text,
                    text: "adc",
                },
            ];

            assert_parsing_tokens_with_func_gives_and_consumes(&mock_tokens, parse_operation, Operation::ADC, 1);
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
                        token_type: AssemblyTokensType::Number,
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
                    token_type: AssemblyTokensType::Number,
                    text: "5312",
                },
            ];

            assert_parsing_tokens_with_func_gives_and_consumes(&mock_tokens, parse_expression, Expression::Number(5312), 1);
        }
    }
}
