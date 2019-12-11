use std::vec::Vec;

mod instruction;

use instruction::{Instruction, decode_instruction};
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

    pub fn fetch_operand(&self, mode : OperandMode) -> (u16, u8) {
        let pc = self.registers.program_counter as usize;
        match mode {
            OperandMode::Absolute => {
                let address = self.get_word_from_memory(pc) as usize;
                let operand = self.get_byte_from_memory(address);
                (2, operand)
            },
            OperandMode::AbsoluteX => {
                let address = self.get_word_from_memory(pc) as usize;
                let offset = self.registers.x as usize;
                let operand = self.get_byte_from_memory(address + offset);
                (2, operand)
            },
            OperandMode::AbsoluteY => {
                let address = self.get_word_from_memory(pc) as usize;
                let offset = self.registers.y as usize;
                let operand = self.get_byte_from_memory(address + offset);
                (2, operand)
            },
            OperandMode::Accumulator => (0, self.registers.accumulator),
            OperandMode::Immediate => (2, self.get_byte_from_memory(pc)),
            OperandMode::Implied => (0, 0),
            OperandMode::Indirect => {
                let pointer_address = self.get_word_from_memory(pc) as usize;
                let pointer = self.get_word_from_memory(pointer_address) as usize;
                let operand = self.get_byte_from_memory(pointer);
                (2, operand)
            },
            OperandMode::IndirectX => {
                let address = self.get_byte_from_memory(pc) as usize;
                let offset = self.registers.x as usize;
                let pointer_address = (address + offset) & 0xff;
                let pointer = self.get_word_from_memory(pointer_address) as usize;
                let operand = self.get_byte_from_memory(pointer);
                (1, operand)
            },
            OperandMode::IndirectY => {
                let address = self.get_byte_from_memory(pc) as usize;
                let pointer = self.get_word_from_memory(address) as usize;
                let offset = self.registers.x as usize;
                let operand = self.get_byte_from_memory(pointer + offset);
                (1, operand)
            },
            OperandMode::ZeroPage => {
                let address = self.get_byte_from_memory(pc) as usize;
                let operand = self.get_byte_from_memory(address);
                (1, operand)
            },
            OperandMode::ZeroPageX => {
                let address = self.get_byte_from_memory(pc) as usize;
                let offset = self.registers.x as usize;
                let final_address = (address + offset) & 0xff;
                let operand = self.get_byte_from_memory(final_address);
                (1, operand)
            },
            OperandMode::ZeroPageY => {
                let address = self.get_byte_from_memory(pc) as usize;
                let offset = self.registers.y as usize;
                let final_address = (address + offset) & 0xff;
                let operand = self.get_byte_from_memory(final_address);
                (1, operand)
            },
       }
    }

    pub fn step(mut self) -> Self {
        let instruction = self.memory[self.registers.program_counter as usize];
        self.registers.program_counter += 1;
        let decoded_instruction = decode_instruction(instruction).unwrap();

        let (bytes, operand) = self.fetch_operand(decoded_instruction.0);
        self.registers.program_counter += bytes;
        // execute instruction--> executes the instruction, changing state accordingly
        // increment program counter

        self
    }

    pub fn multiple_steps(mut self, steps: u32) -> Self {
        (0..steps).fold(self, |state, _| state.step())
    }
}
