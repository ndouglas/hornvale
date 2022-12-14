use crate::astronomy::host_star::error::Error as HostStarError;
use crate::astronomy::moon::error::Error as MoonError;
use crate::astronomy::planet::error::Error as PlanetError;

/// Moon-related errors.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// Host Star Error.
  #[error("an error occurred in the host star ({0})")]
  HostStarError(#[from] HostStarError),
  /// Moon Error.
  #[error("an error occurred in the moon ({0})")]
  MoonError(#[from] MoonError),
  /// Planet Error.
  #[error("an error occurred in the planet ({0})")]
  PlanetError(#[from] PlanetError),
}
