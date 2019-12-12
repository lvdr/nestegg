use std::vec::Vec;

mod instruction;

use instruction::decode_instruction;
use instruction::operand_mode::OperandMode;
use instruction::operation::Operation;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct RegisterFile {
    accumulator : u8,
    x : u8,
    y : u8,
    status : u8,
    stack_pointer : u8,
    program_counter : u16,
}

pub enum StatusFlag
{
    CARRY     = 0,
    ZERO      = 1,
    INTERRUPT = 2,
    DECIMAL   = 3,
    BREAK     = 4,
    RESERVED  = 5,
    OVERFLOW  = 6,
    NEGATIVE  = 7
}


#[derive(Clone, PartialEq, Eq)]
pub struct ComputerState {
    pub memory : Vec<u8>,
    pub registers : RegisterFile,
}


impl ComputerState {
    pub fn initialize_from_image(memory : Vec<u8>) -> ComputerState {
        ComputerState {memory, registers: RegisterFile{ ..Default::default()}}
    }

    pub fn get_byte_from_memory(&self, index: usize) -> u8 {
        self.memory[index]
    }

    pub fn get_word_from_memory(&self, index: usize) -> u16 {
        let low = self.memory[index] as u16;
        let high = self.memory[index + 1] as u16;
        (high << 8) | low
    }

    pub fn step(mut self) -> Result<Self, &'static str> {
        let instruction = self.memory[self.registers.program_counter as usize];
        self.registers.program_counter += 1;

        let decoded_instruction = decode_instruction(instruction)?;

        let operand = self.fetch_operand(decoded_instruction.0);

        self.execute_operation(decoded_instruction.1, operand)?;

        Ok(self)
    }

    pub fn multiple_steps(self, steps: u32) -> Result<Self, &'static str> {
        (0..steps).try_fold(self, |state, _| state.step())
    }

    fn get_status_flag(&self, status_flag: StatusFlag) -> bool
    {
        let index = status_flag as u8;
        let flag = self.registers.status >> index;

        return (flag & 0x1) != 0;
    }

    fn set_status_flag(&mut self, status_flag: StatusFlag, new_flag: bool)
    {
        let index = status_flag as u8;
        self.registers.status &= !(1 << index);
        self.registers.status |= (new_flag as u8) << index;
    }

    fn execute_operation(&mut self, op : Operation, operand: u8) -> Result<(), &'static str> {
        match op {
            Operation::NOP => Ok(()),
            _ => Err("Unimplemented operation")
       }
    }

    fn fetch_operand(&mut self, mode : OperandMode) -> u8 {
        let pc = self.registers.program_counter as usize;
        match mode {
            OperandMode::Absolute => {
                let address = self.get_word_from_memory(pc) as usize;
                let operand = self.get_byte_from_memory(address);
                self.registers.program_counter += 2;
                operand
            },
            OperandMode::AbsoluteX => {
                let address = self.get_word_from_memory(pc) as usize;
                let offset = self.registers.x as usize;
                let operand = self.get_byte_from_memory(address + offset);
                self.registers.program_counter += 2;
                operand
            },
            OperandMode::AbsoluteY => {
                let address = self.get_word_from_memory(pc) as usize;
                let offset = self.registers.y as usize;
                let operand = self.get_byte_from_memory(address + offset);
                self.registers.program_counter += 2;
                operand
            },
            OperandMode::Accumulator => self.registers.accumulator,
            OperandMode::Immediate => {
                self.registers.program_counter += 2;
                self.get_byte_from_memory(pc)
            },
            OperandMode::Implied => 0,
            OperandMode::Indirect => {
                let pointer_address = self.get_word_from_memory(pc) as usize;
                let pointer = self.get_word_from_memory(pointer_address) as usize;
                let operand = self.get_byte_from_memory(pointer);
                self.registers.program_counter += 2;
                operand
            },
            OperandMode::IndirectX => {
                let address = self.get_byte_from_memory(pc) as usize;
                let offset = self.registers.x as usize;
                let pointer_address = (address + offset) & 0xff;
                let pointer = self.get_word_from_memory(pointer_address) as usize;
                let operand = self.get_byte_from_memory(pointer);
                self.registers.program_counter += 1;
                operand
            },
            OperandMode::IndirectY => {
                let address = self.get_byte_from_memory(pc) as usize;
                let pointer = self.get_word_from_memory(address) as usize;
                let offset = self.registers.x as usize;
                let operand = self.get_byte_from_memory(pointer + offset);
                self.registers.program_counter += 1;
                operand
            },
            OperandMode::ZeroPage => {
                let address = self.get_byte_from_memory(pc) as usize;
                let operand = self.get_byte_from_memory(address);
                self.registers.program_counter += 1;
                operand
            },
            OperandMode::ZeroPageX => {
                let address = self.get_byte_from_memory(pc) as usize;
                let offset = self.registers.x as usize;
                let final_address = (address + offset) & 0xff;
                let operand = self.get_byte_from_memory(final_address);
                self.registers.program_counter += 1;
                operand
            },
            OperandMode::ZeroPageY => {
                let address = self.get_byte_from_memory(pc) as usize;
                let offset = self.registers.y as usize;
                let final_address = (address + offset) & 0xff;
                let operand = self.get_byte_from_memory(final_address);
                self.registers.program_counter += 1;
                operand
            },
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    mod describe_computer_state {
        use super::*;

        #[test]
        fn it_fetches_bytes() {
            const NUM_VALUES : usize = 1024;
            let mut initial_memory = Vec::new();
            for i in 0..NUM_VALUES {
                let masked = (i & 0xff) as u8;
                let inverted = masked ^ 0xff;
                initial_memory.push(inverted);
            }

            let state = ComputerState::initialize_from_image(initial_memory.clone());

            for i in  0..NUM_VALUES {
                assert_eq!(state.get_byte_from_memory(i), initial_memory[i]);
            }
        }

        #[test]
        fn it_fetches_words() {
            const NUM_VALUES : usize = 1024;
            let mut initial_memory = Vec::new();
            let mut expected_memory = Vec::new();
            for i in 0..NUM_VALUES {
                let inverted = i ^ 0xffff;
                let masked_low = (inverted & 0xff) as u8;
                initial_memory.push(masked_low);
                let masked_high = ((inverted >> 8) & 0xff) as u8;
                initial_memory.push(masked_high);

                expected_memory.push(inverted as u16);
            }

            let state = ComputerState::initialize_from_image(initial_memory.clone());

            for i in  0..NUM_VALUES {
                assert_eq!(state.get_word_from_memory(i*2), expected_memory[i]);
            }
        }

        #[test]
        fn it_executes_nop_without_changing_anything() {
            let mut state = ComputerState::initialize_from_image(vec![0; 1024]);
            let state_initial_registers = state.registers;

            state.execute_operation(Operation::NOP, 0).expect("Couldn't execute NOP");

            assert_eq!(state_initial_registers, state.registers);
        }

        #[test]
        fn it_sets_status_flags() {
            let mut state = ComputerState::initialize_from_image(vec![0; 1024]);
            state.registers.status = 0b00110011;
            state.set_status_flag(StatusFlag::ZERO, false);

            assert_eq!(state.registers.status, 0b00110001);
        }

        #[test]
        fn it_gets_status_flags() {
            let mut state = ComputerState::initialize_from_image(vec![0; 1024]);
            state.registers.status = 0b00110011;

            assert_eq!(state.get_status_flag(StatusFlag::ZERO), true);
            assert_eq!(state.get_status_flag(StatusFlag::NEGATIVE), false);
        }
    }
}
