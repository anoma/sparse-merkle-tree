use crate::{string, H256};

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    MissingKey(usize, H256),
    CorruptedProof,
    EmptyProof,
    EmptyKeys,
    IncorrectNumberOfLeaves { expected: usize, actual: usize },
    Store(string::String),
    CorruptedStack,
    NonSiblings,
    InvalidCode(u8),
    NonMergableRange,
    ExistenceProof,
    NonExistenceProof,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::MissingKey(height, key) => {
                write!(f, "Missing key at height {}, key {:?}", height, key)?;
            }
            Error::CorruptedProof => {
                write!(f, "Corrupted proof")?;
            }
            Error::EmptyProof => {
                write!(f, "Empty proof")?;
            }
            Error::EmptyKeys => {
                write!(f, "Empty keys")?;
            }
            Error::IncorrectNumberOfLeaves { expected, actual } => {
                write!(
                    f,
                    "Incorrect number of leaves, expected {} actual {}",
                    expected, actual
                )?;
            }
            Error::Store(err_msg) => {
                write!(f, "Backend store error: {}", err_msg)?;
            }
            Error::CorruptedStack => {
                write!(f, "Corrupted compiled proof stack")?;
            }
            Error::NonSiblings => {
                write!(f, "Merging non-siblings in compiled stack")?;
            }
            Error::InvalidCode(code) => {
                write!(f, "Invalid compiled proof code: {}", code)?;
            }
            Error::NonMergableRange => {
                write!(f, "Ranges can not be merged")?;
            }
            Error::ExistenceProof => {
                write!(f, "Try to get ExistenceProof for non existing value")?;
            }
            Error::NonExistenceProof => {
                write!(f, "Try to get NonExistenceProof for the existing value")?;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
