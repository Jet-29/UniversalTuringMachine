use std::fmt::Debug;

pub mod error;
pub mod language;
pub mod tape;
pub mod transition;

use error::Error;
use language::Language;
use tape::Tape;
use transition::Table;

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
    pub fn new(table: Table<L>) -> Self {
        Self {
            tape_head: 0,
            state: 0,
            steps: 0,
            table,
            tape: Tape::new(),
        }
    }

    pub fn write_to_tape_single(&mut self, offset: usize, value: L) {
        self.tape.write_single(offset, value);
    }

    pub fn write_to_tape_slice(&mut self, slice: &[L]) {
        self.write_to_tape_slice_offset(0, slice);
    }

    pub fn write_to_tape_slice_offset(&mut self, offset: usize, slice: &[L]) {
        self.tape.write_slice(offset, slice);
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

        self.write_to_tape_single(self.tape_head, write);

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

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
}
