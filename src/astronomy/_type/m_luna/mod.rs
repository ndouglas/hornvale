use super::MEarth;
use crate::astronomy::_constants::*;

/// The `MLuna` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct MLuna(pub f64);

impl From<MEarth> for MLuna {
  fn from(original: MEarth) -> Self {
    Self(original.0 * LUNA_MASS_PER_EARTH_MASS)
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_m_earth_to_m_luna() {
    init();
    let actual: MLuna = MEarth(1.0).into();
    assert_approx_eq!(actual.0, LUNA_MASS_PER_EARTH_MASS, 0.01);
  }
}
