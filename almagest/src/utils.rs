//! # Type-Safe Physical Units and Mathematical Utilities
//!
//! This module provides type-safe wrappers for physical units used throughout the library,
//! ensuring dimensional correctness at compile time and preventing unit conversion errors.
//!
//! ## Design Principles
//!
//! - **Zero-cost abstractions**: Unit types compile down to raw `f64` values
//! - **Dimensional analysis**: The type system prevents invalid operations (e.g., adding length to mass)
//! - **Operator overloading**: Natural mathematical syntax with proper unit propagation
//! - **Display formatting**: Human-readable output with appropriate unit symbols
//!
//! ## Unit System
//!
//! All units are based on the International System of Units (SI):
//!
//! | Type | Unit | Symbol | Usage |
//! |------|------|--------|-------|
//! | [`Meters`] | meter | m | Length, distance, radius |
//! | [`Kilometers`] | kilometer | km | Large distances, orbital parameters |
//! | [`MetersSquared`] | square meter | m² | Area, cross-sections |
//! | [`MetersCubed`] | cubic meter | m³ | Volume |
//! | [`Pascals`] | pascal | Pa | Pressure, stress, tensile strength |
//! | [`Kilograms`] | kilogram | kg | Mass |
//! | [`KilogramsPerMetersCubed`] | kg/m³ | kg/m³ | Density |
//! | [`MetersPerSecond`] | meter per second | m/s | Velocity, speed |
//! | [`MetersPerSecondSquared`] | meter per second squared | m/s² | Acceleration, gravity |
//! | [`MetersCubedByKilogramSecondsSquared`] | m³/(kg·s²) | m³/(kg·s²) | Gravitational parameter |
//!
//! ## Mathematical Operations
//!
//! Units support natural mathematical operations with automatic unit conversion:
//!
//! ```rust
//! use almagest::utils::{Meters, MetersSquared};
//!
//! let length = Meters(10.0);
//! let width = Meters(5.0);
//! let area: MetersSquared = length * width;  // = 50 m²
//!
//! let perimeter = (length + width) * 2.0;   // = 30 m
//! let ratio: f64 = length / width;          // = 2.0 (dimensionless)
//! ```
//!
//! ## Constants
//!
//! Mathematical and physical constants are provided with high precision:
//!
//! - [`PI`] - Archimedes' constant (π) ≈ 3.14159...
//! - [`TAU`] - Full circle constant (2π) ≈ 6.28318...
//! - [`E`] - Euler's number (e) ≈ 2.71828...
//! - [`G`] - Gravitational constant ≈ 6.6742×10⁻¹¹ m³/(kg·s²)

use core::cmp::{PartialEq, PartialOrd};
use core::fmt::{Debug, Display};
use core::ops::{Add, Div, Mul, Sub};

/// Floating-point type used throughout the library for maximum precision.
///
/// Currently set to `f64` for high-precision calculations. All physical
/// quantities and mathematical operations use this type.
pub type Real = f64;

/// Archimedes’ constant (π)
pub const PI: Real = core::f64::consts::PI;
/// The full circle constant (τ)
/// Equal to 2π.
pub const TAU: Real = core::f64::consts::TAU;
/// Euler's number (e)
pub const E: Real = core::f64::consts::E;

/// Length measurement in centimeters.
///
/// Used for small distances and material lengths/thicknesses
/// where meter values would be unwieldy.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Centimeters(pub Real);

/// Length measurement in meters.
///
/// Used for distances, radii, altitudes, and other linear measurements.
///
/// # Examples
/// ```rust
/// use almagest::utils::Meters;
///
/// let earth_radius = Meters(6_371_000.0);  // Earth's radius in meters
/// let altitude = Meters(408_000.0);        // ISS altitude
/// let orbit_radius = earth_radius + altitude;
/// ```
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Meters(pub Real);

/// Length measurement in kilometers.
///
/// Used for large distances and orbital parameters where meter values
/// would be unwieldy.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Kilometers(pub Real);

