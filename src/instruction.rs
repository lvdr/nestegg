pub mod operand_mode;
pub mod operation;

use operand_mode::OperandMode;
use operation::Operation;

pub struct Instruction(OperandMode, Operation);

pub fn decode_instruction(instruction: u8) -> Result<Instruction, &'static str> {
    return Err("Can't find instruction");
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    mod describe_decode_instruction {
        use super::*;

        #[test]
        fn it_errors_when_instruction_not_found() {
            assert!(decode_instruction(0x80).is_err());
        }
    }
}

