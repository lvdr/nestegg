use nestegg::ComputerState;

#[test]
fn smoketest() {
    let initial_memory = vec![0xEA; 10];
    let expected_memory = initial_memory.clone();

    let final_memory = ComputerState::initialize_from_image(initial_memory)
        .multiple_steps(10)
        .unwrap()
    .memory;

    assert_eq!(final_memory, expected_memory);
}

#[test]
fn fibonacci_test() {
    let mut padding = vec![0; 512];
    let mut program = vec![
        0x18,             // CLC       - clear carry, otherwise first value may be off by 1
        0xA2, 0x00,       // LDX $0    - set starting values: 0 to X,
        0xA0, 0x01,       // LDY $1    -                      1 to Y
        // .loop
        0x98,             // TYA       - move Y to A
        0x86, 0x20,       // STX tmp   - can't add X directly, so store to memory
        0x65, 0x20,       // ADC tmp   - and add to accumulator
        0xB0, 0x08,       // BCS .end  - if carry set (result > 255), branch to .end
        0x48,             // PHA       - push result to stack
        0xA4, 0x20,       // LDY tmp   - load previously stored X value to Y
        0xAA,             // TAX       - and move A to X
        0x90, 0xf5,       // BCC .loop - jump back to start of .loop
        //     .end
        0x70, 0x00,       // BCS (0) - jump back to this address, stalling in infloop
        // .data
        0x00];            // tmp
    program.append(&mut padding); // Space for stack

    let final_memory = ComputerState::initialize_from_image(program)
        .multiple_steps(600)
        .unwrap()
    .memory;

    let expected = vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];
    for i in 0..12 {
        assert_eq!(expected[i], final_memory[0x1ff - i]);
    }
}