/// Area measurement in square centimeters.
///
/// Used for cross-sectional areas, surface areas, etc.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct CentimetersSquared(pub Real);

/// Area measurement in square meters.
///
/// Automatically created by multiplying two [`Meters`] values.
/// Used for cross-sectional areas, surface areas, etc.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct MetersSquared(pub Real);

/// Volume measurement in cubic meters.
///
/// Created by multiplying [`Meters`] by [`MetersSquared`].
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct MetersCubed(pub Real);

/// Pressure or stress measurement in pascals (N/m²).
///
/// Used for tensile strength, atmospheric pressure, and mechanical stress.
/// Common conversions:
/// - 1 GPa = 1×10⁹ Pa
/// - 1 MPa = 1×10⁶ Pa
/// - 1 bar = 1×10⁵ Pa
///
/// # Examples
/// ```rust
/// use almagest::utils::Pascals;
///
/// let steel_strength = Pascals(400e6);     // 400 MPa
/// let carbon_strength = Pascals(3.5e9);    // 3.5 GPa
/// ```
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Pascals(pub Real);

/// Force per unit area in N/m² (equivalent to [`Pascals`]).
/// (Newtons are equivalent to kg·m/s²)
///
/// Provided for situations where the force-per-area interpretation
/// is more natural than pressure.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct NewtonsPerMetersSquared(pub Real);

/// Mass measurement in kilograms.
///
/// Used for spacecraft mass, celestial body mass, and material mass.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Kilograms(pub Real);

/// Density measurement in kg/m³.
///
/// Used for material density, atmospheric density, and bulk density calculations.
///
/// # Examples
/// ```rust
/// use almagest::utils::KilogramsPerMetersCubed;
///
/// let water_density = KilogramsPerMetersCubed(1000.0);
/// let aluminum_density = KilogramsPerMetersCubed(2700.0);
/// ```
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct KilogramsPerMetersCubed(pub Real);

/// Gravitational parameter in m³/(kg·s²).
///
/// Used for gravitational constant G and standard gravitational parameters μ.
/// The gravitational parameter μ = GM where G is the gravitational constant
/// and M is the mass of the central body.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct MetersCubedByKilogramSecondsSquared(pub Real);

/// Velocity measurement in m/s.
///
/// Used for orbital velocities, characteristic velocities, and speed calculations.
///
/// # Examples
/// ```rust
/// use almagest::utils::MetersPerSecond;
///
/// let orbital_velocity = MetersPerSecond(7_800.0);  // ~LEO orbital velocity
/// let escape_velocity = MetersPerSecond(11_200.0);  // Earth escape velocity
/// ```
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct MetersPerSecond(pub Real);

/// Acceleration measurement in m/s².
///
/// Used for acceleration (positive), deceleration (negative), and gravity.
///
/// # Examples
/// ```rust
/// use almagest::utils::MetersPerSecondSquared;
///
/// let earth_gravity = MetersPerSecondSquared(9.81);
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct MetersPerSecondSquared(pub Real);

/// Time measurement in seconds.
///
/// Used for orbital periods, time intervals, and duration calculations.
///
/// # Examples
/// ```rust
/// use almagest::utils::Seconds;
///
/// let orbital_period = Seconds(5400.0);    // ~90 minute LEO orbit
/// let day = Seconds(86400.0);              // 24 hours
/// ```
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Seconds(pub Real);

/// Angular velocity measurement in rad/s.
///
/// Used for rotational motion, orbital angular velocity, and tether rotation.
///
/// # Examples
/// ```rust
/// use almagest::utils::RadiansPerSecond;
///
/// let earth_rotation = RadiansPerSecond(7.2921e-5);  // Earth's rotation rate
/// ```
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct RadiansPerSecond(pub Real);

impl MetersPerSecond {
    pub fn value(&self) -> Real {
        self.0
    }
}

impl Seconds {
    pub fn value(&self) -> Real {
        self.0
    }
}

impl RadiansPerSecond {
    pub fn value(&self) -> Real {
        self.0
    }
}

