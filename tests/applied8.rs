use universal_turing_machine::{Direction, Transition, TransitionTable, TuringMachine};

#[derive(Copy, Clone, PartialEq, Debug)]
enum Language {
    A,
    B,
    Empty,
}

impl universal_turing_machine::Language for Language {
    fn empty() -> Self {
        Self::Empty
    }
}

fn generate_tm() -> TuringMachine<Language> {
    let transitions = [
        Transition::new(0, 2, Language::A, Language::Empty, Direction::Right),
        Transition::new(0, 4, Language::B, Language::Empty, Direction::Right),
        Transition::new(0, 1, Language::Empty, Language::Empty, Direction::Right),
        // First value is an a
        Transition::new(2, 2, Language::A, Language::A, Direction::Right),
        Transition::new(2, 2, Language::B, Language::B, Direction::Right),
        Transition::new(2, 3, Language::Empty, Language::Empty, Direction::Left),
        Transition::new(3, 6, Language::A, Language::A, Direction::Left),
        Transition::new(3, 7, Language::B, Language::A, Direction::Left),
        Transition::new(3, 1, Language::Empty, Language::A, Direction::Right),
        // First value is a b
        Transition::new(4, 4, Language::A, Language::A, Direction::Right),
        Transition::new(4, 4, Language::B, Language::B, Direction::Right),
        Transition::new(4, 5, Language::Empty, Language::Empty, Direction::Left),
        Transition::new(5, 6, Language::A, Language::B, Direction::Left),
        Transition::new(5, 7, Language::B, Language::B, Direction::Left),
        Transition::new(5, 1, Language::Empty, Language::B, Direction::Right),
        // Last value is an a
        Transition::new(6, 6, Language::A, Language::A, Direction::Left),
        Transition::new(6, 6, Language::B, Language::B, Direction::Left),
        Transition::new(6, 1, Language::Empty, Language::A, Direction::Right),
        // Last value is a b
        Transition::new(7, 7, Language::A, Language::A, Direction::Left),
        Transition::new(7, 7, Language::B, Language::B, Direction::Left),
        Transition::new(7, 1, Language::Empty, Language::B, Direction::Right),
    ];

    let mut transition_table: TransitionTable<Language> = TransitionTable::new();

    transition_table.add_transitions(&transitions);

    TuringMachine::new(transition_table)
}

fn get_expected_steps(input_length: usize) -> usize {
    2 * input_length + 1
}

#[test]
fn test_empty() {
    let mut tm = generate_tm();
    let input = [Language::Empty];
    tm.write_to_tape_slice(&input);
    let result = tm.run();

    match result {
        Ok((answer, steps)) => {
            assert_eq!(answer, &[Language::Empty]);
            assert_eq!(steps, get_expected_steps(0));
        }
        Err(e) => panic!("Error running machine: {e:?}"),
    }
}

#[test]
fn test_a() {
    let mut tm = generate_tm();
    let input = [Language::A];
    tm.write_to_tape_slice(&input);
    let result = tm.run();

    match result {
        Ok((answer, steps)) => {
            assert_eq!(answer, &[Language::A]);
            assert_eq!(steps, get_expected_steps(input.len()));
        }
        Err(e) => panic!("Error running machine: {e:?}"),
    }
}

#[test]
fn test_b() {
    let mut tm = generate_tm();
    let input = [Language::B];
    tm.write_to_tape_slice(&input);
    let result = tm.run();

    match result {
        Ok((answer, steps)) => {
            assert_eq!(answer, &[Language::B]);
            assert_eq!(steps, get_expected_steps(input.len()));
        }
        Err(e) => panic!("Error running machine: {e:?}"),
    }
}

#[test]
fn test_ab() {
    let mut tm = generate_tm();
    let input = [Language::A, Language::B];
    tm.write_to_tape_slice(&input);
    let result = tm.run();

    match result {
        Ok((answer, steps)) => {
            assert_eq!(answer, &[Language::B, Language::A]);
            assert_eq!(steps, get_expected_steps(input.len()));
        }
        Err(e) => panic!("Error running machine: {e:?}"),
    }
}

#[test]
fn test_baa() {
    let mut tm = generate_tm();
    let input = [Language::B, Language::A, Language::A];
    tm.write_to_tape_slice(&input);
    let result = tm.run();

    match result {
        Ok((answer, steps)) => {
            assert_eq!(answer, &[Language::A, Language::A, Language::B]);
            assert_eq!(steps, get_expected_steps(input.len()));
        }
        Err(e) => panic!("Error running machine: {e:?}"),
    }
}

#[test]
fn test_abba() {
    let mut tm = generate_tm();
    let input = [Language::A, Language::B, Language::B, Language::A];
    tm.write_to_tape_slice(&input);
    let result = tm.run();

    match result {
        Ok((answer, steps)) => {
            assert_eq!(
                answer,
                &[Language::A, Language::B, Language::B, Language::A]
            );
            assert_eq!(steps, get_expected_steps(input.len()));
        }
        Err(e) => panic!("Error running machine: {e:?}"),
    }
}

#[test]
fn test_babbb() {
    let mut tm = generate_tm();
    let input = [
        Language::B,
        Language::A,
        Language::B,
        Language::B,
        Language::B,
    ];
    tm.write_to_tape_slice(&input);
    let result = tm.run();

    match result {
        Ok((answer, steps)) => {
            assert_eq!(
                answer,
                &[
                    Language::B,
                    Language::A,
                    Language::B,
                    Language::B,
                    Language::B,
                ]
            );
            assert_eq!(steps, get_expected_steps(input.len()));
        }
        Err(e) => panic!("Error running machine: {e:?}"),
    }
}
