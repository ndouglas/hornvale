use std::error::Error as StdError;
use std::io::Error as IoError;

use crate::parsing::error::Error as ParsingError;

/// Errors encountered in interpreting.
#[derive(Debug, Error)]
pub enum Error {
  /// A standard error occurred.
  #[error("an error occurred ({0})")]
  StandardError(#[from] Box<dyn StdError>),
  /// An I/O error occurred.
  #[error("an error occurred ({0})")]
  IoError(#[from] IoError),
  /// A parsing error occurred.
  #[error("a parsing error occurred ({0})")]
  ParsingError(#[from] ParsingError),
}
