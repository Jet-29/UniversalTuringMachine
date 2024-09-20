use crate::{
    error::Error,
    language::Language,
    tape::{Location, Tape},
    transition::Table,
};

#[derive(Default)]
pub struct TuringMachine<L: Language> {
    tape_head: Location,
    steps: usize,
    state: usize,
    table: Table<L>,
    tape: Tape<L>,
}

impl<L: Language> TuringMachine<L> {
    #[must_use]
    pub fn new(table: Table<L>, tape: Tape<L>) -> Self {
        Self {
            tape_head: Location::new(),
            state: 0,
            steps: 0,
            table,
            tape,
        }
    }

    /// # Errors
    pub fn step(&mut self) -> Result<(), Error> {
        self.steps += 1;
        // Get value at current position/
        let current_value: L = self.tape.read_location(&self.tape_head)?;

        // Using the current state and value, get the transition.
        let transition = self
            .table
            .get_from_state_and_value(self.state, current_value)?;

        let write = transition.write;
        let direction = transition.direction;
        let to = transition.to;

        self.tape.write_location(&self.tape_head, write)?;
        self.tape_head.move_direction(direction);

        self.state = to;
        Ok(())
    }

    /// # Errors
    pub fn run(&mut self) -> Result<(&Tape<L>, usize), Error> {
        // Loop until it ends up in a final state.
        while self.state != 1 {
            self.step()?;
        }

        // Return the result
        Ok((&self.tape, self.steps))
    }
}
