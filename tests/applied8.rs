use universal_turing_machine::{
    transition::{Table, Transition},
    Direction, TuringMachine,
};

#[derive(Copy, Clone, PartialEq, Debug)]
enum Language {
    A,
    B,
    Empty,
}

impl universal_turing_machine::language::Language for Language {
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

    let mut transition_table: Table<Language> = Table::new();

    transition_table.add_transitions(&transitions);

    TuringMachine::new(transition_table)
}

fn get_expected_steps(input_length: usize) -> usize {
    2 * input_length + 1
}

fn check_result(input: &[Language], expected: &[Language]) {
    let mut tm = generate_tm();
    tm.write_to_tape_slice(input);
    let result = tm.run();
    match result {
        Ok((answer, steps)) => {
            assert_eq!(answer, expected);
            assert_eq!(steps, get_expected_steps(input.len()));
        }
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_empty() {
    // This will currently fail due to tape length logic.
    // Will be fixed soon.
    check_result(&[], &[Language::Empty]);
}

#[test]
fn test_a() {
    check_result(&[Language::A], &[Language::A]);
}

#[test]
fn test_b() {
    check_result(&[Language::B], &[Language::B]);
}

#[test]
fn test_ab() {
    check_result(&[Language::A, Language::B], &[Language::B, Language::A]);
}

#[test]
fn test_baa() {
    check_result(
        &[Language::B, Language::A, Language::A],
        &[Language::A, Language::A, Language::B],
    );
}

#[test]
fn test_abba() {
    check_result(
        &[Language::A, Language::B, Language::B, Language::A],
        &[Language::A, Language::B, Language::B, Language::A],
    );
}

#[test]
fn test_babbb() {
    check_result(
        &[
            Language::B,
            Language::A,
            Language::B,
            Language::B,
            Language::B,
        ],
        &[
            Language::B,
            Language::A,
            Language::B,
            Language::B,
            Language::B,
        ],
    );
}
