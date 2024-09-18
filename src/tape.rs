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

    /// Extends the array to fit the desired size.
    /// Will not truncate the array.
    fn extend_to_fit(&mut self, size: usize) {
        // resize will truncate the vec if it is greater than the size requested.
        if self.internal_tape.len() < (size + 1) {
            self.internal_tape.resize(size + 1, L::empty());
        }
    }

    /// Clear the tape
    pub fn clear(&mut self) {
        self.internal_tape.clear();
    }

    /// Override the tape with a slice
    pub fn set_tape(&mut self, new_tape: &[L]) {
        self.internal_tape = new_tape.to_vec();
    }

    /// Writes a single value to the array.
    /// if idx > len it will resize the array
    pub fn write_single(&mut self, pos: usize, value: L) {
        self.extend_to_fit(pos);
        self.internal_tape[pos] = value;
    }

    /// Writes a slice to the internal array.
    /// Will resize the internal array to fit the new slice
    pub fn write_slice(&mut self, offset: usize, slice: &[L]) {
        self.extend_to_fit(offset + slice.len());
        self.internal_tape[offset..(offset + slice.len())].copy_from_slice(slice);
    }

    /// Will read a single value.
    /// If idx > len, it will return an empty
    #[must_use]
    pub fn read_single(&self, pos: usize) -> L {
        if self.internal_tape.len() <= pos {
            return L::empty();
        }
        self.internal_tape[pos]
    }

    /// Returns a slice to the internal vec
    #[must_use]
    pub fn read_slice(&self, offset: usize, size: usize) -> &[L] {
        // TODO: When asking for values not in the array, ie offset > len. return a slice of empties somehow?
        &self.internal_tape[offset..(offset + size)]
    }

    /// The capacity is the number of symbols on the tape.
    /// This includes all empty symbols.
    #[must_use]
    pub fn get_capacity(&self) -> usize {
        self.internal_tape.len()
    }

    /// The length is the number of symbols on the tape.
    /// This excludes all empty symbols past the last non-empty symbol
    /// Assuming empty symbols are not used for anything, this will return the length of the data.
    #[must_use]
    pub fn get_length(&self) -> usize {
        // Loop from the back until the first non-empty symbol is found.
        (0..self.get_capacity())
            .rev()
            .find(|&idx| self.read_single(idx) != L::empty())
            .map_or(0, |idx| idx + 1) // Add one to convert from idx to length
    }
}

impl<L: Language> PartialEq for Tape<L> {
    fn eq(&self, other: &Self) -> bool {
        let var_name =
            self.read_slice(0, self.get_length()) == other.read_slice(0, other.get_length());
        var_name
    }
}

impl<L: Language> From<&[L]> for Tape<L> {
    fn from(value: &[L]) -> Self {
        let mut tape = Self::new();
        tape.set_tape(value);
        tape
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Language {
        A,
        B,
        Empty,
    }

    impl crate::language::Language for Language {
        fn empty() -> Self {
            Language::Empty
        }
    }

    #[test]
    fn test_from() {
        let data = vec![Language::A, Language::B, Language::Empty];
        assert_eq!(
            Tape::from(data.as_slice()),
            Tape {
                internal_tape: vec![Language::A, Language::B, Language::Empty]
            }
        );
    }

    #[test]
    fn test_eq() {
        let tape_true =
            Tape::from([Language::A, Language::Empty, Language::B, Language::Empty].as_slice());
        let tape_same =
            Tape::from([Language::A, Language::Empty, Language::B, Language::Empty].as_slice());
        let tape_missing_space = Tape::from([Language::A, Language::Empty, Language::B].as_slice());
        let tape_different =
            Tape::from([Language::A, Language::Empty, Language::Empty, Language::B].as_slice());

        assert_eq!(tape_true, tape_same, "Test two identical tapes");
        assert_eq!(
            tape_true, tape_missing_space,
            "Test trailing spaces are ignored"
        );
        assert_ne!(tape_true, tape_different, "Test two different tapes");
    }
}
