//! # Celestial Body Properties
//!
//! This module provides physical properties for planets, moons, and other celestial bodies
//! commonly used in astrodynamics calculations.
//!
//! ## Overview
//!
//! Celestial bodies are represented by the [`CelestialBody`] struct, which contains
//! fundamental physical properties needed for orbital mechanics:
//!
//! - **Mass**: Used for gravitational parameter calculations (μ = GM)
//! - **Radius**: Used for surface gravity, escape velocity, and orbital altitude references
//! - **Name**: Human-readable identification
//!
//! ## Usage
//!
//! ```rust
//! use almagest::celestials::celestial_bodies::{EARTH, MARS};
//!
//! // Access predefined celestial bodies
//! println!("Earth mass: {:.3e} kg", EARTH.mass.0);
//! println!("Mars radius: {} km", MARS.radius.0);
//!
//! // Use in orbital calculations
//! // TODO
//! // let earth_surface_gravity = calculate_surface_gravity(&EARTH);
//! ```
//!
//! ## Data Sources
//!
//! Physical properties are sourced from:
//! - NASA/JPL Planetary Fact Sheets
//! - IAU 2015 Nominal Values
//! - IERS Conventions (2010)
//!
//! Values are given in SI units with sufficient precision for most astrodynamics applications.

use crate::utils::{Kilograms, Kilometers};

/// Represents a celestial body with fundamental physical properties.
///
/// Contains the basic properties needed for most astrodynamics calculations:
/// mass, radius, and a human-readable name.
///
/// # Examples
///
/// ```rust
/// use almagest::celestials::{CelestialBody, celestial_bodies::EARTH};
/// use almagest::utils::{Kilograms, Kilometers};
///
/// // Using predefined bodies
/// let earth = EARTH;
/// println!("{} has mass {:.2e} kg", earth.name, earth.mass.0);
///
/// // Creating custom bodies
/// let custom_body = CelestialBody {
///     name: "Asteroid Ceres",
///     mass: Kilograms(9.1e20),      // kg
///     radius: Kilometers(473.0),    // km (mean radius)
/// };
/// ```
#[derive(Debug)]
pub struct CelestialBody<'a> {
    /// Human-readable name of the celestial body
    pub name: &'a str,
    /// Total mass in kilograms
    pub mass: Kilograms,
    /// Mean radius in kilometers
    pub radius: Kilometers,
}

/// Pre-defined celestial bodies with accurate physical properties.
///
/// This module contains constants for commonly referenced planets and moons,
/// with properties sourced from authoritative astronomical databases.
pub mod celestial_bodies {
    use super::*;

    /// Earth - Third planet from the Sun
    ///
    /// Physical properties:
    /// - **Mass**: 5.972×10²⁴ kg (± 0.006×10²⁴ kg)
    /// - **Radius**: 6,371 km (mean radius)
    /// - **Surface gravity**: ~9.81 m/s²
    /// - **Escape velocity**: ~11.2 km/s
    /// - **Standard gravitational parameter (μ)**: 398,600.4418 km³/s²
    ///
    /// # References
    /// - IAU 2015 Resolution B3
    /// - IERS Conventions (2010)
    pub const EARTH: CelestialBody<'static> = CelestialBody {
        name: "Earth",
        mass: Kilograms(5.972e24),
        radius: Kilometers(6_371.0),
    };

    /// Mars - Fourth planet from the Sun, "The Red Planet"
    ///
    /// Physical properties:
    /// - **Mass**: 6.417×10²³ kg
    /// - **Radius**: 3,390 km (mean radius)
    /// - **Surface gravity**: ~3.71 m/s² (38% of Earth's)
    /// - **Escape velocity**: ~5.03 km/s
    /// - **Day length**: ~24.62 hours (1.026 Earth days)
    /// - **Standard gravitational parameter (μ)**: 42,828.37 km³/s²
    ///
    /// # References
    /// - NASA Mars Fact Sheet
    /// - JPL Solar System Dynamics
    pub const MARS: CelestialBody<'static> = CelestialBody {
        name: "Mars",
        mass: Kilograms(6.417e23),
        radius: Kilometers(3_390.0),
    };

    /// Moon - Earth's natural satellite
    ///
    /// Physical properties:
    /// - **Mass**: 7.35×10²² kg (~1.2% of Earth's mass)
    /// - **Radius**: 1,737 km (mean radius, ~27% of Earth's)
    /// - **Surface gravity**: ~1.62 m/s² (16.5% of Earth's)
    /// - **Escape velocity**: ~2.38 km/s
    /// - **Orbital distance**: ~384,400 km from Earth (mean)
    /// - **Standard gravitational parameter (μ)**: 4,902.8 km³/s²
    ///
    /// # References
    /// - NASA Moon Fact Sheet
    /// - IAU lunar parameters
    pub const MOON: CelestialBody<'static> = CelestialBody {
        name: "Moon",
        mass: Kilograms(7.35e22),
        radius: Kilometers(1_737.48),
    };
}
