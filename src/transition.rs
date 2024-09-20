use crate::{
    error::{self, Error},
    language::Language,
    tape::Direction,
};

/// # Transition Table
///
/// A table to house all transitions.
#[derive(Default)]
pub struct Table<L: Language> {
    transitions: Vec<Transition<L>>,
}

impl<L: Language> Table<L> {
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

/// # Transition
///
/// A simple structure to house all values needed for a transition.
#[derive(Debug, Clone)]
pub struct Transition<L: Language> {
    pub from: usize,
    pub to: usize,
    pub read: L,
    pub write: L,
    pub direction: Direction,
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
