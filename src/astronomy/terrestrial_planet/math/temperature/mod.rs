use crate::astronomy::_constant::*;
use crate::astronomy::_type::*;
use crate::astronomy::star::math::distance::METERS_PER_AU;
use crate::astronomy::terrestrial_planet::constants::*;
use std::f64::consts::PI;

pub const GREENHOUSE_EFFECT: f64 = 0.5841;

/// Calculate the equilibrium temperature for a planet based on the host star's
/// luminosity, distance, etc.
/// Answer in Kelvin.
pub fn get_equilibrium_temperature(
  bond_albedo: f64,
  greenhouse_effect: f64,
  star_luminosity: LSol,
  star_distance: LAu,
) -> TKel {
  let luminosity = star_luminosity * ERGS_PER_SEC_PER_LSOL;
  let distance = star_distance * METERS_PER_AU * 100.0;
  let t_greenhouse = greenhouse_effect * GREENHOUSE_EFFECT;
  let absorption = ((1.0 - bond_albedo) * luminosity.0 / (16.0 * PI * STEFAN_BOLTZMANN_CONSTANT)).sqrt();
  let t_effective = absorption.sqrt() * (1.0 / distance.0.sqrt());
  let t_equilibrium = t_effective.powf(4.0) * (1.0 + (3.0 * t_greenhouse / 4.0));
  let t_surface = t_equilibrium / 0.9;
  TKel(t_surface.powf(1.0 / 4.0))
}
