/// Errors encountered in scanning the source code.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// Encountered an unexpected character.
  #[error("encountered an unexpected character ({0})")]
  UnexpectedCharacter(char),
  /// Encountered an unterminated string.
  #[error("encountered an unterminated string on line {0}")]
  UnterminatedString(usize),
}
