use universal_turing_machine::{
    tape::{Direction, Tape},
    transition::{Table, Transition},
    turing_machine::TuringMachine,
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

fn generate_table() -> Table<Language> {
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
    transition_table
}

fn get_expected_steps(input_length: usize) -> usize {
    2 * input_length + 1
}

fn check_result(input: Tape<Language>, expected: &Tape<Language>) {
    let input_length = input.get_length();
    let mut tm = TuringMachine::new(generate_table(), input);
    let result = tm.run();
    match result {
        Ok((answer, steps)) => {
            assert_eq!(*answer, *expected, "Compare results");
            assert_eq!(
                steps,
                get_expected_steps(input_length),
                "Compare number of steps taken"
            );
        }
        Err(e) => panic!("{e}"),
    }
}
#[test]
fn test_empty() {
    let input = [];
    let output = [Language::Empty];
    check_result(Tape::from(input.as_slice()), &Tape::from(output.as_slice()));
}

#[test]
fn test_a() {
    let input = [Language::A];
    let output = [Language::A];
    check_result(Tape::from(input.as_slice()), &Tape::from(output.as_slice()));
}

#[test]
fn test_b() {
    let input = [Language::B];
    let output = [Language::B];
    check_result(Tape::from(input.as_slice()), &Tape::from(output.as_slice()));
}

#[test]
fn test_ab() {
    let input = [Language::A, Language::B];
    let output = [Language::B, Language::A];
    check_result(Tape::from(input.as_slice()), &Tape::from(output.as_slice()));
}

#[test]
fn test_baa() {
    let input = [Language::B, Language::A, Language::A];
    let output = [Language::A, Language::A, Language::B];
    check_result(Tape::from(input.as_slice()), &Tape::from(output.as_slice()));
}

#[test]
fn test_abba() {
    let input = [Language::A, Language::B, Language::B, Language::A];
    let output = [Language::A, Language::B, Language::B, Language::A];
    check_result(Tape::from(input.as_slice()), &Tape::from(output.as_slice()));
}

#[test]
fn test_babbb() {
    let input = [
        Language::B,
        Language::A,
        Language::B,
        Language::B,
        Language::B,
    ];
    let output = [
        Language::B,
        Language::A,
        Language::B,
        Language::B,
        Language::B,
    ];
    check_result(Tape::from(input.as_slice()), &Tape::from(output.as_slice()));
}
