use nestegg::ComputerState;
use std::vec::Vec;

fn memory_test(initial_memory: Vec<u8>, expected_memory: &Vec<u8>,
               steps: u32) -> bool {
    let mut state = ComputerState::initialize_from_image(initial_memory);
    state.step(steps);
    state.memory.iter().zip(expected_memory).all(|(a, b)| a == b)
}

#[test]
fn initial_smoketest() {
    let initial_memory : Vec<u8> = vec![0xEA; 10];
    let expected_memory = initial_memory.clone();
    assert!(memory_test(initial_memory, &expected_memory, 10));
}