pub const G: MetersCubedByKilogramSecondsSquared = MetersCubedByKilogramSecondsSquared(6.6742e-11);

impl Pascals {
    pub fn to_newtons_per_meters_squared(&self) -> NewtonsPerMetersSquared {
        NewtonsPerMetersSquared(self.value())
    }

    pub fn value(&self) -> Real {
        self.0
    }
}

impl Div<KilogramsPerMetersCubed> for Pascals {
    type Output = MetersPerSecond;
    fn div(self, rhs: KilogramsPerMetersCubed) -> MetersPerSecond {
        MetersPerSecond(self.0 / rhs.0)
    }
}

impl Mul<Real> for Pascals {
    type Output = Self;
    fn mul(self, rhs: Real) -> Self::Output {
        Pascals(self.0 * rhs)
    }
}

impl Kilometers {
    pub const fn value(&self) -> Real {
        self.0
    }
}

impl From<Kilometers> for Meters {
    fn from(km: Kilometers) -> Self {
        Meters(km.value() * 1_000.0)
    }
}

impl Add for Kilometers {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Kilometers(self.0 + rhs.0)
    }
}

impl Meters {
    pub const ZERO: Self = Meters(0.0);

    pub const fn value(&self) -> Real {
        self.0
    }
}

impl From<Meters> for Kilometers {
    fn from(m: Meters) -> Self {
        Kilometers(m.value() / 1_000.0)
    }
}

impl Add for Meters {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Meters(self.0 + rhs.0)
    }
}

// Meters / Meters = dimensionless ratio
impl Div for Meters {
    type Output = Real;
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

// Scalar multiplication
impl Mul<Real> for Meters {
    type Output = Self;
    fn mul(self, rhs: Real) -> Self::Output {
        Meters(self.0 * rhs)
    }
}

// Scalar division
impl Div<Real> for Meters {
    type Output = Self;
    fn div(self, rhs: Real) -> Self::Output {
        Meters(self.0 / rhs)
    }
}

// Meters * Meters = MetersSquared (area)
impl Mul for Meters {
    type Output = MetersSquared;
    fn mul(self, rhs: Self) -> Self::Output {
        MetersSquared(self.0 * rhs.0)
    }
}

// Display implementations
impl Display for Meters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} m", self.0)
    }
}

impl Display for MetersSquared {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} m²", self.0)
    }
}

impl Display for MetersCubed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} m³", self.0)
    }
}

impl CentimetersSquared {
    pub const fn value(&self) -> Real {
        self.0
    }
}

// MetersSquared operations
impl MetersSquared {
    pub const fn value(self) -> Real {
        self.0
    }
}

impl From<MetersSquared> for CentimetersSquared {
    fn from(m2: MetersSquared) -> Self {
        CentimetersSquared(m2.value() * 10_000.0)
    }
}

impl From<CentimetersSquared> for MetersSquared {
    fn from(cm2: CentimetersSquared) -> Self {
        MetersSquared(cm2.value() * 1e-4)
    }
}

// MetersCubed operations
impl MetersCubed {
    pub const fn value(self) -> Real {
        self.0
    }
}

impl Add for MetersSquared {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        MetersSquared(self.0 + rhs.0)
    }
}

impl Sub for MetersSquared {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        MetersSquared(self.0 - rhs.0)
    }
}

impl Mul<Real> for MetersSquared {
    type Output = Self;
    fn mul(self, rhs: Real) -> Self::Output {
        MetersSquared(self.0 * rhs)
    }
}

impl Div<Real> for MetersSquared {
    type Output = Self;
    fn div(self, rhs: Real) -> Self::Output {
        MetersSquared(self.0 / rhs)
    }
}

// MetersSquared / Meters = Meters
impl Div<Meters> for MetersSquared {
    type Output = Meters;
    fn div(self, rhs: Meters) -> Self::Output {
        Meters(self.0 / rhs.0)
    }
}

