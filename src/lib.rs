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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operand {
    Accumulator,
    Address(u16),
    Immediate(u8),
    Implied,
}

impl ComputerState {

    pub fn initialize() -> ComputerState {
        ComputerState {memory: vec![0; 2usize.pow(16)],
                       registers: RegisterFile{ ..Default::default()}}
    }

    pub fn initialize_from_image(memory : Vec<u8>) -> ComputerState {
        ComputerState {memory, registers: RegisterFile{ ..Default::default()}}
    }

    pub fn get_byte_from_memory(&self, index: usize) -> u8 {
        self.memory[index]
    }

    pub fn get_word_from_memory(&self, index: usize) -> u16 {
        let low = self.memory[index];
        let high = self.memory[index + 1];
        u16::from_le_bytes([low, high])
    }

    pub fn write_byte_to_memory(&mut self, index: usize, value: u8) {
        self.memory[index] = value;
    }

    pub fn write_word_to_memory(&mut self, index: usize, value: u16) {
        let bytes = value.to_le_bytes();
        self.memory[index] = bytes[0];
        self.memory[index + 1] = bytes[1];
    }

    pub fn pull_byte_from_stack(&mut self) -> u8 {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        let stack_address = self.registers.stack_pointer as usize + 0x100;
        self.get_byte_from_memory(stack_address)
    }

    pub fn pull_word_from_stack(&mut self) -> u16 {
        let low = self.pull_byte_from_stack();
        let high = self.pull_byte_from_stack();
        u16::from_le_bytes([low, high])
    }

    pub fn push_byte_to_stack(&mut self, value: u8) {
        let stack_address = self.registers.stack_pointer as usize + 0x100;
        self.write_byte_to_memory(stack_address, value);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
    }

    pub fn push_word_to_stack(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.push_byte_to_stack(bytes[1]);
        self.push_byte_to_stack(bytes[0]);
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

    fn get_operand_value(&self, operand: Operand) -> Result<u8, &'static str> {
        match operand {
            Operand::Accumulator => Ok(self.registers.accumulator),
            Operand::Address(addr) => Ok(self.get_byte_from_memory(addr as usize)),
            Operand::Immediate(value) => Ok(value),
            Operand::Implied => Err("Cannot get implied operand value")
        }
    }

