use crate::{token::{tokenize, Token}, instruction::{operation::Operation, operand_mode::OperandMode}, Operand};

pub struct Program {
    statements: Vec<Statements>,
}

pub struct Statement<'a> {
    label: Option<&'a str>,
    operation: Operation,
    operand_mode: OperandMode,
    operand: Option<Operand>,
}

pub fn parse(tokens: Vec<Token>) -> Program {

}