// Meters * MetersSquared = MetersCubed
impl Mul<MetersSquared> for Meters {
    type Output = MetersCubed;
    fn mul(self, rhs: MetersSquared) -> Self::Output {
        MetersCubed(self.0 * rhs.0)
    }
}

// Real * Meters = Meters (commutative scalar multiplication)
impl Mul<Meters> for Real {
    type Output = Meters;
    fn mul(self, rhs: Meters) -> Self::Output {
        Meters(self * rhs.0)
    }
}

impl Sub for Meters {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Meters(self.0 - rhs.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Eccentricity(Real);

impl Eccentricity {
    pub fn new(value: Real) -> Result<Self, &'static str> {
        if value < 0.0 {
            Err("Eccentricity cannot be negative")
        } else {
            Ok(Eccentricity(value))
        }
    }

    pub fn value(&self) -> Real {
        self.0
    }
}

#[cfg(test)]
mod test_units {
    use super::*;
    use approx::assert_relative_eq;

    // === Basic Arithmetic Operations ===

    #[test]
    fn meters_addition() {
        let a = Meters(10.0);
        let b = Meters(5.0);
        assert_eq!(a + b, Meters(15.0));
    }

    #[test]
    fn meters_subtraction() {
        let a = Meters(10.0);
        let b = Meters(3.0);
        assert_eq!(a - b, Meters(7.0));
    }

    #[test]
    fn meters_scalar_multiplication() {
        let m = Meters(5.0);
        assert_eq!(m * 3.0, Meters(15.0));
    }

    #[test]
    fn meters_scalar_division() {
        let m = Meters(15.0);
        assert_eq!(m / 3.0, Meters(5.0));
    }

    #[test]
    fn meters_ratio_division() {
        let a = Meters(15.0);
        let b = Meters(3.0);
        assert_relative_eq!(a / b, 5.0, epsilon = 1e-10);
    }

    // === Dimensional Analysis Tests ===

    #[test]
    fn meters_multiplication_creates_area() {
        let length = Meters(4.0);
        let width = Meters(3.0);
        let area: MetersSquared = length * width;
        assert_eq!(area.value(), 12.0);
    }

    #[test]
    fn area_division_by_meters_gives_meters() {
        let area = MetersSquared(20.0);
        let width = Meters(4.0);
        let length: Meters = area / width;
        assert_eq!(length, Meters(5.0));
    }

    #[test]
    fn meters_times_area_gives_volume() {
        let height = Meters(2.0);
        let area = MetersSquared(10.0);
        let volume: MetersCubed = height * area;
        assert_eq!(volume.value(), 20.0);
    }

    #[test]
    fn area_arithmetic() {
        let a1 = MetersSquared(10.0);
        let a2 = MetersSquared(5.0);
        assert_eq!(a1 + a2, MetersSquared(15.0));
        assert_eq!(a1 - a2, MetersSquared(5.0));
        assert_eq!(a1 * 2.0, MetersSquared(20.0));
        assert_eq!(a1 / 2.0, MetersSquared(5.0));
    }

    // === Unit Conversion Tests ===

    #[test]
    fn meters_convert_to_km() {
        let m = Meters(1_000.0);
        let km = Kilometers(1.0);
        let expected: Kilometers = m.into();
        assert_eq!(expected, km);
    }

    #[test]
    fn meters_convert_precision() {
        let m = Meters(1_234.567);
        let km: Kilometers = m.into();
        assert_relative_eq!(km.0, 1.234567, epsilon = 1e-10);
    }

    // === Constants and Special Values ===

    #[test]
    fn meters_zero_constant() {
        assert_eq!(Meters::ZERO, Meters(0.0));
        assert_eq!(Meters::ZERO + Meters(5.0), Meters(5.0));
    }

    #[test]
    fn meters_value_accessor() {
        let m = Meters(42.0);
        assert_eq!(m.value(), 42.0);
    }

    // === Edge Cases and Error Conditions ===

