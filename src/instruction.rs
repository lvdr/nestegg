pub mod operand_mode;
pub mod operation;

use operand_mode::OperandMode;
use operation::Operation;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Instruction(pub OperandMode, pub Operation);

pub struct CycleCount {
    pub cycles: u8,
    pub page_boundary_costs_extra: bool,
}

pub fn instruction_to_bytecode(instruction: Instruction) -> Result<u8, &'static str> {
    for (byte, instr) in &INSTRUCTION_TUPLES {
        if instr == &instruction {
            return Ok(*byte);
        }
    }

    // If we get to this point in the code, something horrible has happened,
    // Whatever happened, the input wasn't an instruction
    // ... or we had some weird cosmic ray hit our CPU
    // either way, this is an error
    Err("Can't find instruction")
}

pub fn bytecode_to_instruction(bytecode: u8) -> Result<Instruction, &'static str> {
    for (byte, instr) in &INSTRUCTION_TUPLES {
        if byte == &bytecode {
            return Ok(*instr);
        }
    }

    // If we get to this point in the code, something horrible has happened,
    // Whatever happened, the input wasn't an instruction
    // ... or we had some weird cosmic ray hit our CPU
    // either way, this is an error
    Err("Can't find instruction")
}

// These are from https://www.nesdev.org/wiki/6502_cycle_times
// Where values are implemented but not present, I've set the
// cycle cost as 1 (lower than the lowest possible)
// TODO: Add missing cycle values
pub fn calculate_cycles(instr: &Instruction) -> Result<CycleCount, &'static str> {
    match instr {
        Instruction(OperandMode::Immediate,   Operation::ADC) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::ADC) => Ok(cycles(3)),
        Instruction(OperandMode::ZeroPageX,   Operation::ADC) => Ok(cycles(4)),
        Instruction(OperandMode::Absolute,    Operation::ADC) => Ok(cycles(4)),
        Instruction(OperandMode::AbsoluteX,   Operation::ADC) => Ok(cycles_with_extra_cost(4)),
        Instruction(OperandMode::AbsoluteY,   Operation::ADC) => Ok(cycles_with_extra_cost(4)),
        Instruction(OperandMode::IndirectX,   Operation::ADC) => Ok(cycles(6)),
        Instruction(OperandMode::IndirectY,   Operation::ADC) => Ok(cycles_with_extra_cost(5)),

        Instruction(OperandMode::Immediate,   Operation::AND) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::AND) => Ok(cycles(3)),
        Instruction(OperandMode::ZeroPageX,   Operation::AND) => Ok(cycles(4)),
        Instruction(OperandMode::Absolute,    Operation::AND) => Ok(cycles(4)),
        Instruction(OperandMode::AbsoluteX,   Operation::AND) => Ok(cycles(4)),
        Instruction(OperandMode::AbsoluteY,   Operation::AND) => Ok(cycles(4)),
        Instruction(OperandMode::IndirectX,   Operation::AND) => Ok(cycles(6)),
        Instruction(OperandMode::IndirectY,   Operation::AND) => Ok(cycles(5)),

        Instruction(OperandMode::Accumulator, Operation::ASL) => Ok(cycles(1)),
        Instruction(OperandMode::ZeroPage,    Operation::ASL) => Ok(cycles(5)),
        Instruction(OperandMode::ZeroPageX,   Operation::ASL) => Ok(cycles(6)),
        Instruction(OperandMode::Absolute,    Operation::ASL) => Ok(cycles(6)),
        Instruction(OperandMode::AbsoluteX,   Operation::ASL) => Ok(cycles(7)),

        Instruction(OperandMode::Immediate,   Operation::BCC) => Ok(cycles(1)),
        Instruction(OperandMode::Immediate,   Operation::BCS) => Ok(cycles(1)),
        Instruction(OperandMode::Immediate,   Operation::BEQ) => Ok(cycles(1)),

        Instruction(OperandMode::ZeroPage,    Operation::BIT) => Ok(cycles(3)),
        Instruction(OperandMode::Absolute,    Operation::BIT) => Ok(cycles(4)),

        Instruction(OperandMode::Immediate,   Operation::BMI) => Ok(cycles(1)),
        Instruction(OperandMode::Immediate,   Operation::BNE) => Ok(cycles(1)),
        Instruction(OperandMode::Immediate,   Operation::BPL) => Ok(cycles(1)),
        Instruction(OperandMode::Implied,     Operation::BRK) => Ok(cycles(7)),
        Instruction(OperandMode::Immediate,   Operation::BVC) => Ok(cycles(1)),
        Instruction(OperandMode::Immediate,   Operation::BVS) => Ok(cycles(1)),
        Instruction(OperandMode::Implied,     Operation::CLC) => Ok(cycles(2)),
        Instruction(OperandMode::Implied,     Operation::CLD) => Ok(cycles(2)),
        Instruction(OperandMode::Implied,     Operation::CLI) => Ok(cycles(2)),
        Instruction(OperandMode::Implied,     Operation::CLV) => Ok(cycles(2)),

        Instruction(OperandMode::Immediate,   Operation::CMP) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::CMP) => Ok(cycles(3)),
        Instruction(OperandMode::ZeroPageX,   Operation::CMP) => Ok(cycles(4)),
        Instruction(OperandMode::Absolute,    Operation::CMP) => Ok(cycles(4)),
        Instruction(OperandMode::AbsoluteX,   Operation::CMP) => Ok(cycles_with_extra_cost(4)),
        Instruction(OperandMode::AbsoluteY,   Operation::CMP) => Ok(cycles_with_extra_cost(4)),
        Instruction(OperandMode::IndirectX,   Operation::CMP) => Ok(cycles(6)),
        Instruction(OperandMode::IndirectY,   Operation::CMP) => Ok(cycles_with_extra_cost(5)),

        Instruction(OperandMode::Immediate,   Operation::CPX) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::CPX) => Ok(cycles(3)),
        Instruction(OperandMode::Absolute,    Operation::CPX) => Ok(cycles(4)),

        Instruction(OperandMode::Immediate,   Operation::CPY) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::CPY) => Ok(cycles(3)),
        Instruction(OperandMode::Absolute,    Operation::CPY) => Ok(cycles(4)),

        Instruction(OperandMode::ZeroPage,    Operation::DEC) => Ok(cycles(5)),
        Instruction(OperandMode::ZeroPageX,   Operation::DEC) => Ok(cycles(6)),
        Instruction(OperandMode::Absolute,    Operation::DEC) => Ok(cycles(6)),
        Instruction(OperandMode::AbsoluteX,   Operation::DEC) => Ok(cycles(7)),

        Instruction(OperandMode::Implied,     Operation::DEX) => Ok(cycles(2)),
        Instruction(OperandMode::Implied,     Operation::DEY) => Ok(cycles(2)),

        Instruction(OperandMode::Immediate,   Operation::EOR) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::EOR) => Ok(cycles(3)),
        Instruction(OperandMode::ZeroPageX,   Operation::EOR) => Ok(cycles(4)),
        Instruction(OperandMode::Absolute,    Operation::EOR) => Ok(cycles(4)),
        Instruction(OperandMode::AbsoluteX,   Operation::EOR) => Ok(cycles_with_extra_cost(4)),
        Instruction(OperandMode::AbsoluteY,   Operation::EOR) => Ok(cycles_with_extra_cost(4)),
        Instruction(OperandMode::IndirectX,   Operation::EOR) => Ok(cycles(6)),
        Instruction(OperandMode::IndirectY,   Operation::EOR) => Ok(cycles_with_extra_cost(5)),

        Instruction(OperandMode::ZeroPage,    Operation::INC) => Ok(cycles(5)),
        Instruction(OperandMode::ZeroPageX,   Operation::INC) => Ok(cycles(6)),
        Instruction(OperandMode::Absolute,    Operation::INC) => Ok(cycles(6)),
        Instruction(OperandMode::AbsoluteX,   Operation::INC) => Ok(cycles(7)),

        Instruction(OperandMode::Implied,     Operation::INX) => Ok(cycles(2)),
        Instruction(OperandMode::Implied,     Operation::INY) => Ok(cycles(2)),

        Instruction(OperandMode::Absolute,    Operation::JMP) => Ok(cycles(3)),
        Instruction(OperandMode::Indirect,    Operation::JMP) => Ok(cycles(5)),

        Instruction(OperandMode::Absolute,    Operation::JSR) => Ok(cycles(6)),

        Instruction(OperandMode::Immediate,   Operation::LDA) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::LDA) => Ok(cycles(3)),
        Instruction(OperandMode::ZeroPageX,   Operation::LDA) => Ok(cycles(4)),
        Instruction(OperandMode::Absolute,    Operation::LDA) => Ok(cycles(4)),
        Instruction(OperandMode::AbsoluteX,   Operation::LDA) => Ok(cycles_with_extra_cost(4)),
        Instruction(OperandMode::AbsoluteY,   Operation::LDA) => Ok(cycles_with_extra_cost(4)),
        Instruction(OperandMode::IndirectX,   Operation::LDA) => Ok(cycles(6)),
        Instruction(OperandMode::IndirectY,   Operation::LDA) => Ok(cycles_with_extra_cost(5)),

        Instruction(OperandMode::Immediate,   Operation::LDX) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::LDX) => Ok(cycles(3)),
        Instruction(OperandMode::ZeroPageY,   Operation::LDX) => Ok(cycles(4)),
        Instruction(OperandMode::Absolute,    Operation::LDX) => Ok(cycles(4)),
        Instruction(OperandMode::AbsoluteY,   Operation::LDX) => Ok(cycles_with_extra_cost(4)),

        Instruction(OperandMode::Immediate,   Operation::LDY) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::LDY) => Ok(cycles(3)),
        Instruction(OperandMode::ZeroPageX,   Operation::LDY) => Ok(cycles(4)),
        Instruction(OperandMode::Absolute,    Operation::LDY) => Ok(cycles(4)),
        Instruction(OperandMode::AbsoluteX,   Operation::LDY) => Ok(cycles_with_extra_cost(4)),

        Instruction(OperandMode::Accumulator, Operation::LSR) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::LSR) => Ok(cycles(5)),
        Instruction(OperandMode::ZeroPageX,   Operation::LSR) => Ok(cycles(6)),
        Instruction(OperandMode::Absolute,    Operation::LSR) => Ok(cycles(6)),
        Instruction(OperandMode::AbsoluteX,   Operation::LSR) => Ok(cycles(7)),

        Instruction(OperandMode::Implied,     Operation::NOP) => Ok(cycles(2)),

        Instruction(OperandMode::Immediate,   Operation::ORA) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::ORA) => Ok(cycles(3)),
        Instruction(OperandMode::ZeroPageX,   Operation::ORA) => Ok(cycles(4)),
        Instruction(OperandMode::Absolute,    Operation::ORA) => Ok(cycles(4)),
        Instruction(OperandMode::AbsoluteX,   Operation::ORA) => Ok(cycles_with_extra_cost(4)),
        Instruction(OperandMode::AbsoluteY,   Operation::ORA) => Ok(cycles_with_extra_cost(4)),
        Instruction(OperandMode::IndirectX,   Operation::ORA) => Ok(cycles(6)),
        Instruction(OperandMode::IndirectY,   Operation::ORA) => Ok(cycles_with_extra_cost(5)),

        Instruction(OperandMode::Implied,     Operation::PHA) => Ok(cycles(1)),
        Instruction(OperandMode::Implied,     Operation::PHP) => Ok(cycles(1)),
        Instruction(OperandMode::Implied,     Operation::PLA) => Ok(cycles(1)),
        Instruction(OperandMode::Implied,     Operation::PLP) => Ok(cycles(1)),

        Instruction(OperandMode::Accumulator, Operation::ROL) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::ROL) => Ok(cycles(5)),
        Instruction(OperandMode::ZeroPageX,   Operation::ROL) => Ok(cycles(6)),
        Instruction(OperandMode::Absolute,    Operation::ROL) => Ok(cycles(6)),
        Instruction(OperandMode::AbsoluteX,   Operation::ROL) => Ok(cycles(7)),

        Instruction(OperandMode::Accumulator, Operation::ROR) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::ROR) => Ok(cycles(5)),
        Instruction(OperandMode::ZeroPageX,   Operation::ROR) => Ok(cycles(6)),
        Instruction(OperandMode::Absolute,    Operation::ROR) => Ok(cycles(6)),
        Instruction(OperandMode::AbsoluteX,   Operation::ROR) => Ok(cycles(7)),

        Instruction(OperandMode::Implied,     Operation::RTI) => Ok(cycles(6)),
        Instruction(OperandMode::Implied,     Operation::RTS) => Ok(cycles(6)),

        Instruction(OperandMode::Immediate,   Operation::SBC) => Ok(cycles(2)),
        Instruction(OperandMode::ZeroPage,    Operation::SBC) => Ok(cycles(3)),
        Instruction(OperandMode::ZeroPageX,   Operation::SBC) => Ok(cycles(4)),
        Instruction(OperandMode::Absolute,    Operation::SBC) => Ok(cycles(4)),
        Instruction(OperandMode::AbsoluteX,   Operation::SBC) => Ok(cycles_with_extra_cost(4)),
        Instruction(OperandMode::AbsoluteY,   Operation::SBC) => Ok(cycles_with_extra_cost(4)),
        Instruction(OperandMode::IndirectX,   Operation::SBC) => Ok(cycles(6)),
        Instruction(OperandMode::IndirectY,   Operation::SBC) => Ok(cycles_with_extra_cost(5)),

        Instruction(OperandMode::Implied,     Operation::SEC) => Ok(cycles(2)),
        Instruction(OperandMode::Implied,     Operation::SED) => Ok(cycles(2)),
        Instruction(OperandMode::Implied,     Operation::SEI) => Ok(cycles(2)),

        Instruction(OperandMode::ZeroPage,    Operation::STA) => Ok(cycles(3)),
        Instruction(OperandMode::ZeroPageX,   Operation::STA) => Ok(cycles(4)),
        Instruction(OperandMode::Absolute,    Operation::STA) => Ok(cycles(4)),
        Instruction(OperandMode::AbsoluteX,   Operation::STA) => Ok(cycles(5)),
        Instruction(OperandMode::AbsoluteY,   Operation::STA) => Ok(cycles(5)),
        Instruction(OperandMode::IndirectX,   Operation::STA) => Ok(cycles(6)),
        Instruction(OperandMode::IndirectY,   Operation::STA) => Ok(cycles(6)),

        Instruction(OperandMode::ZeroPage,    Operation::STX) => Ok(cycles(3)),
        Instruction(OperandMode::ZeroPageY,   Operation::STX) => Ok(cycles(4)),
        Instruction(OperandMode::Absolute,    Operation::STX) => Ok(cycles(4)),

        Instruction(OperandMode::ZeroPage,    Operation::STY) => Ok(cycles(3)),
        Instruction(OperandMode::ZeroPageX,   Operation::STY) => Ok(cycles(4)),
        Instruction(OperandMode::Absolute,    Operation::STY) => Ok(cycles(4)),

        Instruction(OperandMode::Implied,     Operation::TAX) => Ok(cycles(2)),
        Instruction(OperandMode::Implied,     Operation::TAY) => Ok(cycles(2)),
        Instruction(OperandMode::Implied,     Operation::TSX) => Ok(cycles(2)),
        Instruction(OperandMode::Implied,     Operation::TXA) => Ok(cycles(2)),
        Instruction(OperandMode::Implied,     Operation::TXS) => Ok(cycles(2)),
        Instruction(OperandMode::Implied,     Operation::TYA) => Ok(cycles(2)),
        _                                                     => Err("Instruction timings not found")
    }
}

