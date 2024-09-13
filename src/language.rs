/// # Language trait
///
/// Is used to allow any language to be used with a turing machine.
/// A turing machine can only use one language at a time so this language must encompase everything.
///
/// The language must also include important symbols such as empty which will be used for padding and will be trimmed from the end when returing the result.
///
/// ## Traits
/// Copy is needed as symbols will get copied into different locations in the tape as well as duplicated.
/// Partial Eq is needed to compare symbols with each other to allow to empty symbol checking.
///
pub trait Language: Copy + PartialEq {
    fn empty() -> Self;
}
