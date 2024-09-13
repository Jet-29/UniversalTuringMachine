use std::fmt::Debug;

pub mod error;
pub mod language;
pub mod tape;
pub mod transition;
pub mod turing_machine;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
}
