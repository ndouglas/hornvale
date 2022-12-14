use crate::astronomy::stellar_neighborhood::error::Error as StellarNeighborhoodError;

/// Galaxy-class errors.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// Stellar Neighborhood Error.
  #[error("an error occurred in the stellar neighborhood ({0})")]
  StellarNeighborhoodError(#[from] StellarNeighborhoodError),
}
