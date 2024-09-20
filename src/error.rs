use std::fmt::Display;

// TODO: Include important info about the current state.
// Maybe even a transition trace.

/// # Error
///
/// A structure to hold all key details about the error that occured.
#[derive(Debug)]
pub struct Error {
    error_type: Type,
}

impl Error {
    #[must_use]
    pub fn new(error_type: Type) -> Self {
        Self { error_type }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.error_type)
    }
}

/// Type of error that occured.
#[derive(Debug)]
pub enum Type {
    NonDeterministic,
    NoStateFound,
    EndOfTapeReached,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::NonDeterministic => "Non-Deterministic",
                Self::NoStateFound => "No valid next state",
                Self::EndOfTapeReached => "Invalid tape location",
            }
        )
    }
}
