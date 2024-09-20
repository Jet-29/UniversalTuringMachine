use crate::{
    error::{self, Error},
    language::Language,
};

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
    ///
    /// # Errors
    /// Can return an error if the location is invalid.
    pub fn write_location(&mut self, loc: &Location, value: L) -> Result<(), Error> {
        let idx = loc.get_location()?;
        self.extend_to_fit(idx);
        self.internal_tape[idx] = value;
        Ok(())
    }

    /// Writes a slice to the internal array.
    /// Will resize the internal array to fit the new slice
    ///
    /// # Errors
    /// Can return an error if the location is invalid.
    pub fn write_location_slice(&mut self, start_loc: &Location, slice: &[L]) -> Result<(), Error> {
        let offset = start_loc.get_location()?;
        self.extend_to_fit(offset + slice.len());
        self.internal_tape[offset..(offset + slice.len())].copy_from_slice(slice);
        Ok(())
    }

    /// Will read a single value.
    /// If location > len, it will return an empty
    ///
    /// # Errors
    /// Can return an error if the location is invalid.
    pub fn read_location(&self, loc: &Location) -> Result<L, Error> {
        let idx = loc.get_location()?;
        if self.internal_tape.len() <= idx {
            return Ok(L::empty());
        }
        Ok(self.internal_tape[idx])
    }

    /// Returns a slice to the internal vec
    ///
    /// Uses Start location and size to read
    ///
    /// # Errors
    /// Can return an error if the location is invalid.
    pub fn read_slice_offset(&self, start_loc: &Location, size: usize) -> Result<&[L], Error> {
        // TODO: When asking for values not in the array, ie offset > len. return a slice of empties somehow?
        let offset = start_loc.get_location()?;
        Ok(&self.internal_tape[offset..(offset + size)])
    }

    /// Returns a slice range
    ///
    /// Uses Start and End Location
    ///
    /// # Errors
    /// Can return an error if the location is invalid.
    pub fn read_slice_range(
        &self,
        start_loc: &Location,
        end_loc: &Location,
    ) -> Result<&[L], Error> {
        // TODO: When asking for values not in the array, ie offset > len. return a slice of empties somehow?
        let start_idx = start_loc.get_location()?;
        let end_idx = end_loc.get_location()?;
        Ok(&self.internal_tape[start_idx..end_idx])
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
    ///
    /// # Panics
    /// This function will only panic if an internal bug has occured.
    #[must_use]
    pub fn get_length(&self) -> usize {
        // Loop from the back until the first non-empty symbol is found.
        (0..self.get_capacity())
            .rev()
            .find(|&idx| {
                self.read_location(&Location::from(idx))
                    .expect("Expected to terminate at 0")
                    != L::empty()
            })
            .map_or(0, |idx| idx + 1) // Add one to convert from idx to length
    }
}

impl<L: Language> PartialEq for Tape<L> {
    fn eq(&self, other: &Self) -> bool {
        let zero_idx = Location::from(0);
        // TODO: Kinda ugly please refactor
        self.read_slice_offset(&zero_idx, self.get_length())
            .expect("Expected tape to be of length 0 to self.length")
            == other
                .read_slice_offset(&zero_idx, other.get_length())
                .expect("Expected tape to be of length 0 to self.length")
    }
}

impl<L: Language> From<&[L]> for Tape<L> {
    fn from(value: &[L]) -> Self {
        let mut tape = Self::new();
        tape.set_tape(value);
        tape
    }
}

pub struct Location {
    location: usize,
    is_valid: bool,
}

impl Location {
    #[must_use]
    pub fn new() -> Self {
        Self {
            location: 0,
            is_valid: true,
        }
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    fn get_location(&self) -> Result<usize, Error> {
        if self.is_valid {
            return Ok(self.location);
        };
        Err(Error::new(error::Type::EndOfTapeReached))
    }

    pub fn move_direction(&mut self, dir: Direction) {
        if self.location == 0 && dir == Direction::Left {
            self.is_valid = false;
        } else if dir == Direction::Left {
            self.location -= 1;
        } else {
            self.location += 1;
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl From<usize> for Location {
    fn from(value: usize) -> Self {
        Self {
            location: value,
            ..Default::default()
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
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
