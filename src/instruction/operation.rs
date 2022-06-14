use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Operation {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI,
    BNE, BPL, BRK, BVC, BVS, CLC, CLD, CLI,
    CLV, CMP, CPX, CPY, DEC, DEX, DEY, EOR,
    INC, INX, INY, JMP, JSR, LDA, LDX, LDY,
    LSR, NOP, ORA, PHA, PHP, PLA, PLP, ROL,
    ROR, RTI, RTS, SBC, SEC, SED, SEI, STA,
    STX, STY, TAX, TAY, TSX, TXA, TXS, TYA
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADC" => Ok(Operation::ADC),
            "AND" => Ok(Operation::AND),
            "ASL" => Ok(Operation::ASL),
            "BCC" => Ok(Operation::BCC),
            "BCS" => Ok(Operation::BCS),
            "BEQ" => Ok(Operation::BEQ),
            "BIT" => Ok(Operation::BIT),
            "BMI" => Ok(Operation::BMI),
            "BNE" => Ok(Operation::BNE),
            "BPL" => Ok(Operation::BPL),
            "BRK" => Ok(Operation::BRK),
            "BVC" => Ok(Operation::BVC),
            "BVS" => Ok(Operation::BVS),
            "CLC" => Ok(Operation::CLC),
            "CLD" => Ok(Operation::CLD),
            "CLI" => Ok(Operation::CLI),
            "CLV" => Ok(Operation::CLV),
            "CMP" => Ok(Operation::CMP),
            "CPX" => Ok(Operation::CPX),
            "CPY" => Ok(Operation::CPY),
            "DEC" => Ok(Operation::DEC),
            "DEX" => Ok(Operation::DEX),
            "DEY" => Ok(Operation::DEY),
            "EOR" => Ok(Operation::EOR),
            "INC" => Ok(Operation::INC),
            "INX" => Ok(Operation::INX),
            "INY" => Ok(Operation::INY),
            "JMP" => Ok(Operation::JMP),
            "JSR" => Ok(Operation::JSR),
            "LDA" => Ok(Operation::LDA),
            "LDX" => Ok(Operation::LDX),
            "LDY" => Ok(Operation::LDY),
            "LSR" => Ok(Operation::LSR),
            "NOP" => Ok(Operation::NOP),
            "ORA" => Ok(Operation::ORA),
            "PHA" => Ok(Operation::PHA),
            "PHP" => Ok(Operation::PHP),
            "PLA" => Ok(Operation::PLA),
            "PLP" => Ok(Operation::PLP),
            "ROL" => Ok(Operation::ROL),
            "ROR" => Ok(Operation::ROR),
            "RTI" => Ok(Operation::RTI),
            "RTS" => Ok(Operation::RTS),
            "SBC" => Ok(Operation::SBC),
            "SEC" => Ok(Operation::SEC),
            "SED" => Ok(Operation::SED),
            "SEI" => Ok(Operation::SEI),
            "STA" => Ok(Operation::STA),
            "STX" => Ok(Operation::STX),
            "STY" => Ok(Operation::STY),
            "TAX" => Ok(Operation::TAX),
            "TAY" => Ok(Operation::TAY),
            "TSX" => Ok(Operation::TSX),
            "TXA" => Ok(Operation::TXA),
            "TXS" => Ok(Operation::TXS),
            "TYA" => Ok(Operation::TYA),
            _     => Err("Couldn't find matching instruction")
        }
    }
}