fn cycles(cycles: u8) -> CycleCount {
    CycleCount { cycles, page_boundary_costs_extra: false }
}

fn cycles_with_extra_cost(cycles: u8) -> CycleCount {
    CycleCount { cycles, page_boundary_costs_extra: true }
}

const NUM_TUPLES: usize = 151;
static INSTRUCTION_TUPLES: [(u8, Instruction); NUM_TUPLES] = [
    (0x69, Instruction(OperandMode::Immediate,   Operation::ADC)),
    (0x65, Instruction(OperandMode::ZeroPage,    Operation::ADC)),
    (0x75, Instruction(OperandMode::ZeroPageX,   Operation::ADC)),
    (0x6D, Instruction(OperandMode::Absolute,    Operation::ADC)),
    (0x7D, Instruction(OperandMode::AbsoluteX,   Operation::ADC)),
    (0x79, Instruction(OperandMode::AbsoluteY,   Operation::ADC)),
    (0x61, Instruction(OperandMode::IndirectX,   Operation::ADC)),
    (0x71, Instruction(OperandMode::IndirectY,   Operation::ADC)),
    (0x29, Instruction(OperandMode::Immediate,   Operation::AND)),
    (0x25, Instruction(OperandMode::ZeroPage,    Operation::AND)),
    (0x35, Instruction(OperandMode::ZeroPageX,   Operation::AND)),
    (0x2D, Instruction(OperandMode::Absolute,    Operation::AND)),
    (0x3D, Instruction(OperandMode::AbsoluteX,   Operation::AND)),
    (0x39, Instruction(OperandMode::AbsoluteY,   Operation::AND)),
    (0x21, Instruction(OperandMode::IndirectX,   Operation::AND)),
    (0x31, Instruction(OperandMode::IndirectY,   Operation::AND)),
    (0x0A, Instruction(OperandMode::Accumulator, Operation::ASL)),
    (0x06, Instruction(OperandMode::ZeroPage,    Operation::ASL)),
    (0x16, Instruction(OperandMode::ZeroPageX,   Operation::ASL)),
    (0x0E, Instruction(OperandMode::Absolute,    Operation::ASL)),
    (0x1E, Instruction(OperandMode::AbsoluteX,   Operation::ASL)),
    (0x90, Instruction(OperandMode::Immediate,   Operation::BCC)),
    (0xB0, Instruction(OperandMode::Immediate,   Operation::BCS)),
    (0xF0, Instruction(OperandMode::Immediate,   Operation::BEQ)),
    (0x24, Instruction(OperandMode::ZeroPage,    Operation::BIT)),
    (0x2C, Instruction(OperandMode::Absolute,    Operation::BIT)),
    (0x30, Instruction(OperandMode::Immediate,   Operation::BMI)),
    (0xD0, Instruction(OperandMode::Immediate,   Operation::BNE)),
    (0x10, Instruction(OperandMode::Immediate,   Operation::BPL)),
    (0x00, Instruction(OperandMode::Implied,     Operation::BRK)),
    (0x50, Instruction(OperandMode::Immediate,   Operation::BVC)),
    (0x70, Instruction(OperandMode::Immediate,   Operation::BVS)),
    (0x18, Instruction(OperandMode::Implied,     Operation::CLC)),
    (0xD8, Instruction(OperandMode::Implied,     Operation::CLD)),
    (0x58, Instruction(OperandMode::Implied,     Operation::CLI)),
    (0xB8, Instruction(OperandMode::Implied,     Operation::CLV)),
    (0xC9, Instruction(OperandMode::Immediate,   Operation::CMP)),
    (0xC5, Instruction(OperandMode::ZeroPage,    Operation::CMP)),
    (0xD5, Instruction(OperandMode::ZeroPageX,   Operation::CMP)),
    (0xCD, Instruction(OperandMode::Absolute,    Operation::CMP)),
    (0xDD, Instruction(OperandMode::AbsoluteX,   Operation::CMP)),
    (0xD9, Instruction(OperandMode::AbsoluteY,   Operation::CMP)),
    (0xC1, Instruction(OperandMode::IndirectX,   Operation::CMP)),
    (0xD1, Instruction(OperandMode::IndirectY,   Operation::CMP)),
    (0xE0, Instruction(OperandMode::Immediate,   Operation::CPX)),
    (0xE4, Instruction(OperandMode::ZeroPage,    Operation::CPX)),
    (0xEC, Instruction(OperandMode::Absolute,    Operation::CPX)),
    (0xC0, Instruction(OperandMode::Immediate,   Operation::CPY)),
    (0xC4, Instruction(OperandMode::ZeroPage,    Operation::CPY)),
    (0xCC, Instruction(OperandMode::Absolute,    Operation::CPY)),
    (0xC6, Instruction(OperandMode::ZeroPage,    Operation::DEC)),
    (0xD6, Instruction(OperandMode::ZeroPageX,   Operation::DEC)),
    (0xCE, Instruction(OperandMode::Absolute,    Operation::DEC)),
    (0xDE, Instruction(OperandMode::AbsoluteX,   Operation::DEC)),
    (0xCA, Instruction(OperandMode::Implied,     Operation::DEX)),
    (0x88, Instruction(OperandMode::Implied,     Operation::DEY)),
    (0x49, Instruction(OperandMode::Immediate,   Operation::EOR)),
    (0x45, Instruction(OperandMode::ZeroPage,    Operation::EOR)),
    (0x55, Instruction(OperandMode::ZeroPageX,   Operation::EOR)),
    (0x4D, Instruction(OperandMode::Absolute,    Operation::EOR)),
    (0x5D, Instruction(OperandMode::AbsoluteX,   Operation::EOR)),
    (0x59, Instruction(OperandMode::AbsoluteY,   Operation::EOR)),
    (0x41, Instruction(OperandMode::IndirectX,   Operation::EOR)),
    (0x51, Instruction(OperandMode::IndirectY,   Operation::EOR)),
    (0xE6, Instruction(OperandMode::ZeroPage,    Operation::INC)),
    (0xF6, Instruction(OperandMode::ZeroPageX,   Operation::INC)),
    (0xEE, Instruction(OperandMode::Absolute,    Operation::INC)),
    (0xFE, Instruction(OperandMode::AbsoluteX,   Operation::INC)),
    (0xE8, Instruction(OperandMode::Implied,     Operation::INX)),
    (0xC8, Instruction(OperandMode::Implied,     Operation::INY)),
    (0x4C, Instruction(OperandMode::Absolute,    Operation::JMP)),
    (0x6C, Instruction(OperandMode::Indirect,    Operation::JMP)),
    (0x20, Instruction(OperandMode::Absolute,    Operation::JSR)),
    (0xA9, Instruction(OperandMode::Immediate,   Operation::LDA)),
    (0xA5, Instruction(OperandMode::ZeroPage,    Operation::LDA)),
    (0xB5, Instruction(OperandMode::ZeroPageX,   Operation::LDA)),
    (0xAD, Instruction(OperandMode::Absolute,    Operation::LDA)),
    (0xBD, Instruction(OperandMode::AbsoluteX,   Operation::LDA)),
    (0xB9, Instruction(OperandMode::AbsoluteY,   Operation::LDA)),
    (0xA1, Instruction(OperandMode::IndirectX,   Operation::LDA)),
    (0xB1, Instruction(OperandMode::IndirectY,   Operation::LDA)),
    (0xA2, Instruction(OperandMode::Immediate,   Operation::LDX)),
    (0xA6, Instruction(OperandMode::ZeroPage,    Operation::LDX)),
    (0xB6, Instruction(OperandMode::ZeroPageY,   Operation::LDX)),
    (0xAE, Instruction(OperandMode::Absolute,    Operation::LDX)),
    (0xBE, Instruction(OperandMode::AbsoluteY,   Operation::LDX)),
    (0xA0, Instruction(OperandMode::Immediate,   Operation::LDY)),
    (0xA4, Instruction(OperandMode::ZeroPage,    Operation::LDY)),
    (0xB4, Instruction(OperandMode::ZeroPageX,   Operation::LDY)),
    (0xAC, Instruction(OperandMode::Absolute,    Operation::LDY)),
    (0xBC, Instruction(OperandMode::AbsoluteX,   Operation::LDY)),
    (0x4A, Instruction(OperandMode::Accumulator, Operation::LSR)),
    (0x46, Instruction(OperandMode::ZeroPage,    Operation::LSR)),
    (0x56, Instruction(OperandMode::ZeroPageX,   Operation::LSR)),
    (0x4E, Instruction(OperandMode::Absolute,    Operation::LSR)),
    (0x5E, Instruction(OperandMode::AbsoluteX,   Operation::LSR)),
    (0xEA, Instruction(OperandMode::Implied,     Operation::NOP)),
    (0x09, Instruction(OperandMode::Immediate,   Operation::ORA)),
    (0x05, Instruction(OperandMode::ZeroPage,    Operation::ORA)),
    (0x15, Instruction(OperandMode::ZeroPageX,   Operation::ORA)),
    (0x0D, Instruction(OperandMode::Absolute,    Operation::ORA)),
    (0x1D, Instruction(OperandMode::AbsoluteX,   Operation::ORA)),
    (0x19, Instruction(OperandMode::AbsoluteY,   Operation::ORA)),
    (0x01, Instruction(OperandMode::IndirectX,   Operation::ORA)),
    (0x11, Instruction(OperandMode::IndirectY,   Operation::ORA)),
    (0x48, Instruction(OperandMode::Implied,     Operation::PHA)),
    (0x08, Instruction(OperandMode::Implied,     Operation::PHP)),
    (0x68, Instruction(OperandMode::Implied,     Operation::PLA)),
    (0x28, Instruction(OperandMode::Implied,     Operation::PLP)),
    (0x2A, Instruction(OperandMode::Accumulator, Operation::ROL)),
    (0x26, Instruction(OperandMode::ZeroPage,    Operation::ROL)),
    (0x36, Instruction(OperandMode::ZeroPageX,   Operation::ROL)),
    (0x2E, Instruction(OperandMode::Absolute,    Operation::ROL)),
    (0x3E, Instruction(OperandMode::AbsoluteX,   Operation::ROL)),
    (0x6A, Instruction(OperandMode::Accumulator, Operation::ROR)),
    (0x66, Instruction(OperandMode::ZeroPage,    Operation::ROR)),
    (0x76, Instruction(OperandMode::ZeroPageX,   Operation::ROR)),
    (0x6E, Instruction(OperandMode::Absolute,    Operation::ROR)),
    (0x7E, Instruction(OperandMode::AbsoluteX,   Operation::ROR)),
    (0x40, Instruction(OperandMode::Implied,     Operation::RTI)),
    (0x60, Instruction(OperandMode::Implied,     Operation::RTS)),
    (0xE9, Instruction(OperandMode::Immediate,   Operation::SBC)),
    (0xE5, Instruction(OperandMode::ZeroPage,    Operation::SBC)),
    (0xF5, Instruction(OperandMode::ZeroPageX,   Operation::SBC)),
    (0xED, Instruction(OperandMode::Absolute,    Operation::SBC)),
    (0xFD, Instruction(OperandMode::AbsoluteX,   Operation::SBC)),
    (0xF9, Instruction(OperandMode::AbsoluteY,   Operation::SBC)),
    (0xE1, Instruction(OperandMode::IndirectX,   Operation::SBC)),
    (0xF1, Instruction(OperandMode::IndirectY,   Operation::SBC)),
    (0x38, Instruction(OperandMode::Implied,     Operation::SEC)),
    (0xF8, Instruction(OperandMode::Implied,     Operation::SED)),
    (0x78, Instruction(OperandMode::Implied,     Operation::SEI)),
    (0x85, Instruction(OperandMode::ZeroPage,    Operation::STA)),
    (0x95, Instruction(OperandMode::ZeroPageX,   Operation::STA)),
    (0x8D, Instruction(OperandMode::Absolute,    Operation::STA)),
    (0x9D, Instruction(OperandMode::AbsoluteX,   Operation::STA)),
    (0x99, Instruction(OperandMode::AbsoluteY,   Operation::STA)),
    (0x81, Instruction(OperandMode::IndirectX,   Operation::STA)),
    (0x91, Instruction(OperandMode::IndirectY,   Operation::STA)),
    (0x86, Instruction(OperandMode::ZeroPage,    Operation::STX)),
    (0x96, Instruction(OperandMode::ZeroPageY,   Operation::STX)),
    (0x8E, Instruction(OperandMode::Absolute,    Operation::STX)),
    (0x84, Instruction(OperandMode::ZeroPage,    Operation::STY)),
    (0x94, Instruction(OperandMode::ZeroPageX,   Operation::STY)),
    (0x8C, Instruction(OperandMode::Absolute,    Operation::STY)),
    (0xAA, Instruction(OperandMode::Implied,     Operation::TAX)),
    (0xA8, Instruction(OperandMode::Implied,     Operation::TAY)),
    (0xBA, Instruction(OperandMode::Implied,     Operation::TSX)),
    (0x8A, Instruction(OperandMode::Implied,     Operation::TXA)),
    (0x9A, Instruction(OperandMode::Implied,     Operation::TXS)),
    (0x98, Instruction(OperandMode::Implied,     Operation::TYA)),
];

#[cfg(test)]
mod unit_tests {
    use super::*;

    mod describe_bytecode_to_instruction {
        use super::*;

        #[test]
        fn it_errors_when_instruction_not_found() {
            assert!(bytecode_to_instruction(0x80).is_err());
        }

        #[test]
        fn it_works() {
            assert_eq!(bytecode_to_instruction(0x98), Ok(Instruction(OperandMode::Implied, Operation::TYA)));
        }
    }

    mod describe_instruction_to_bytecode {
        use super::*;

        #[test]
        fn it_works() {
            assert_eq!(instruction_to_bytecode(Instruction(OperandMode::Implied, Operation::TYA)), Ok(0x98));
        }
    }
}

