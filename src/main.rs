use universal_turing_machine::{Direction, Transition, TransitionTable, TuringMachine};

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

fn main() {
    // TODO: Make this cli to the actual lib so i can give it a transition table file or just some string encoding a table.
    let transitions = [
        Transition::new(0, 0, Language::A, Language::A, Direction::Right),
        Transition::new(0, 2, Language::B, Language::A, Direction::Right),
        Transition::new(2, 2, Language::A, Language::A, Direction::Right),
        Transition::new(2, 3, Language::Empty, Language::Empty, Direction::Left),
        Transition::new(3, 1, Language::A, Language::Empty, Direction::Right),
    ];

    let mut transition_table: TransitionTable<Language> = TransitionTable::new();

    transition_table.add_transitions(&transitions);

    let mut machine = TuringMachine::new(transition_table);
    machine.write_to_tape_slice(&[
        Language::A,
        Language::A,
        Language::A,
        Language::B,
        Language::A,
        Language::A,
    ]);
    match machine.run() {
        Ok(result) => {
            dbg!(result);
        }
        Err(err) => {
            println!("{err}");
        }
    };
}
