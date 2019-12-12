use std::vec::Vec;

mod instruction;
mod util;

use instruction::decode_instruction;
use instruction::operand_mode::OperandMode;
use instruction::operation::Operation;
use util::is_negative;

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
            Operation::ADC => Ok(self.execute_add_with_carry(operand)),
            _ => Err("Unimplemented operation")
       }
    }

    fn fetch_operand(&mut self, mode : OperandMode) -> u8 {
        let pc = self.registers.program_counter as usize;
        match mode {
            OperandMode::Absolute => self.get_absolute_operand(),
            OperandMode::AbsoluteX => self.get_absolute_x_operand(),
            OperandMode::AbsoluteY => self.get_absolute_y_operand(),
            OperandMode::Accumulator => self.get_accumulator_operand(),
            OperandMode::Immediate => self.get_immediate_operand(),
            OperandMode::Implied => self.get_implied_operand(),
            OperandMode::Indirect => self.get_indirect_operand(),
            OperandMode::IndirectX => self.get_indirect_x_operand(),
            OperandMode::IndirectY => self.get_indirect_y_operand(),
            OperandMode::ZeroPage => self.get_zero_page_operand(),
            OperandMode::ZeroPageX => self.get_zero_page_x_operand(),
            OperandMode::ZeroPageY => self.get_zero_page_y_operand(),
        }
    }

    fn get_absolute_operand(&mut self) -> u8 {
        let address = self.get_word_from_memory(self.registers.program_counter as usize) as usize;
        let operand = self.get_byte_from_memory(address);
        self.registers.program_counter += 2;
        operand
    }

    fn get_absolute_x_operand(&mut self) -> u8 {
        let address = self.get_word_from_memory(self.registers.program_counter as usize) as usize;
        let offset = self.registers.x as usize;
        let operand = self.get_byte_from_memory(address + offset);
        self.registers.program_counter += 2;
        operand
    }

    fn get_absolute_y_operand(&mut self) -> u8 {
        let address = self.get_word_from_memory(self.registers.program_counter as usize) as usize;
        let offset = self.registers.y as usize;
        let operand = self.get_byte_from_memory(address + offset);
        self.registers.program_counter += 2;
        operand
    }

    fn get_accumulator_operand(&mut self) -> u8 {
        self.registers.accumulator
    }

    fn get_immediate_operand(&mut self) -> u8 {
        self.registers.program_counter += 2;
        self.get_byte_from_memory(self.registers.program_counter as usize)
    }

    fn get_implied_operand(&mut self) -> u8 {
        0
    }

    fn get_indirect_operand(&mut self) -> u8 {
        let pointer_address = self.get_word_from_memory(self.registers.program_counter as usize) as usize;
        let pointer = self.get_word_from_memory(pointer_address) as usize;
        let operand = self.get_byte_from_memory(pointer);
        self.registers.program_counter += 2;
        operand
    }

    fn get_indirect_x_operand(&mut self) -> u8 {
        let address = self.get_byte_from_memory(self.registers.program_counter as usize) as usize;
        let offset = self.registers.x as usize;
        let pointer_address = (address + offset) & 0xff;
        let pointer = self.get_word_from_memory(pointer_address) as usize;
        let operand = self.get_byte_from_memory(pointer);
        self.registers.program_counter += 1;
        operand
    }

    fn get_indirect_y_operand(&mut self) -> u8 {
        let address = self.get_byte_from_memory(self.registers.program_counter as usize) as usize;
        let pointer = self.get_word_from_memory(address) as usize;
        let offset = self.registers.x as usize;
        let operand = self.get_byte_from_memory(pointer + offset);
        self.registers.program_counter += 1;
        operand
    }

    fn get_zero_page_operand(&mut self) -> u8 {
        let address = self.get_byte_from_memory(self.registers.program_counter as usize) as usize;
        let operand = self.get_byte_from_memory(address);
        self.registers.program_counter += 1;
        operand
    }

    fn get_zero_page_x_operand(&mut self) -> u8 {
        let address = self.get_byte_from_memory(self.registers.program_counter as usize) as usize;
        let offset = self.registers.x as usize;
        let final_address = (address + offset) & 0xff;
        let operand = self.get_byte_from_memory(final_address);
        self.registers.program_counter += 1;
        operand
    }

    fn get_zero_page_y_operand(&mut self) -> u8 {
        let address = self.get_byte_from_memory(self.registers.program_counter as usize) as usize;
        let offset = self.registers.y as usize;
        let final_address = (address + offset) & 0xff;
        let operand = self.get_byte_from_memory(final_address);
        self.registers.program_counter += 1;
        operand
    }

    fn execute_add_with_carry(&mut self, operand: u8)
    {
        let carry: u16 = self.get_status_flag(StatusFlag::CARRY) as u16;
        let accumulator: u16 = self.registers.accumulator as u16;

        let mut sum: u16 = operand as u16 + carry + accumulator;

        let new_carry: bool = (sum & (1 << 8)) != 0;
        self.set_status_flag(StatusFlag::CARRY, new_carry);

        sum &= 0xFF;

        let byte_positive: bool = !is_negative(operand);
        let accumulator_positive: bool = !is_negative(accumulator as u8);
        let sum_positive: bool = !is_negative(sum as u8);
        let overflow: bool = (byte_positive == accumulator_positive) && (sum_positive != byte_positive);
        self.set_status_flag(StatusFlag::OVERFLOW, overflow);

        self.registers.accumulator = sum as u8;
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
        fn it_executes_adc() {
            let mut state = ComputerState::initialize_from_image(vec![0; 1024]);

            state.registers.accumulator = 33;
            state.execute_operation(Operation::ADC, 24).expect("Couldn't execute ADC");

            assert_eq!(state.registers.accumulator, 33 + 24);
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
