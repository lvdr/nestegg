use crate::{token::{tokenize, Token, self}, instruction::{operation::Operation, operand_mode::OperandMode}, Operand};
use std::str::FromStr;


pub struct Program<'a> {
    statements: Vec<Statement<'a>>,
}

pub struct Statement<'a> {
    label: Option<&'a str>,
    operation: Operation,
    operand_mode: OperandMode,
    operand: Option<Operand>,
}

fn munch_label<'a>(tokens: &'a [Token]) -> (&'a [Token<'a>], Option<&'a str>) {
    if tokens.len() >= 2 && tokens[0].name == "Label" && tokens[1].name == "LabelColon" {
        (&tokens[2..], Some(tokens[0].text))
    } else {
        (&tokens, None)
    }
}

fn munch_operation<'a>(tokens: &'a [Token]) -> Result<(&'a [Token<'a>], Operation), &'static str> {
    if tokens.len() == 0 {
        Err("Unexpected end of program")
    } else {
        Ok((&tokens[1..], Operation::from_str(tokens[0].text)?))
    }
}

fn parse_statement<'a>(tokens: &'a [Token<'a>]) -> Result<(Statement<'a>, &'a [Token<'a>]), &'static str> {
    let (tokens_without_label, label) = munch_label(tokens);
    let (tokens_without_operation, operation) = munch_operation(tokens_without_label)?;

    let statement = Statement { label, operation, operand_mode: OperandMode::Absolute, operand: None };

    Ok((statement, tokens_without_operation))
}

pub fn parse<'a>(tokens: &'a [Token<'a>]) -> Result<Program<'a>, &'static str> {
    let mut mut_tokens = tokens;
    let mut statements = vec![];

    while mut_tokens.len() > 0 {
        let parsed_statement = parse_statement(mut_tokens)?;
        mut_tokens = parsed_statement.1;
        statements.push(parsed_statement.0);
    }

    Ok(Program { statements })
}
