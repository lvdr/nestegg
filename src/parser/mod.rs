use crate::{token::{tokenize, Token}, instruction::{operation::Operation, operand_mode::OperandMode}, Operand};

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

pub fn parse_statement<'a>(tokens: &'a [Token<'a>]) -> Result<(Statement<'a>, &'a [Token<'a>]), &'static str> {
    let (mut mut_tokens, label) = munch_label(tokens);

    Ok((statement, mut_tokens))
}

pub fn parse<'a>(tokens: &[Token]) -> Program<'a> {

}
