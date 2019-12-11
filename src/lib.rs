use std::vec::Vec;

mod instruction;

use instruction::{Instruction, decode_instruction};

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

    pub fn step(mut self) -> Self {
        let instruction = self.memory[self.registers.program_counter as usize];

        let decoded_instruction = decode_instruction(instruction);

        // get operand        --> something that returns the operand while changing
        //                        the state of the computer
        // execute instruction--> executes the instruction, changing state accordingly
        // increment program counter

        self
    }

    pub fn multiple_steps(mut self, steps: u32) -> Self {
        (0..steps).fold(self, |state, _| state.step())
    }
}