    fn set_operand_value(&mut self, operand: Operand, value: u8) -> Result<(), &'static str> {
        match operand {
            Operand::Accumulator => self.registers.accumulator = value,
            Operand::Address(addr) => self.write_byte_to_memory(addr as usize, value),
            Operand::Immediate(_) => return Err("Cannot set immediate operand value"),
            Operand::Implied => return Err("Cannot set implied operand value"),
        }
        Ok(())
    }

    fn get_status_flag(&self, status_flag: StatusFlag) -> bool {
        let index = status_flag as u8;
        let flag = self.registers.status >> index;

        return (flag & 0x1) != 0;
    }

    fn set_status_flag(&mut self, status_flag: StatusFlag, new_flag: bool) {
        let index = status_flag as u8;
        self.registers.status &= !(1 << index);
        self.registers.status |= (new_flag as u8) << index;
    }

    fn set_zero_and_negative_flags(&mut self, value: u8) {
        self.set_status_flag(StatusFlag::ZERO, value == 0);
        self.set_status_flag(StatusFlag::NEGATIVE, value & (1 << 7) != 0);
    }

    fn execute_operation(&mut self, op : Operation, operand: Operand) -> Result<(), &'static str> {
        match op {
            Operation::ADC => Ok(self.execute_add_with_carry(operand)?),
            Operation::AND => Ok(self.execute_and(operand)?),
            Operation::ASL => Ok(self.execute_left_shift(operand)?),
            Operation::BCC => Ok(self.execute_branch_if(operand, StatusFlag::CARRY, false)?),
            Operation::BCS => Ok(self.execute_branch_if(operand, StatusFlag::CARRY, true)?),
            Operation::BEQ => Ok(self.execute_branch_if(operand, StatusFlag::ZERO, true)?),
            Operation::BIT => Ok(self.execute_bit_test(operand)?),
            Operation::BMI => Ok(self.execute_branch_if(operand, StatusFlag::NEGATIVE, true)?),
            Operation::BNE => Ok(self.execute_branch_if(operand, StatusFlag::ZERO, false)?),
            Operation::BPL => Ok(self.execute_branch_if(operand, StatusFlag::NEGATIVE, false)?),
            //Operation::BRK
            Operation::BVC => Ok(self.execute_branch_if(operand, StatusFlag::OVERFLOW, false)?),
            Operation::BVS => Ok(self.execute_branch_if(operand, StatusFlag::OVERFLOW, true)?),
            Operation::CLC => Ok(self.set_status_flag(StatusFlag::CARRY, false)),
            Operation::CLD => Ok(self.set_status_flag(StatusFlag::DECIMAL, false)),
            Operation::CLI => Ok(self.set_status_flag(StatusFlag::INTERRUPT, false)),
            Operation::CLV => Ok(self.set_status_flag(StatusFlag::OVERFLOW, false)),
            Operation::CMP => Ok(self.execute_compare(operand, self.registers.accumulator)?),
            Operation::CPX => Ok(self.execute_compare(operand, self.registers.x)?),
            Operation::CPY => Ok(self.execute_compare(operand, self.registers.y)?),
            Operation::DEC => Ok(self.execute_increment(operand, true)?),
            Operation::DEX => Ok(self.execute_increment_x(true)?),
            Operation::DEY => Ok(self.execute_increment_y(true)?),
            Operation::EOR => Ok(self.execute_exclusive_or(operand)?),
            Operation::INC => Ok(self.execute_increment(operand, false)?),
            Operation::INX => Ok(self.execute_increment_x(false)?),
            Operation::INY => Ok(self.execute_increment_y(false)?),
            Operation::JMP => Ok(self.execute_jump(operand, false)?),
            Operation::JSR => Ok(self.execute_jump(operand, true)?),
            Operation::LDA => Ok(self.execute_load_accumulator(operand)?),
            Operation::LDX => Ok(self.execute_load_x(operand)?),
            Operation::LDY => Ok(self.execute_load_y(operand)?),
            Operation::LSR => Ok(self.execute_right_shift(operand)?),
            Operation::NOP => Ok(()),
            Operation::ORA => Ok(self.execute_inclusive_or(operand)?),
            Operation::PHA => Ok(self.push_byte_to_stack(self.registers.accumulator)),
            Operation::PHP => Ok(self.push_byte_to_stack(self.registers.status)),
            Operation::PLA => Ok(self.execute_pull_accumulator()?), 
            Operation::PLP => Ok(self.execute_pull_status()?), 
            Operation::ROL => Ok(self.execute_rotate_left(operand)?),
            Operation::ROR => Ok(self.execute_rotate_right(operand)?),
            Operation::RTI => Ok(self.execute_return_from_interrupt()?),
            Operation::RTS => Ok(self.execute_return_from_subroutine()?),
            Operation::SBC => Ok(self.execute_substract_with_carry(operand)?),
            Operation::SEC => Ok(self.set_status_flag(StatusFlag::CARRY, true)),
            Operation::SED => Ok(self.set_status_flag(StatusFlag::DECIMAL, true)),
            Operation::SEI => Ok(self.set_status_flag(StatusFlag::INTERRUPT, true)),
            _ => Err("Unimplemented operation")
       }
    }

    fn fetch_operand(&mut self, mode : OperandMode) -> Operand {
        match mode {
            OperandMode::Absolute => self.get_absolute_operand(0),
            OperandMode::AbsoluteX => self.get_absolute_operand(self.registers.x),
            OperandMode::AbsoluteY => self.get_absolute_operand(self.registers.y),
            OperandMode::Accumulator => Operand::Accumulator,
            OperandMode::Immediate => self.get_immediate_operand(),
            OperandMode::Implied => Operand::Implied,
            OperandMode::Indirect => self.get_indirect_operand(),
            OperandMode::IndirectX => self.get_indirect_x_operand(),
            OperandMode::IndirectY => self.get_indirect_y_operand(),
            OperandMode::ZeroPage => self.get_zero_page_operand(0),
            OperandMode::ZeroPageX => self.get_zero_page_operand(self.registers.x),
            OperandMode::ZeroPageY => self.get_zero_page_operand(self.registers.y),
        }
    }

    fn get_absolute_operand(&mut self, offset: u8) -> Operand {
        let address = self.get_word_from_memory(self.registers.program_counter as usize) as u16;
        self.registers.program_counter += 2;
        Operand::Address(address + offset as u16)
    }

    fn get_immediate_operand(&mut self) -> Operand {
        let address =  self.registers.program_counter as usize;
        self.registers.program_counter += 1;
        Operand::Immediate(self.get_byte_from_memory(address))
    }

    fn get_indirect_operand(&mut self) -> Operand {
        let pointer_address = self.get_word_from_memory(self.registers.program_counter as usize) as usize;
        let pointer = self.get_word_from_memory(pointer_address) as u16;
        self.registers.program_counter += 2;
        Operand::Address(pointer)
    }

    fn get_indirect_x_operand(&mut self) -> Operand {
        let address = self.get_byte_from_memory(self.registers.program_counter as usize) as usize;
        let offset = self.registers.x as usize;
        let pointer_address = (address + offset) & 0xff;
        let pointer = self.get_word_from_memory(pointer_address);
        self.registers.program_counter += 1;
        Operand::Address(pointer)
    }

    fn get_indirect_y_operand(&mut self) -> Operand {
        let address = self.get_byte_from_memory(self.registers.program_counter as usize) as usize;
        let pointer = self.get_word_from_memory(address);
        let offset = self.registers.x as u16;
        self.registers.program_counter += 1;
        Operand::Address(pointer + offset)
    }

    fn get_zero_page_operand(&mut self, offset: u8) -> Operand {
        let address = self.get_byte_from_memory(self.registers.program_counter as usize);
        let final_address = (address + offset) & 0xff;
        self.registers.program_counter += 1;
        Operand::Address(final_address as u16)
    }

    fn execute_add_with_carry(&mut self, operand: Operand) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        let carry: u16 = self.get_status_flag(StatusFlag::CARRY) as u16;
        let accumulator: u16 = self.registers.accumulator as u16;

        let mut sum: u16 = operand_value as u16 + carry + accumulator;

        let new_carry: bool = (sum & (1 << 8)) != 0;
        self.set_status_flag(StatusFlag::CARRY, new_carry);

        sum &= 0xFF;

        let byte_positive: bool = !is_negative(operand_value);
        let accumulator_positive: bool = !is_negative(accumulator as u8);
        let sum_positive: bool = !is_negative(sum as u8);
        let overflow: bool = (byte_positive == accumulator_positive) && (sum_positive != byte_positive);
        self.set_status_flag(StatusFlag::OVERFLOW, overflow);

        self.set_zero_and_negative_flags(sum as u8);
        self.registers.accumulator = sum as u8;

        Ok(())
    }

    fn execute_and(&mut self, operand: Operand) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        let result = self.registers.accumulator & operand_value;

        self.set_zero_and_negative_flags(result);
        self.registers.accumulator = result;

        Ok(())
    }

    fn execute_left_shift(&mut self, operand: Operand) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        let high_bit = operand_value & (1 << 7) != 0;
        let result = operand_value << 1;

        self.set_status_flag(StatusFlag::CARRY, high_bit);
        self.set_zero_and_negative_flags(result);
        self.set_operand_value(operand, result)?;

        Ok(())
    }

    fn execute_branch_if(&mut self, operand: Operand, flag: StatusFlag, value: bool) -> Result<(), &'static str> {
        if self.get_status_flag(flag) == value {
            let mut operand_value = self.get_operand_value(operand)? as u16;
            if operand_value > 127 {
                // Signed eight-bit operand, decrement unsigned value with 2^8
                operand_value = operand_value.wrapping_sub(256);
            }
            // Operand is advanced by 2 from fetching opcode and operand
            self.registers.program_counter = self.registers.program_counter
                                                 .wrapping_add(operand_value.wrapping_sub(2));
        }
        Ok(())
    }

    fn execute_bit_test(&mut self, operand: Operand) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        let bit_7 = operand_value & (1 << 7) != 0;
        let bit_6 = operand_value & (1 << 6) != 0;
        let and_result = self.registers.accumulator & operand_value;

        self.set_status_flag(StatusFlag::NEGATIVE, bit_7);
        self.set_status_flag(StatusFlag::OVERFLOW, bit_6);
        self.set_status_flag(StatusFlag::ZERO, and_result == 0);
        Ok(())
    }

    fn execute_compare(&mut self, operand: Operand, register: u8) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        let substract_value = register.wrapping_sub(operand_value);

        self.set_zero_and_negative_flags(substract_value);
        self.set_status_flag(StatusFlag::CARRY, register >= operand_value);

        Ok(())
    }

    fn execute_increment(&mut self, operand: Operand, negate: bool) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        let result;
        if negate {
            result = operand_value.wrapping_sub(1);
        } else {
            result = operand_value.wrapping_add(1);
        }

        self.set_zero_and_negative_flags(result);
        self.set_operand_value(operand, result)?;

        Ok(())
    }

    fn execute_increment_x(&mut self, negate: bool) -> Result<(), &'static str> {
        let result;
        if negate {
            result = self.registers.x.wrapping_sub(1);
        } else {
            result = self.registers.x.wrapping_add(1);
        }


        self.set_zero_and_negative_flags(result);
        self.registers.x = result;

        Ok(())
    }

    fn execute_increment_y(&mut self, negate: bool) -> Result<(), &'static str> {
        let result;
        if negate {
            result = self.registers.y.wrapping_sub(1);
        } else {
            result = self.registers.y.wrapping_add(1);
        }

        self.set_zero_and_negative_flags(result);
        self.registers.y = result;

        Ok(())
    }

    fn execute_exclusive_or(&mut self, operand: Operand) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        let result = self.registers.accumulator ^ operand_value;

        self.set_zero_and_negative_flags(result);
        self.registers.accumulator = result;

        Ok(())
    }

    fn execute_jump(&mut self, operand: Operand, save_ra: bool) -> Result<(), &'static str> {
        if save_ra {
            self.push_word_to_stack(self.registers.program_counter - 1);
        }

        let jump_address = match operand {
            Operand::Address(a) => a,
            _ => return Err("Jump must have Address-type operand"),
        };
        self.registers.program_counter = jump_address;

        Ok(())
    }

    fn execute_load_accumulator(&mut self, operand: Operand) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        
        self.set_zero_and_negative_flags(operand_value);
        self.registers.accumulator = operand_value;

        Ok(())
    }

    fn execute_load_x(&mut self, operand: Operand) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        
        self.set_zero_and_negative_flags(operand_value);
        self.registers.x = operand_value;

        Ok(())
    }

    fn execute_load_y(&mut self, operand: Operand) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        
        self.set_zero_and_negative_flags(operand_value);
        self.registers.y = operand_value;

        Ok(())
    }

    fn execute_right_shift(&mut self, operand: Operand) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        let low_bit = operand_value & 1 != 0;
        let result = operand_value >> 1;

        self.set_status_flag(StatusFlag::CARRY, low_bit);
        self.set_zero_and_negative_flags(result);
        self.set_operand_value(operand, result)?;

        Ok(())
    }

    fn execute_inclusive_or(&mut self, operand: Operand) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        let result = self.registers.accumulator | operand_value;

        self.set_zero_and_negative_flags(result);
        self.registers.accumulator = result;

        Ok(())
    }

    fn execute_pull_accumulator(&mut self) -> Result<(), &'static str> {
        let new_accumulator = self.pull_byte_from_stack();

        self.set_zero_and_negative_flags(new_accumulator);
        self.registers.accumulator = new_accumulator;

        Ok(())
    }

    fn execute_pull_status(&mut self) -> Result<(), &'static str> {
        self.registers.status = self.pull_byte_from_stack();
        Ok(())
    }

    fn execute_rotate_right(&mut self, operand: Operand) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        let low_bit = operand_value & 1 != 0;
        let new_high_bit = (self.get_status_flag(StatusFlag::CARRY) as u8) << 7;
        let result = (operand_value >> 1) | new_high_bit;

        self.set_zero_and_negative_flags(result);
        self.set_status_flag(StatusFlag::CARRY, low_bit);
        self.set_operand_value(operand, result)?;

        Ok(())
    }

    fn execute_rotate_left(&mut self, operand: Operand) -> Result<(), &'static str> {
        let operand_value = self.get_operand_value(operand)?;
        let high_bit = operand_value & (1 << 7) != 0;
        let new_low_bit = self.get_status_flag(StatusFlag::CARRY) as u8;
        let result = (operand_value << 1) | new_low_bit;

        self.set_zero_and_negative_flags(result);
        self.set_status_flag(StatusFlag::CARRY, high_bit);
        self.set_operand_value(operand, result)?;

        Ok(())
    }

    fn execute_return_from_interrupt(&mut self) -> Result<(), &'static str> {
        self.registers.status = self.pull_byte_from_stack();
        self.registers.program_counter = self.pull_word_from_stack();
        Ok(())
    }

    fn execute_return_from_subroutine(&mut self) -> Result<(), &'static str> {
        self.registers.program_counter = self.pull_word_from_stack() + 1;
        Ok(())
    }

    fn execute_substract_with_carry(&mut self, operand: Operand) -> Result<(), &'static  str> {
        let operand_value = self.get_operand_value(operand)?;
        let accumulator = self.registers.accumulator;
        let carry = 1 - (self.get_status_flag(StatusFlag::CARRY) as u8);

        let (first_sum, first_overflow) = accumulator.overflowing_sub(operand_value);
        let (result, second_overflow) = first_sum.overflowing_sub(carry);

        let new_carry: bool = !first_overflow && !second_overflow;
        self.set_status_flag(StatusFlag::CARRY, new_carry);

        let byte_positive: bool = !is_negative(operand_value);
        let accumulator_positive: bool = !is_negative(accumulator as u8);
        let sum_positive: bool = !is_negative(result);
        let overflow: bool = (byte_positive != accumulator_positive) && (sum_positive == byte_positive);
        self.set_status_flag(StatusFlag::OVERFLOW, overflow);

        self.set_zero_and_negative_flags(result);
        self.registers.accumulator = result;

        Ok(())
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
            let mut state = ComputerState::initialize();
            let state_initial_registers = state.registers;

            state.execute_operation(Operation::NOP, Operand::Implied).expect("Couldn't execute NOP");

            assert_eq!(state_initial_registers, state.registers);
        }

        fn check_accumulator_op(state: &mut ComputerState, operation: Operation,
                                initial_value: Option<u8>, operand_value: Option<u8>,
                                expected_value: u8,
                                expected_flags: Vec<StatusFlag>) {
            match initial_value {
                Some(value) => state.registers.accumulator = value,
                None => (),
            };

            let operand = match  operand_value {
                Some(val) => Operand::Immediate(val),
                None => Operand::Accumulator,
            };

            state.execute_operation(operation, operand).unwrap();
            assert_eq!(state.registers.accumulator, expected_value);
            check_status_flags(&state, &expected_flags);
        }

        fn check_status_flags(state: &ComputerState, expected_flags: &Vec<StatusFlag>) {
            let all_flags = vec![StatusFlag::CARRY, StatusFlag::ZERO, StatusFlag::INTERRUPT, StatusFlag::DECIMAL,
                                 StatusFlag::BREAK, StatusFlag::RESERVED, StatusFlag::OVERFLOW, StatusFlag::NEGATIVE];
            let negative_flags = all_flags.iter().filter(|f| !expected_flags.contains(*f));

            for flag in expected_flags.iter() {
                assert!(state.get_status_flag(*flag), "{:?} should be set", flag);
            }
            for flag in negative_flags {
                assert!(!state.get_status_flag(*flag), "{:?} should not be set", flag);
            }
        }

        #[test]
        fn it_executes_adc() {
            let mut state = ComputerState::initialize();

            check_accumulator_op(&mut state, Operation::ADC, Some(33),   Some(24), 33 + 24, vec![]);
            check_accumulator_op(&mut state, Operation::ADC, None,       Some(55), 33 + 24 + 55, vec![]);
            check_accumulator_op(&mut state, Operation::ADC, None,       Some(200), 56, vec![StatusFlag::CARRY]);
            check_accumulator_op(&mut state, Operation::ADC, None,       Some(100), 157, vec![StatusFlag::NEGATIVE, StatusFlag::OVERFLOW]);
            check_accumulator_op(&mut state, Operation::ADC, None,       Some(99), 0, vec![StatusFlag::CARRY, StatusFlag::ZERO]);
            check_accumulator_op(&mut state, Operation::ADC, Some(0x80), Some(0x80), 0x01, vec![StatusFlag::CARRY, StatusFlag::OVERFLOW]);
        }

        #[test]
        fn it_executes_and() {
            let mut state = ComputerState::initialize();

            check_accumulator_op(&mut state, Operation::AND, Some(0xFF), Some(0x55), 0xff & 0x55, vec![]);
            check_accumulator_op(&mut state, Operation::AND, Some(0xaa), Some(0xf0), 0xaa & 0xf0, vec![StatusFlag::NEGATIVE]);
            check_accumulator_op(&mut state, Operation::AND, Some(0x45), Some(0x0f), 0x45 & 0x0f, vec![]);
            check_accumulator_op(&mut state, Operation::AND, Some(0x55), Some(0xa0), 0x55 & 0xa0, vec![StatusFlag::ZERO]);
            check_accumulator_op(&mut state, Operation::AND, Some(0x01), Some(0x01), 0x01 & 0x01, vec![]);
        }

        #[test]
        fn it_executes_asl() {
            let mut state = ComputerState::initialize();

            check_accumulator_op(&mut state, Operation:: ASL, Some(0xff), None, 0xfe, vec![StatusFlag::NEGATIVE, StatusFlag::CARRY]);
            check_accumulator_op(&mut state, Operation:: ASL, Some(0x55), None, 0xaa, vec![StatusFlag::NEGATIVE]);
            check_accumulator_op(&mut state, Operation:: ASL, Some(0x80), None, 0x00, vec![StatusFlag::ZERO, StatusFlag::CARRY]);
            check_accumulator_op(&mut state, Operation:: ASL, Some(0x25), None, 0x4a, vec![]);
        }

        #[test]
        fn it_branches() {
            let mut state = ComputerState::initialize();

            state.set_status_flag(StatusFlag::OVERFLOW, false);
            state.execute_operation(Operation::BVS, Operand::Immediate(0x55)).expect("Couldn't execute BVS");
            assert_eq!(state.registers.program_counter, 0x00);

            state.execute_operation(Operation::BVC, Operand::Immediate(0x55)).expect("Couldn't execute BVC");
            assert_eq!(state.registers.program_counter, 0x53);

            state.set_status_flag(StatusFlag::OVERFLOW, true);
            state.execute_operation(Operation::BVS, Operand::Immediate(0x55)).expect("Couldn't execute BVS");
            assert_eq!(state.registers.program_counter, 0xa6);

            state.set_status_flag(StatusFlag::CARRY, true);
            state.execute_operation(Operation::BCS, Operand::Immediate(0xa0)).expect("Couldn't execute BCS");
            assert_eq!(state.registers.program_counter, 0x44);

            state.set_status_flag(StatusFlag::CARRY, false);
            state.execute_operation(Operation::BCS, Operand::Immediate(0xa0)).expect("Couldn't execute BCS");
            assert_eq!(state.registers.program_counter, 0x44);

            state.execute_operation(Operation::BCC, Operand::Immediate(0xf0)).expect("Couldn't execute BCC");
            assert_eq!(state.registers.program_counter, 0x32);

            state.registers.status = 0x00;
            state.set_status_flag(StatusFlag::ZERO, true);
            state.execute_operation(Operation::BEQ, Operand::Immediate(0xf0)).expect("Couldn't execute BEQ");
            assert_eq!(state.registers.program_counter, 0x20);

            state.registers.status = 0xff;
            state.set_status_flag(StatusFlag::ZERO, false);
            state.execute_operation(Operation::BNE, Operand::Immediate(0x55)).expect("Couldn't execute BNE");
            assert_eq!(state.registers.program_counter, 0x73);

            state.registers.status = 0x00;
            state.set_status_flag(StatusFlag::NEGATIVE, true);
            state.execute_operation(Operation::BMI, Operand::Immediate(0xf0)).expect("Couldn't execute BMI");
            assert_eq!(state.registers.program_counter, 0x61);

            state.registers.status = 0xff;
            state.set_status_flag(StatusFlag::NEGATIVE, false);
            state.execute_operation(Operation::BPL, Operand::Immediate(0xf0)).expect("Couldn't execute BPL");
            assert_eq!(state.registers.program_counter, 0x4f);
        }

        #[test]
        fn it_executes_bit() {
            let mut state = ComputerState::initialize();

            check_accumulator_op(&mut state, Operation::BIT, Some(0x01), Some(0x55), 0x01,
                                 vec![StatusFlag::OVERFLOW]);
            check_accumulator_op(&mut state, Operation::BIT, Some(0x0f), Some(0xf0), 0x0f,
                                 vec![StatusFlag::NEGATIVE, StatusFlag::OVERFLOW, StatusFlag::ZERO]);
        }

        #[test]
        fn it_executes_flag_sets_and_clears() {
            let mut state = ComputerState::initialize_from_image(vec![0; 1024]);
            state.registers.status = 0xff;
            let mut expected_flags = vec![StatusFlag::ZERO, StatusFlag::BREAK, StatusFlag::RESERVED, StatusFlag::NEGATIVE,
                                          StatusFlag::CARRY, StatusFlag::DECIMAL, StatusFlag::OVERFLOW, StatusFlag::INTERRUPT];

            expected_flags.pop();
            state.execute_operation(Operation::CLI, Operand::Implied).unwrap();
            check_status_flags(&state, &expected_flags);
            expected_flags.pop();
            state.execute_operation(Operation::CLV, Operand::Implied).unwrap();
            check_status_flags(&state, &expected_flags);

            expected_flags.pop();
            state.execute_operation(Operation::CLD, Operand::Implied).unwrap();
            check_status_flags(&state, &expected_flags);

            expected_flags.pop();
            state.execute_operation(Operation::CLC, Operand::Implied).unwrap();
            check_status_flags(&state, &expected_flags);

            expected_flags.push(StatusFlag::CARRY);
            state.execute_operation(Operation::SEC, Operand::Implied).unwrap();
            check_status_flags(&state, &expected_flags);

            expected_flags.push(StatusFlag::DECIMAL);
            state.execute_operation(Operation::SED, Operand::Implied).unwrap();
            check_status_flags(&state, &expected_flags);

            expected_flags.push(StatusFlag::INTERRUPT);
            state.execute_operation(Operation::SEI, Operand::Implied).unwrap();
            check_status_flags(&state, &expected_flags)

        }

        #[test]
        fn it_executes_compares() {
            let mut state = ComputerState::initialize();

            state.registers.accumulator = 0x10;
            state.registers.x = 0xf0;
            state.registers.y = 0xff;

            state.execute_operation(Operation::CMP, Operand::Immediate(0x00)).expect("Couldn't execute CMP");
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::CARRY));
            state.execute_operation(Operation::CMP, Operand::Immediate(0x80)).expect("Couldn't execute CMP");
            assert!(state.get_status_flag(StatusFlag::NEGATIVE));
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::CARRY));


            state.execute_operation(Operation::CPX, Operand::Immediate(0x00)).expect("Couldn't execute CPX");
            assert!(state.get_status_flag(StatusFlag::NEGATIVE));
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::CARRY));
            state.execute_operation(Operation::CPX, Operand::Immediate(0x80)).expect("Couldn't execute CPX");
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::CARRY));
            state.execute_operation(Operation::CPX, Operand::Immediate(0xf0)).expect("Couldn't execute CPX");
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert!(state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::CARRY));

            state.execute_operation(Operation::CPY, Operand::Immediate(0x00)).expect("Couldn't execute CPY");
            assert!(state.get_status_flag(StatusFlag::NEGATIVE));
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::CARRY));
            state.execute_operation(Operation::CPY, Operand::Immediate(0xfe)).expect("Couldn't execute CPY");
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::CARRY));
            state.execute_operation(Operation::CPY, Operand::Immediate(0xff)).expect("Couldn't execute CPY");
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert!(state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::CARRY));
        }

        #[test]
        fn it_executes_decrement() {
            let mut state = ComputerState::initialize_from_image(vec![3]);
            state.registers.x = 4;
            state.registers.y = 2;

            state.execute_operation(Operation::DEC, Operand::Address(0)).unwrap();
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.get_byte_from_memory(0), 2);

            state.execute_operation(Operation::DEC, Operand::Address(0)).unwrap();
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.get_byte_from_memory(0), 1);

            state.execute_operation(Operation::DEC, Operand::Address(0)).unwrap();
            assert!(state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.get_byte_from_memory(0), 0);

            state.execute_operation(Operation::DEC, Operand::Address(0)).unwrap();
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.get_byte_from_memory(0), 255);

            state.execute_operation(Operation::DEX, Operand::Implied).unwrap();
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.registers.x, 3);

            state.execute_operation(Operation::DEX, Operand::Implied).unwrap();
            state.execute_operation(Operation::DEX, Operand::Implied).unwrap();
            state.execute_operation(Operation::DEX, Operand::Implied).unwrap();
            assert!(state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.registers.x, 0);

            state.execute_operation(Operation::DEX, Operand::Implied).unwrap();
            state.execute_operation(Operation::DEX, Operand::Implied).unwrap();
            state.execute_operation(Operation::DEX, Operand::Implied).unwrap();
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.registers.x, 253);

            state.execute_operation(Operation::DEY, Operand::Implied).unwrap();
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.registers.y, 1);

            state.execute_operation(Operation::DEY, Operand::Implied).unwrap();
            assert!(state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.registers.y, 0);

            state.execute_operation(Operation::DEY, Operand::Implied).unwrap();
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.registers.y, 255);
        }

        #[test]
        fn it_executes_eor() {
            let mut state = ComputerState::initialize();

            check_accumulator_op(&mut state, Operation::EOR, Some(0xff), Some(0x55), 0xaa, vec![StatusFlag::NEGATIVE]);
            check_accumulator_op(&mut state, Operation::EOR, Some(0x55), Some(0x55), 0x00, vec![StatusFlag::ZERO]);
            check_accumulator_op(&mut state, Operation::EOR, Some(0xff), Some(0xf0), 0x0f, vec![]);
            check_accumulator_op(&mut state, Operation::EOR, Some(0x0f), Some(0xf0), 0xff, vec![StatusFlag::NEGATIVE]);
            check_accumulator_op(&mut state, Operation::EOR, Some(0xaa), Some(0xa0), 0x0a, vec![]);
        }

        #[test]
        fn it_executes_increment() {
            let mut state = ComputerState::initialize_from_image(vec![254]);
            state.registers.x = 253;
            state.registers.y = 252;

            state.execute_operation(Operation::INC, Operand::Address(0)).unwrap();
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.get_byte_from_memory(0), 255);

            state.execute_operation(Operation::INC, Operand::Address(0)).unwrap();
            assert!(state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.get_byte_from_memory(0), 0);

            state.execute_operation(Operation::INC, Operand::Address(0)).unwrap();
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.get_byte_from_memory(0), 1);

            state.execute_operation(Operation::INX, Operand::Implied).unwrap();
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.registers.x, 254);

            state.execute_operation(Operation::INX, Operand::Implied).unwrap();
            state.execute_operation(Operation::INX, Operand::Implied).unwrap();
            assert!(state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.registers.x, 0);

            state.execute_operation(Operation::INX, Operand::Implied).unwrap();
            state.execute_operation(Operation::INX, Operand::Implied).unwrap();
            state.execute_operation(Operation::INX, Operand::Implied).unwrap();
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.registers.x, 3);

            state.execute_operation(Operation::INY, Operand::Implied).unwrap();
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.registers.y, 253);

            state.execute_operation(Operation::INY, Operand::Implied).unwrap();
            state.execute_operation(Operation::INY, Operand::Implied).unwrap();
            state.execute_operation(Operation::INY, Operand::Implied).unwrap();
            assert!(state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.registers.y, 0);

            state.execute_operation(Operation::INY, Operand::Implied).unwrap();
            state.execute_operation(Operation::INY, Operand::Implied).unwrap();
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert_eq!(state.registers.y, 2);
        }

        #[test]
        fn in_executes_jump() {
            let mut state = ComputerState::initialize();
            state.registers.stack_pointer = 0xff;

            state.execute_operation(Operation::JMP, Operand::Address(0x55aa)).unwrap();
            assert_eq!(state.registers.program_counter, 0x55aa);

            state.execute_operation(Operation::JSR, Operand::Address(0x7777)).unwrap();
            assert_eq!(state.registers.program_counter, 0x7777);
            assert_eq!(state.registers.stack_pointer, 0xfd);
            assert_eq!(state.pull_word_from_stack(), 0x55aa - 1);
        }

        #[test]
        fn it_executes_loads() {
            let mut state = ComputerState::initialize();

            state.execute_operation(Operation::LDA, Operand::Immediate(0x55)).unwrap();
            assert_eq!(state.registers.accumulator, 0x55);
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));

            state.execute_operation(Operation::LDX, Operand::Immediate(0x80)).unwrap();
            assert_eq!(state.registers.x, 0x80);
            assert!(!state.get_status_flag(StatusFlag::ZERO));
            assert!(state.get_status_flag(StatusFlag::NEGATIVE));

            state.execute_operation(Operation::LDY, Operand::Immediate(0x00)).unwrap();
            assert_eq!(state.registers.y, 0x00);
            assert!(state.get_status_flag(StatusFlag::ZERO));
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
        }

        #[test]
        fn it_executes_lsr() {
            let mut state = ComputerState::initialize();

            check_accumulator_op(&mut state, Operation::LSR, Some(0xff), None, 0x7f, vec![StatusFlag::CARRY]);
            check_accumulator_op(&mut state, Operation::LSR, Some(0x00), None, 0x00, vec![StatusFlag::ZERO]);
            check_accumulator_op(&mut state, Operation::LSR, Some(0x01), None, 0x00, vec![StatusFlag::ZERO, StatusFlag::CARRY]);
            check_accumulator_op(&mut state, Operation::LSR, Some(0x92), None, 0x49, vec![]);
        }

        #[test]
        fn it_executes_ora() {
            let mut state = ComputerState::initialize();

            check_accumulator_op(&mut state, Operation::ORA, Some(0x00), Some(0x00), 0x00, vec![StatusFlag::ZERO]);
            check_accumulator_op(&mut state, Operation::ORA, Some(0x01), Some(0x00), 0x01, vec![]);
            check_accumulator_op(&mut state, Operation::ORA, Some(0xf0), Some(0x0e), 0xfe, vec![StatusFlag::NEGATIVE]);
            check_accumulator_op(&mut state, Operation::ORA, Some(0x0f), Some(0xaa), 0xaf, vec![StatusFlag::NEGATIVE]);
            check_accumulator_op(&mut state, Operation::ORA, Some(0x32), Some(0x44), 0x76, vec![]);
        }

        #[test]
        fn it_executes_pushes_and_pulls() {
            let mut state = ComputerState::initialize();
            state.registers.stack_pointer = 0xff;
            
            state.registers.accumulator = 0x00;
            state.execute_operation(Operation::PHA, Operand::Implied).unwrap();
            assert_eq!(state.get_byte_from_memory(0x1ff), 0x00);
            assert_eq!(state.registers.stack_pointer, 0xfe);

            state.registers.status = 0x55;
            state.execute_operation(Operation::PHP, Operand::Implied).unwrap();
            assert_eq!(state.get_byte_from_memory(0x1fe), 0x55);
            assert_eq!(state.registers.stack_pointer, 0xfd);

            state.registers.accumulator = 0x43;
            state.execute_operation(Operation::PHA, Operand::Implied).unwrap();
            assert_eq!(state.get_byte_from_memory(0x1fd), 0x43);
            assert_eq!(state.registers.stack_pointer, 0xfc);

            state.registers.accumulator = 0x00;
            state.execute_operation(Operation::PLA, Operand::Implied).unwrap();
            assert_eq!(state.registers.accumulator, 0x43);
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert!(!state.get_status_flag(StatusFlag::ZERO));

            state.execute_operation(Operation::PLP, Operand::Implied).unwrap();
            assert_eq!(state.registers.status, 0x55);

            state.execute_operation(Operation::PLA, Operand::Implied).unwrap();
            assert_eq!(state.registers.accumulator, 0x00);
            assert!(!state.get_status_flag(StatusFlag::NEGATIVE));
            assert!(state.get_status_flag(StatusFlag::ZERO));
        }

        #[test]
        fn it_executes_rotates() {
            let mut state = ComputerState::initialize();

            check_accumulator_op(&mut state, Operation::ROL, Some(0x88), None, 0x10, vec![StatusFlag::CARRY]);
            check_accumulator_op(&mut state, Operation::ROL, Some(0x3a), None, 0x75, vec![]);
            check_accumulator_op(&mut state, Operation::ROL, Some(0x80), None, 0x00, vec![StatusFlag::ZERO, StatusFlag::CARRY]);
            check_accumulator_op(&mut state, Operation::ROL, Some(0xf0), None, 0xe1, vec![StatusFlag::NEGATIVE, StatusFlag::CARRY]);

            check_accumulator_op(&mut state, Operation::ROR, Some(0xf2), None, 0xf9, vec![StatusFlag::NEGATIVE]);
            check_accumulator_op(&mut state, Operation::ROR, Some(0x00), None, 0x00, vec![StatusFlag::ZERO]);
            check_accumulator_op(&mut state, Operation::ROR, Some(0x03), None, 0x01, vec![StatusFlag::CARRY]);
            check_accumulator_op(&mut state, Operation::ROR, Some(0x01), None, 0x80, vec![StatusFlag::NEGATIVE, StatusFlag::CARRY]);
        }

        #[test]
        fn it_executes_returns() {
            let mut state = ComputerState::initialize();

            state.push_word_to_stack(0xdead);
            state.push_byte_to_stack(0x57);
            state.push_word_to_stack(0xbeef);

            state.execute_operation(Operation::RTS, Operand::Implied).unwrap();
            assert_eq!(state.registers.program_counter, 0xbef0);

            state.execute_operation(Operation::RTI, Operand::Implied).unwrap();
            assert_eq!(state.registers.program_counter, 0xdead);
            assert_eq!(state.registers.status, 0x57);
        }

        #[test]
        fn it_executes_sbc() {
            let mut state = ComputerState::initialize();
            state.set_status_flag(StatusFlag::CARRY, true);

            check_accumulator_op(&mut state, Operation::SBC, Some(120),  Some(30), 90, vec![StatusFlag::CARRY]);
            check_accumulator_op(&mut state, Operation::SBC, None,       Some(25), 65, vec![StatusFlag::CARRY]);
            check_accumulator_op(&mut state, Operation::SBC, None,       Some(65+46), 210, vec![StatusFlag::NEGATIVE]);
            check_accumulator_op(&mut state, Operation::SBC, None,       Some(200), 9, vec![StatusFlag::CARRY]);
            check_accumulator_op(&mut state, Operation::SBC, None,       Some(9), 0, vec![StatusFlag::CARRY, StatusFlag::ZERO]);
            check_accumulator_op(&mut state, Operation::SBC, Some(10),   Some(128), 138, vec![StatusFlag::NEGATIVE, StatusFlag::OVERFLOW]);
        }

        #[test]
        fn it_sets_status_flags() {
            let mut state = ComputerState::initialize();
            state.registers.status = 0b00110011;
            state.set_status_flag(StatusFlag::ZERO, false);

            assert_eq!(state.registers.status, 0b00110001);
        }

        #[test]
        fn it_gets_status_flags() {
            let mut state = ComputerState::initialize();
            state.registers.status = 0b00110011;

            assert_eq!(state.get_status_flag(StatusFlag::ZERO), true);
            assert_eq!(state.get_status_flag(StatusFlag::NEGATIVE), false);
        }
    }
}