    #[test]
    fn meters_with_infinity() {
        let inf = Meters(Real::INFINITY);
        let finite = Meters(10.0);
        assert!(inf.value().is_infinite());
        assert!((inf + finite).value().is_infinite());
    }

    #[test]
    fn meters_with_nan() {
        let nan = Meters(Real::NAN);
        assert!(nan.value().is_nan());
        // NaN propagates through operations
        assert!((nan + Meters(5.0)).value().is_nan());
    }

    #[test]
    fn meters_division_by_zero() {
        let m = Meters(10.0);
        let result = m / 0.0;
        assert!(result.value().is_infinite());
    }

    #[test]
    fn zero_divided_by_meters() {
        let zero = Meters(0.0);
        let divisor = Meters(5.0);
        assert_eq!(zero / divisor, 0.0);
    }

    // === Eccentricity Validation Tests ===

    #[test]
    fn eccentricity_valid_values() {
        assert!(Eccentricity::new(0.0).is_ok());
        assert!(Eccentricity::new(0.5).is_ok());
        assert!(Eccentricity::new(0.999).is_ok());
        assert!(Eccentricity::new(1.0).is_ok());
    }

    #[test]
    fn eccentricity_invalid_negative() {
        assert!(Eccentricity::new(-0.1).is_err());
        assert!(Eccentricity::new(-1.0).is_err());
    }

    #[test]
    fn eccentricity_value_accessor() {
        let e = Eccentricity::new(0.5).unwrap();
        assert_eq!(e.value(), 0.5);
    }

    // === Comparison and Ordering Tests ===

    #[test]
    fn meters_comparison() {
        let a = Meters(5.0);
        let b = Meters(10.0);
        let c = Meters(5.0);

        assert!(a < b);
        assert!(b > a);
        assert_eq!(a, c);
        assert!(a <= c);
        assert!(a >= c);
    }

    #[test]
    fn area_comparison() {
        let small = MetersSquared(5.0);
        let large = MetersSquared(10.0);

        assert!(small < large);
        assert!(large > small);
    }

    // === Mathematical Properties ===

    #[test]
    fn meters_associativity() {
        let a = Meters(2.0);
        let b = Meters(3.0);
        let c = Meters(4.0);

        // Addition associativity: (a + b) + c = a + (b + c)
        assert_eq!((a + b) + c, a + (b + c));
    }

    #[test]
    fn meters_commutativity() {
        let a = Meters(7.0);
        let b = Meters(11.0);

        // Addition commutativity: a + b = b + a
        assert_eq!(a + b, b + a);

        // Multiplication commutativity with dimensionality
        let area1: MetersSquared = a * b;
        let area2: MetersSquared = b * a;
        assert_eq!(area1, area2);
    }

    #[test]
    fn meters_distributivity() {
        let a = Meters(3.0);
        let b = Meters(4.0);
        let scalar = 2.0;

        // Scalar distributivity: k(a + b) = ka + kb
        assert_eq!(scalar * (a + b), scalar * a + scalar * b);
    }

    // === Real-world Scale Tests ===

    #[test]
    fn orbital_scale_calculations() {
        // Earth's radius
        let earth_radius = Meters(6_371_000.0);
        // ISS altitude
        let iss_altitude = Meters(408_000.0);
        let iss_orbit_radius = earth_radius + iss_altitude;

        assert_relative_eq!(iss_orbit_radius.value(), 6_779_000.0, epsilon = 1.0);

        // Check that we can compute orbital circumference (2πr)
        let circumference = iss_orbit_radius * (2.0 * PI);
        assert!(circumference.value() > 42_000_000.0); // ~42.6M meters
    }

    #[test]
    fn astronomical_scale_precision() {
        // Earth-Sun distance (1 AU)
        let au = Meters(149_597_870_700.0);
        let half_au = au / 2.0;

        assert_relative_eq!(half_au.value(), 74_798_935_350.0, epsilon = 1.0);
    }

    // === Display Implementation Tests ===
    // Note: Display tests removed to maintain no_std compatibility
    // Display trait implementations are still available for debugging
}
