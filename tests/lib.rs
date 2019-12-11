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
