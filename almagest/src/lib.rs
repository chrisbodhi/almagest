//! # Almagest: Astrodynamics Library
//!
//! A comprehensive `no_std` library for orbital mechanics, astrodynamics, and space engineering calculations.
//!
//! ## Overview
//!
//! Almagest provides tools for:
//! - **Orbital Mechanics**: Kepler elements, orbit propagation, and analysis
//! - **Material Properties**: Database of aerospace materials with physical properties
//! - **Space Tethers**: Characteristic velocity calculations and tether analysis
//! - **Type-Safe Units**: Compile-time unit checking for physical calculations
//! - **Celestial Bodies**: Properties and parameters for planets and moons
//!
//! ## Design Philosophy
//!
//! - **`no_std` compatible**: Works in embedded and constrained environments
//! - **Type safety**: Physical units are encoded in the type system
//! - **Precision**: Uses `f64` throughout for maximum numerical accuracy
//! - **Validation**: Input validation with meaningful error messages
//! - **Pure functions**: Calculations are deterministic and side-effect free
//!
//! ## Quick Start
//!
//! ```rust
//! use almagest::tethers::characteristic_velocity;
//! use almagest::utils::{Pascals, KilogramsPerMetersCubed};
//! use almagest::materials::fibers;
//!
//! // Calculate characteristic velocity for a tether material
//! let pbo_velocity = characteristic_velocity(
//!     Pascals(5.9e9),                  // 5.9 GPa tensile strength
//!     KilogramsPerMetersCubed(1340.0)  // 1340 kg/m³ density
//! ).expect("Valid material properties");
//!
//! // Or use predefined materials
//! let kevlar_velocity = almagest::tethers::characteristic_velocity_for_material(
//!     &fibers::KEVLAR_49
//! ).expect("Valid Kevlar properties");
//! ```
//!
//! ## Module Organization
//!
//! - [`utils`] - Type-safe physical units, constants, and utility types
//! - [`materials`] - Database of aerospace materials and their properties
//! - [`tethers`] - Space tether analysis and characteristic velocity calculations
//! - [`kepler`] - Orbital mechanics using Keplerian elements
//! - [`celestials`] - Properties of celestial bodies (planets, moons, etc.)
//!
//! ## Mathematical Foundation
//!
//! All calculations use established formulas from astrodynamics and materials science:
//!
//! - **Characteristic Velocity**: `v = √(2σ/ρ)` where σ is tensile strength, ρ is density
//! - **Orbital Elements**: Standard Keplerian element definitions and transformations
//! - **Material Properties**: Based on published aerospace materials data
//!
//! ## Error Handling
//!
//! Functions return `Result<T, &'static str>` for calculations that can fail due to:
//! - Invalid input parameters (negative values, out of range)
//! - Physical impossibilities (unbound orbits, etc.)
//! - Numerical edge cases
//!
//! ## WebAssembly Support
//!
//! For web applications, see the companion `almagest-wasm` crate which provides
//! JavaScript bindings for this library.

#![no_std]

pub mod celestials;
pub mod kepler;
pub mod materials;
pub mod tethers;
pub mod utils;
