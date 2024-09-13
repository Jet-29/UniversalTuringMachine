use crate::language::Language;

/// # Tape
///
/// This is a wrapper around a vector to provide tape specific functionality.
/// This functionality includes:
/// Infinite length in the positive direction.
/// Tape specific get length to ignore empty symbols.
///
/// In the future a different internal type may be used.
/// This should not affect the interface.
#[derive(Default, Debug)]
pub struct Tape<L: Language> {
    internal_tape: Vec<L>,
}

impl<L: Language> Tape<L> {
    #[must_use]
    pub fn new() -> Self {
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

    pub fn clear(&mut self) {
        self.internal_tape.clear();
    }

    pub fn set_tape(&mut self, new_tape: &[L]) {
        self.internal_tape = new_tape.to_vec();
    }

    pub fn write_single(&mut self, pos: usize, value: L) {
        self.internal_tape[pos] = value;
    }

    pub fn write_slice(&mut self, offset: usize, slice: &[L]) {
        self.extend_to_fit(offset + slice.len());
        self.internal_tape[offset..(offset + slice.len())].copy_from_slice(slice);
    }

    #[must_use]
    pub fn read_single(&self, pos: usize) -> L {
        self.internal_tape[pos]
    }

    #[must_use]
    pub fn read_slice(&self, offset: usize, size: usize) -> &[L] {
        &self.internal_tape[offset..(offset + size)]
    }

    /// The capacity is the number of symbols on the tape.
    /// This includes all empty symbols.
    #[must_use]
    fn get_capacity(&self) -> usize {
        self.internal_tape.len()
    }

    /// The length is the number of symbols on the tape.
    /// This excludes all empty symbols past the last non-empty symbol
    /// Assuming empty symbols are not used for anything, this will return the length of the data.
    #[must_use]
    pub fn get_length(&self) -> usize {
        // Loop from the back until the first non-empty symbol is found.
        let first_valid_index = (0..self.get_capacity())
            .rev()
            .find(|&idx| self.read_single(idx) != L::empty())
            .unwrap_or(1);

        // Add 1 to conert from index to count
        first_valid_index + 1
    }
}
