use crate::{
    error::{self, Error},
    language::Language,
    tape::Tape,
    transition::Table,
    Direction,
};

#[derive(Default)]
pub struct TuringMachine<L: Language> {
    tape_head: usize,
    steps: usize,
    state: usize,
    table: Table<L>,
    tape: Tape<L>,
}

impl<L: Language> TuringMachine<L> {
    #[must_use]
    pub fn new(table: Table<L>, tape: Tape<L>) -> Self {
        Self {
            tape_head: 0,
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
        let current_value: L = self.tape.read_single(self.tape_head);

        // Using the current state and value, get the transition.
        let transition = self
            .table
            .get_from_state_and_value(self.state, current_value)?;

        let write = transition.write;
        let direction = transition.direction;
        let to = transition.to;

        println!("{}, {}", self.tape_head, self.tape.get_capacity());
        self.tape.write_single(self.tape_head, write);
        println!("AAA");

        // Ensure that we cannot move off the tape.
        if direction == Direction::Left && self.tape_head == 0 {
            return Err(Error::new(error::Type::EndOfTapeReached));
        }
        match direction {
            Direction::Left => self.tape_head -= 1,
            Direction::Right => self.tape_head += 1,
        };

        self.state = to;
        Ok(())
    }

    /// # Errors
    pub fn run(&mut self) -> Result<(&[L], usize), Error> {
        // Loop until it ends up in a final state.
        while self.state != 1 {
            self.step()?;
        }

        // Return the result
        Ok((self.tape.read_slice(0, self.tape.get_length()), self.steps))
    }
}
