use std::fmt::Debug;

pub mod error;
pub mod language;

use error::Error;
use language::Language;

#[derive(Default)]
pub struct TuringMachine<L: Language> {
    tape_head: usize,
    steps: usize,
    state: usize,
    table: TransitionTable<L>,
    tape: Tape<L>,
}

impl<L: Language> TuringMachine<L> {
    #[must_use]
    pub fn new(table: TransitionTable<L>) -> Self {
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

        let result_length = (1..=self.tape.get_current_length())
            .rev()
            .find(|&idx| self.tape.read_single(idx - 1) != L::empty())
            .unwrap_or(1);

        // Return the result
        Ok((self.tape.read_slice(0, result_length), self.steps))
    }
}

#[derive(Default, Debug)]
struct Tape<L: Language> {
    internal_tape: Vec<L>,
}

impl<L: Language> Tape<L> {
    fn new() -> Self {
        Self {
            internal_tape: Vec::new(),
        }
    }

    fn extend_to_fit(&mut self, size: usize) {
        // resize will truncate the vec if it is greater than the size requested.
        if self.internal_tape.len() < (size + 1) {
            self.internal_tape.resize(size + 1, L::empty());
        }
    }

    fn write_single(&mut self, pos: usize, value: L) {
        self.internal_tape[pos] = value;
    }

    fn write_slice(&mut self, offset: usize, slice: &[L]) {
        self.extend_to_fit(offset + slice.len());
        self.internal_tape[offset..(offset + slice.len())].copy_from_slice(slice);
    }

    fn read_single(&self, pos: usize) -> L {
        if self.internal_tape.len() < pos {
            return L::empty();
        }
        self.internal_tape[pos]
    }

    fn read_slice(&self, offset: usize, size: usize) -> &[L] {
        &self.internal_tape[offset..(offset + size)]
    }

    fn get_current_length(&self) -> usize {
        self.internal_tape.len()
    }
}

#[derive(Default)]
pub struct TransitionTable<L: Language> {
    transitions: Vec<Transition<L>>,
}

impl<L: Language> TransitionTable<L> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            transitions: Vec::new(),
        }
    }

    pub fn add_transition(&mut self, transition: Transition<L>) {
        self.transitions.push(transition);
    }

    pub fn add_transitions(&mut self, transitions: &[Transition<L>]) {
        self.transitions.extend_from_slice(transitions);
    }
    /// # Errors
    pub fn get_from_state_and_value(
        &self,
        state: usize,
        value: L,
    ) -> Result<&Transition<L>, Error> {
        // TODO: Perform table checking for these issues at the start.
        // Dont return the value early, we must check it is deterministic.
        let mut next_transition = None;
        for transition in &self.transitions {
            if transition.from == state && transition.read == value {
                if next_transition.is_some() {
                    return Err(Error::new(error::Type::NonDeterministic));
                }
                next_transition = Some(transition);
            }
        }

        if let Some(next) = next_transition {
            return Ok(next);
        }
        Err(Error::new(error::Type::NoStateFound))
    }
}

#[derive(Debug, Clone)]
pub struct Transition<L: Language> {
    from: usize,
    to: usize,
    read: L,
    write: L,
    direction: Direction,
}

impl<L: Language> Transition<L> {
    pub fn new(from: usize, to: usize, read: L, write: L, direction: Direction) -> Self {
        Self {
            from,
            to,
            read,
            write,
            direction,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
}
