use crate::astronomy::star::math::spectral_class::{
  get_random_habitable_spectral_class, get_random_spectral_class, spectral_class_to_habitable_mass_range,
  spectral_class_to_mass_range,
};
use rand::prelude::*;

const KG_PER_SOLAR_MASS: f64 = 1.989E30;

/// Msol -> KG
pub fn msol_to_kg(msol: f64) -> f64 {
  msol * KG_PER_SOLAR_MASS
}

/// KG -> Msol
pub fn kg_to_msol(kg: f64) -> f64 {
  kg / KG_PER_SOLAR_MASS
}

/// Get a (weighted) random mass for a star.
#[named]
pub fn get_random_stellar_mass<R: Rng + ?Sized>(rng: &mut R) -> f64 {
  trace_enter!();
  let spectral_class = get_random_spectral_class(rng);
  trace_var!(spectral_class);
  let mass_range = spectral_class_to_mass_range(spectral_class);
  trace_var!(mass_range);
  let result = rng.gen_range(mass_range);
  trace_var!(result);
  trace_exit!();
  result
}

/// Get a (weighted) habitable random mass for a star.
#[named]
pub fn get_random_habitable_stellar_mass<R: Rng + ?Sized>(rng: &mut R) -> f64 {
  trace_enter!();
  let spectral_class = get_random_habitable_spectral_class(rng);
  trace_var!(spectral_class);
  let mass_range = spectral_class_to_habitable_mass_range(spectral_class);
  trace_var!(mass_range);
  let result = rng.gen_range(mass_range);
  trace_var!(result);
  trace_exit!();
  result
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_msol_to_kg() {
    init();
    trace_enter!();
    assert_approx_eq!(kg_to_msol(msol_to_kg(1.0)), 1.0);
    assert_approx_eq!(msol_to_kg(1.0), KG_PER_SOLAR_MASS);
    trace_exit!();
  }
}