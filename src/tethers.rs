use libm::exp;

use crate::utils::{G, Kilograms, Meters, Real, Seconds};

pub struct PlanetData {
    pub mass: Kilograms,
    pub radius: Meters,
    pub rotation_period: Seconds,
}

const PLANET_DATA: [PlanetData; 2] = [
    PlanetData {
        mass: Kilograms(5.972e24),
        radius: Meters(6.371e6),
        // Sidereal day
        rotation_period: Seconds(86_164.0),
    },
    PlanetData {
        mass: Kilograms(7.34767309e22),
        radius: Meters(1.7374e6),
        rotation_period: Seconds(2_360_591.0),
    },
];

impl PlanetData {
    pub fn rotational_angular_velocity(&self) -> Real {
        2.0 * core::f64::consts::PI / self.rotation_period.0
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Planets {
    Earth,
    Luna,
}

impl Planets {
    pub fn data(&self) -> &'static PlanetData {
        &PLANET_DATA[*self as usize]
    }
}

// material is an enum
// planet is an enum
// radius is a param
// orbital ang vel is param
// rot ang vel is param

/// Taper ratio, Moravec 1977
pub fn taper_ratio(planet: Planets) -> (Real, Real) {
    let period = planet.data().rotation_period;
    let dubya = planet.data().rotational_angular_velocity();

    // A(r) = cross-sectional area at radius r
    // A(rₚ) = cross-sectional area at planet surface
    //
    // δ = density of filament material
    // τ = tensile strength of filament material
    //
    // G = universal gravitational constant
    //
    // mₚ = mass of the planet
    // rₚ = radius of the planet
    // ωₚ = rotational angular velocity of the planet
    //
    // r₀ = radius of the orbit
    //
    // ω₀ = orbital angular velocity of the satellite
    // ωₛ = rotational angular velocity of the satellite
    //
    // A(r)/A(rₚ) = exp[(δ/τ)(Gmₚ(1/rₚ - 1/r) + r₀(ω₀² - ωₛ²)(rₚ - r) + ωₛ²(rₚ² - r²)/2)]
    todo!()
}
