use crate::materials::Material;
use crate::utils::{
    KilogramsPerMetersCubed, Meters, MetersCubedByKilogramSecondsSquared, MetersPerSecond, Pascals,
    RadiansPerSecond, Seconds, TAU,
};

/// Calculates the characteristic velocity for a space tether material.
///
/// The characteristic velocity is a measure of how fast stress waves propagate
/// through a tether material and represents the maximum theoretical velocity
/// that can be achieved by a space tether system. It's given by:
///
/// v = √(2σ/ρ)
///
/// where:
/// - σ is the tensile strength (Pa)
/// - ρ is the material density (kg/m³)
///
/// This is derived from the speed of sound in the material and represents
/// the velocity limit for tether deployment and orbital mechanics applications.
///
/// # Arguments
/// * `tensile_strength` - Ultimate tensile strength of the tether material in Pascals
/// * `density` - Material density in kg/m³
///
/// # Returns
/// The characteristic velocity in m/s
///
/// # Example
/// ```
/// use almagest::tethers::{characteristic_velocity, characteristic_velocity_for_material};
/// use almagest::materials::fibers;
/// use almagest::utils::{Pascals, KilogramsPerMetersCubed};
///
/// // Using individual parameters
/// let pbo_velocity = characteristic_velocity(
///     Pascals(5.9e9),           // 5.9 GPa tensile strength
///     KilogramsPerMetersCubed(1340.0)  // 1340 kg/m³ density
/// );
///
/// // Or using predefined material
/// let pbo_velocity2 = characteristic_velocity_for_material(&fibers::PBO);
/// // Both results are approximately 2967 m/s
/// ```
pub fn characteristic_velocity(
    tensile_strength: Pascals,
    density: KilogramsPerMetersCubed,
) -> Result<MetersPerSecond, &'static str> {
    // Validate inputs for physical reasonableness
    if tensile_strength.value() <= 0.0 {
        return Err("Tensile strength must be positive");
    }
    if density.0 <= 0.0 {
        return Err("Density must be positive");
    }
    // Check for reasonable upper bounds (strongest known materials)
    if tensile_strength.value() > 200e9 {
        // 200 GPa (theoretical carbon nanotube limit)
        return Err("Tensile strength exceeds known material limits");
    }
    if density.0 > 50_000.0 {
        // 50 g/cm³ (denser than most metals)
        return Err("Density exceeds reasonable material limits");
    }

    let val = libm::sqrt(((tensile_strength * 2.0) / density).value());
    Ok(MetersPerSecond(val))
}

/// Calculates the characteristic velocity for a predefined material.
///
/// This is a convenience function that uses the material properties
/// from the materials database.
///
/// # Arguments
/// * `material` - A reference to a Material struct containing tensile strength and density
///
/// # Returns
/// The characteristic velocity in m/s, or an error if the material properties are invalid
///
/// # Example
/// ```
/// use almagest::tethers::characteristic_velocity_for_material;
/// use almagest::materials::fibers;
///
/// let kevlar_velocity = characteristic_velocity_for_material(&fibers::KEVLAR_49)
///     .expect("Valid Kevlar properties");
/// ```
pub fn characteristic_velocity_for_material(
    material: &Material,
) -> Result<MetersPerSecond, &'static str> {
    characteristic_velocity(material.tensile_strength, material.density)
}

/// Calculates the orbital velocity for a momentum exchange tether at a given radius.
///
/// For a circular orbit, the orbital velocity is given by:
/// v = √(μ/r)
///
/// where:
/// - μ is the standard gravitational parameter (GM)
/// - r is the orbital radius from the center of the central body
///
/// # Arguments
/// * `radius` - Orbital radius from the center of the central body
/// * `gravitational_parameter` - Standard gravitational parameter μ = GM
///
/// # Returns
/// The orbital velocity in m/s
///
/// # Example
/// ```
/// use almagest::tethers::momentum_exchange_orbital_velocity;
/// use almagest::utils::{Meters, MetersCubedByKilogramSecondsSquared};
///
/// // Earth's standard gravitational parameter
/// let earth_mu = MetersCubedByKilogramSecondsSquared(3.986004418e14);
/// let earth_radius = Meters(6.371e6);
/// let leo_altitude = Meters(400e3);
/// let orbit_radius = Meters(earth_radius.0 + leo_altitude.0);
///
/// let velocity = momentum_exchange_orbital_velocity(orbit_radius, earth_mu)
///     .expect("Valid orbital parameters");
/// // Result: approximately 7,669 m/s for 400km altitude
/// ```
pub fn momentum_exchange_orbital_velocity(
    radius: Meters,
    gravitational_parameter: MetersCubedByKilogramSecondsSquared,
) -> Result<MetersPerSecond, &'static str> {
    if radius.0 <= 0.0 {
        return Err("Orbital radius must be positive");
    }
    if gravitational_parameter.0 <= 0.0 {
        return Err("Gravitational parameter must be positive");
    }

    let velocity = libm::sqrt(gravitational_parameter.0 / radius.0);
    Ok(MetersPerSecond(velocity))
}

/// Calculates the orbital period for a momentum exchange tether at a given radius.
///
/// For a circular orbit, the orbital period is given by Kepler's third law:
/// T = 2π√(r³/μ)
///
/// where:
/// - r is the orbital radius
/// - μ is the standard gravitational parameter (GM)
///
/// # Arguments
/// * `radius` - Orbital radius from the center of the central body
/// * `gravitational_parameter` - Standard gravitational parameter μ = GM
///
/// # Returns
/// The orbital period in seconds
///
/// # Example
/// ```
/// use almagest::tethers::momentum_exchange_orbital_period;
/// use almagest::utils::{Meters, MetersCubedByKilogramSecondsSquared};
///
/// let earth_mu = MetersCubedByKilogramSecondsSquared(3.986004418e14);
/// let earth_radius = Meters(6.371e6);
/// let geostationary_altitude = Meters(35.786e6);
/// let geo_radius = Meters(earth_radius.0 + geostationary_altitude.0);
///
/// let period = momentum_exchange_orbital_period(geo_radius, earth_mu)
///     .expect("Valid orbital parameters");
/// // Result: approximately 86,164 seconds (23h 56m 4s)
/// ```
pub fn momentum_exchange_orbital_period(
    radius: Meters,
    gravitational_parameter: MetersCubedByKilogramSecondsSquared,
) -> Result<Seconds, &'static str> {
    if radius.0 <= 0.0 {
        return Err("Orbital radius must be positive");
    }
    if gravitational_parameter.0 <= 0.0 {
        return Err("Gravitational parameter must be positive");
    }

    let r_cubed = libm::pow(radius.0, 3.0);
    let period = TAU * libm::sqrt(r_cubed / gravitational_parameter.0);
    Ok(Seconds(period))
}

/// Calculates the angular velocity for a momentum exchange tether at a given radius.
///
/// The angular velocity is given by:
/// ω = √(μ/r³)
///
/// where:
/// - μ is the standard gravitational parameter (GM)
/// - r is the orbital radius
///
/// # Arguments
/// * `radius` - Orbital radius from the center of the central body
/// * `gravitational_parameter` - Standard gravitational parameter μ = GM
///
/// # Returns
/// The angular velocity in rad/s
///
/// # Example
/// ```
/// use almagest::tethers::momentum_exchange_angular_velocity;
/// use almagest::utils::{Meters, MetersCubedByKilogramSecondsSquared};
///
/// let earth_mu = MetersCubedByKilogramSecondsSquared(3.986004418e14);
/// let earth_radius = Meters(6.371e6);
/// let leo_altitude = Meters(400e3);
/// let orbit_radius = Meters(earth_radius.0 + leo_altitude.0);
///
/// let angular_vel = momentum_exchange_angular_velocity(orbit_radius, earth_mu)
///     .expect("Valid orbital parameters");
/// ```
pub fn momentum_exchange_angular_velocity(
    radius: Meters,
    gravitational_parameter: MetersCubedByKilogramSecondsSquared,
) -> Result<RadiansPerSecond, &'static str> {
    if radius.0 <= 0.0 {
        return Err("Orbital radius must be positive");
    }
    if gravitational_parameter.0 <= 0.0 {
        return Err("Gravitational parameter must be positive");
    }

    let angular_velocity = libm::sqrt(gravitational_parameter.0 / libm::pow(radius.0, 3.0));
    Ok(RadiansPerSecond(angular_velocity))
}

/// Calculates the momentum exchange efficiency for a tether system.
///
/// This represents the efficiency of momentum transfer for a rotating tether
/// system. The efficiency depends on the tether material properties and
/// the orbital parameters.
///
/// For a momentum exchange tether, the efficiency is related to the ratio
/// of the characteristic velocity to the orbital velocity:
/// η = min(1.0, v_char / v_orbital)
///
/// # Arguments
/// * `material` - Tether material properties
/// * `radius` - Orbital radius from the center of the central body
/// * `gravitational_parameter` - Standard gravitational parameter μ = GM
///
/// # Returns
/// The momentum exchange efficiency (0.0 to 1.0)
///
/// # Example
/// ```
/// use almagest::tethers::momentum_exchange_efficiency;
/// use almagest::materials::fibers;
/// use almagest::utils::{Meters, MetersCubedByKilogramSecondsSquared};
///
/// let earth_mu = MetersCubedByKilogramSecondsSquared(3.986004418e14);
/// let earth_radius = Meters(6.371e6);
/// let leo_altitude = Meters(400e3);
/// let orbit_radius = Meters(earth_radius.0 + leo_altitude.0);
///
/// let efficiency = momentum_exchange_efficiency(&fibers::PBO, orbit_radius, earth_mu)
///     .expect("Valid parameters");
/// ```
pub fn momentum_exchange_efficiency(
    material: &Material,
    radius: Meters,
    gravitational_parameter: MetersCubedByKilogramSecondsSquared,
) -> Result<f64, &'static str> {
    let char_velocity = characteristic_velocity_for_material(material)?;
    let orbital_velocity = momentum_exchange_orbital_velocity(radius, gravitational_parameter)?;

    // Efficiency is capped at 1.0 (100%)
    let efficiency = (char_velocity.0 / orbital_velocity.0).min(1.0);
    Ok(efficiency)
}

/// Calculates the spin rate for a momentum exchange tether based on Moravec 1977.
///
/// According to Hans Moravec's 1977 analysis, a momentum exchange tether with
/// a diameter equal to 1/3 of the central body's diameter will "touch down"
/// (have zero velocity relative to the surface) 6 times per orbit.
///
/// For smaller tethers, the spin rate scales approximately linearly with
/// the diameter ratio. This function calculates the appropriate spin rate
/// as a multiple of the orbital angular velocity.
///
/// # Arguments
/// * `tether_length` - Length of the tether from center to tip
/// * `central_body_radius` - Radius of the central body (e.g., Earth radius)
///
/// # Returns
/// The spin rate multiplier (ratio of tether spin to orbital angular velocity)
///
/// # Example
/// ```
/// use almagest::tethers::momentum_exchange_spin_rate;
/// use almagest::utils::Meters;
///
/// let earth_radius = Meters(6.371e6);
/// let tether_length = Meters(200e3);  // 200 km tether
///
/// let spin_multiplier = momentum_exchange_spin_rate(tether_length, earth_radius)
///     .expect("Valid tether parameters");
/// // Result: ~1.1x for a small tether
/// ```
pub fn momentum_exchange_spin_rate(
    tether_length: Meters,
    central_body_radius: Meters,
) -> Result<f64, &'static str> {
    if tether_length.0 <= 0.0 {
        return Err("Tether length must be positive");
    }
    if central_body_radius.0 <= 0.0 {
        return Err("Central body radius must be positive");
    }

    // Calculate tether diameter and central body diameter
    let tether_diameter = tether_length.0 * 2.0;
    let central_body_diameter = central_body_radius.0 * 2.0;
    let diameter_ratio = tether_diameter / central_body_diameter;

    // Moravec 1977: A tether with diameter = 1/3 of central body diameter
    // touches down 6 times per orbit. For smaller tethers, scale linearly.
    // Formula: spin_rate = 1 + (diameter_ratio / (1/3)) * 5
    // This gives: 1x for tiny tethers, 6x for diameter_ratio = 1/3
    let moravec_reference_ratio = 1.0 / 3.0; // 1/3 diameter ratio
    let spin_multiplier = 1.0 + (diameter_ratio / moravec_reference_ratio) * 5.0;

    // Cap at reasonable maximum (don't go beyond Moravec's model)
    let max_spin_rate = 10.0; // Reasonable upper limit
    Ok(spin_multiplier.min(max_spin_rate))
}

// 5.9 GPa for stress
// 1,340 km/m3

#[cfg(test)]
mod tests {
    use crate::materials::{fibers, metals};
    use approx::assert_relative_eq;

    use crate::tethers::*;

    #[test]
    fn test_char_velo() {
        let pbo_char_vel =
            characteristic_velocity(Pascals(5.9e9), KilogramsPerMetersCubed(1_340.0))
                .expect("Valid PBO parameters");
        assert_relative_eq!(pbo_char_vel.0, 2967.49, epsilon = 0.01);
    }

    #[test]
    fn test_char_velo_validation() {
        // Test negative tensile strength
        assert!(
            characteristic_velocity(Pascals(-1000.0), KilogramsPerMetersCubed(1000.0)).is_err()
        );

        // Test negative density
        assert!(characteristic_velocity(Pascals(1e9), KilogramsPerMetersCubed(-1000.0)).is_err());

        // Test zero values
        assert!(characteristic_velocity(Pascals(0.0), KilogramsPerMetersCubed(1000.0)).is_err());
        assert!(characteristic_velocity(Pascals(1e9), KilogramsPerMetersCubed(0.0)).is_err());

        // Test unreasonably high values
        assert!(characteristic_velocity(Pascals(300e9), KilogramsPerMetersCubed(1000.0)).is_err());
        assert!(characteristic_velocity(Pascals(1e9), KilogramsPerMetersCubed(100_000.0)).is_err());
    }

    #[test]
    fn test_char_velo_with_materials() {
        // Test with PBO using the materials database
        let pbo_velocity =
            characteristic_velocity_for_material(&fibers::PBO).expect("Valid PBO material");
        assert_relative_eq!(pbo_velocity.0, 2967.49, epsilon = 0.01);

        // Test with Kevlar
        let kevlar_velocity = characteristic_velocity_for_material(&fibers::KEVLAR_49)
            .expect("Valid Kevlar material");
        // sqrt((2 * 3.6e9) / 1440) ≈ 2236 m/s
        assert_relative_eq!(kevlar_velocity.0, 2236.0, epsilon = 10.0);

        // Verify PBO has higher characteristic velocity than Kevlar
        assert!(pbo_velocity.0 > kevlar_velocity.0);
    }

    #[test]
    fn test_material_specific_strength_correlation() {
        // Materials with higher specific strength should generally have higher characteristic velocity
        let pbo = characteristic_velocity_for_material(&fibers::PBO).unwrap();
        let aluminum = characteristic_velocity_for_material(&metals::ALUMINUM_6061_T6).unwrap();
        let uhmwpe = characteristic_velocity_for_material(&fibers::UHMWPE).unwrap();

        // High-performance fibers should outperform metals significantly
        assert!(pbo.0 > aluminum.0);
        assert!(uhmwpe.0 > aluminum.0);

        // PBO should be among the best performers
        assert!(pbo.0 > uhmwpe.0);
    }

    #[test]
    fn test_momentum_exchange_orbital_velocity() {
        // Earth's standard gravitational parameter
        let earth_mu = MetersCubedByKilogramSecondsSquared(3.986004418e14);

        // Test LEO orbital velocity (400km altitude)
        let earth_radius = Meters(6.371e6);
        let leo_altitude = Meters(400e3);
        let leo_radius = Meters(earth_radius.0 + leo_altitude.0);

        let leo_velocity =
            momentum_exchange_orbital_velocity(leo_radius, earth_mu).expect("Valid LEO parameters");

        // LEO velocity should be approximately 7,669 m/s
        assert_relative_eq!(leo_velocity.0, 7669.0, epsilon = 10.0);

        // Test geostationary orbital velocity
        let geo_altitude = Meters(35.786e6);
        let geo_radius = Meters(earth_radius.0 + geo_altitude.0);

        let geo_velocity =
            momentum_exchange_orbital_velocity(geo_radius, earth_mu).expect("Valid GEO parameters");

        // GEO velocity should be approximately 3,074 m/s
        assert_relative_eq!(geo_velocity.0, 3074.0, epsilon = 10.0);

        // Higher orbits should have lower velocities
        assert!(leo_velocity.0 > geo_velocity.0);
    }

    #[test]
    fn test_momentum_exchange_orbital_velocity_validation() {
        let earth_mu = MetersCubedByKilogramSecondsSquared(3.986004418e14);

        // Test negative radius
        assert!(momentum_exchange_orbital_velocity(Meters(-1000.0), earth_mu).is_err());

        // Test zero radius
        assert!(momentum_exchange_orbital_velocity(Meters(0.0), earth_mu).is_err());

        // Test negative gravitational parameter
        assert!(
            momentum_exchange_orbital_velocity(
                Meters(7e6),
                MetersCubedByKilogramSecondsSquared(-1e14)
            )
            .is_err()
        );

        // Test zero gravitational parameter
        assert!(
            momentum_exchange_orbital_velocity(
                Meters(7e6),
                MetersCubedByKilogramSecondsSquared(0.0)
            )
            .is_err()
        );
    }

    #[test]
    fn test_momentum_exchange_orbital_period() {
        let earth_mu = MetersCubedByKilogramSecondsSquared(3.986004418e14);
        let earth_radius = Meters(6.371e6);

        // Test LEO orbital period (400km altitude)
        let leo_altitude = Meters(400e3);
        let leo_radius = Meters(earth_radius.0 + leo_altitude.0);

        let leo_period =
            momentum_exchange_orbital_period(leo_radius, earth_mu).expect("Valid LEO parameters");

        // LEO period should be approximately 5,543 seconds (~92.4 minutes)
        assert_relative_eq!(leo_period.0, 5543.0, epsilon = 10.0);

        // Test geostationary orbital period
        let geo_altitude = Meters(35.786e6);
        let geo_radius = Meters(earth_radius.0 + geo_altitude.0);

        let geo_period =
            momentum_exchange_orbital_period(geo_radius, earth_mu).expect("Valid GEO parameters");

        // GEO period should be approximately 86,164 seconds (23h 56m 4s)
        assert_relative_eq!(geo_period.0, 86164.0, epsilon = 100.0);

        // Higher orbits should have longer periods
        assert!(geo_period.0 > leo_period.0);
    }

    #[test]
    fn test_momentum_exchange_angular_velocity() {
        let earth_mu = MetersCubedByKilogramSecondsSquared(3.986004418e14);
        let earth_radius = Meters(6.371e6);
        let leo_altitude = Meters(400e3);
        let leo_radius = Meters(earth_radius.0 + leo_altitude.0);

        let angular_vel =
            momentum_exchange_angular_velocity(leo_radius, earth_mu).expect("Valid LEO parameters");

        // Angular velocity should be approximately 0.00113 rad/s for LEO
        assert_relative_eq!(angular_vel.0, 0.00113, epsilon = 0.0001);

        // Test relationship: ω = v/r
        let orbital_vel = momentum_exchange_orbital_velocity(leo_radius, earth_mu).unwrap();
        let expected_angular_vel = orbital_vel.0 / leo_radius.0;
        assert_relative_eq!(angular_vel.0, expected_angular_vel, epsilon = 1e-10);
    }

    #[test]
    fn test_momentum_exchange_efficiency() {
        let earth_mu = MetersCubedByKilogramSecondsSquared(3.986004418e14);
        let earth_radius = Meters(6.371e6);
        let leo_altitude = Meters(400e3);
        let leo_radius = Meters(earth_radius.0 + leo_altitude.0);

        // Test efficiency with PBO material
        let pbo_efficiency = momentum_exchange_efficiency(&fibers::PBO, leo_radius, earth_mu)
            .expect("Valid PBO efficiency calculation");

        // PBO has characteristic velocity ~2967 m/s, LEO orbital velocity ~7669 m/s
        // So efficiency should be ~0.387 (38.7%)
        assert_relative_eq!(pbo_efficiency, 0.387, epsilon = 0.01);

        // Efficiency should be between 0 and 1
        assert!(pbo_efficiency >= 0.0 && pbo_efficiency <= 1.0);

        // Test efficiency with weaker material (aluminum)
        let aluminum_efficiency =
            momentum_exchange_efficiency(&metals::ALUMINUM_6061_T6, leo_radius, earth_mu)
                .expect("Valid aluminum efficiency calculation");

        // Aluminum should have lower efficiency than PBO
        assert!(aluminum_efficiency < pbo_efficiency);
        assert!(aluminum_efficiency >= 0.0 && aluminum_efficiency <= 1.0);
    }

    #[test]
    fn test_momentum_exchange_kepler_third_law() {
        // Verify that our period calculation follows Kepler's third law: T² ∝ r³
        let earth_mu = MetersCubedByKilogramSecondsSquared(3.986004418e14);

        let radius1 = Meters(7e6);
        let radius2 = Meters(14e6); // Double the radius

        let period1 = momentum_exchange_orbital_period(radius1, earth_mu).unwrap();
        let period2 = momentum_exchange_orbital_period(radius2, earth_mu).unwrap();

        // T₂²/T₁² should equal (r₂/r₁)³ = 2³ = 8
        let period_ratio_squared = libm::pow(period2.0 / period1.0, 2.0);
        let radius_ratio_cubed = libm::pow(radius2.0 / radius1.0, 3.0);

        assert_relative_eq!(period_ratio_squared, radius_ratio_cubed, epsilon = 1e-10);
    }

    #[test]
    fn test_momentum_exchange_vis_viva_equation() {
        // Verify that our velocity calculation follows the vis-viva equation for circular orbits
        let earth_mu = MetersCubedByKilogramSecondsSquared(3.986004418e14);

        let test_radii = [
            Meters(6.571e6), // LEO
            Meters(1.2e7),   // Medium orbit
            Meters(4.2e7),   // GEO
        ];

        for radius in &test_radii {
            let velocity = momentum_exchange_orbital_velocity(*radius, earth_mu).unwrap();
            let angular_velocity = momentum_exchange_angular_velocity(*radius, earth_mu).unwrap();

            // For circular orbits: v = ωr
            let velocity_from_angular = angular_velocity.0 * radius.0;
            assert_relative_eq!(velocity.0, velocity_from_angular, epsilon = 1e-10);

            // For circular orbits: v² = μ/r
            let velocity_squared = libm::pow(velocity.0, 2.0);
            let expected_velocity_squared = earth_mu.0 / radius.0;
            assert_relative_eq!(velocity_squared, expected_velocity_squared, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_momentum_exchange_spin_rate() {
        let earth_radius = Meters(6.371e6);

        // Test small tether (should have spin rate close to 1x)
        let small_tether = Meters(50e3); // 50 km
        let small_spin = momentum_exchange_spin_rate(small_tether, earth_radius).unwrap();
        assert!(small_spin >= 1.0 && small_spin < 2.0);

        // Test medium tether
        let medium_tether = Meters(200e3); // 200 km
        let medium_spin = momentum_exchange_spin_rate(medium_tether, earth_radius).unwrap();
        assert!(medium_spin > small_spin);
        assert!(medium_spin < 3.0);

        // Test large tether (approaching Moravec's 1/3 diameter case)
        let large_tether = Meters(1.05e6); // ~1000 km (diameter ~2000 km ≈ 1/6 Earth diameter)
        let large_spin = momentum_exchange_spin_rate(large_tether, earth_radius).unwrap();
        assert!(large_spin > medium_spin);

        // Test Moravec reference case: tether diameter = 1/3 Earth diameter
        // Tether length = Earth diameter / 6 = Earth radius / 3
        let moravec_tether = Meters(earth_radius.0 / 3.0);
        let moravec_spin = momentum_exchange_spin_rate(moravec_tether, earth_radius).unwrap();
        assert_relative_eq!(moravec_spin, 6.0, epsilon = 0.1);
    }

    #[test]
    fn test_momentum_exchange_spin_rate_validation() {
        let earth_radius = Meters(6.371e6);

        // Test negative tether length
        assert!(momentum_exchange_spin_rate(Meters(-1000.0), earth_radius).is_err());

        // Test zero tether length
        assert!(momentum_exchange_spin_rate(Meters(0.0), earth_radius).is_err());

        // Test negative central body radius
        assert!(momentum_exchange_spin_rate(Meters(1000.0), Meters(-1e6)).is_err());

        // Test zero central body radius
        assert!(momentum_exchange_spin_rate(Meters(1000.0), Meters(0.0)).is_err());
    }

    #[test]
    fn test_momentum_exchange_spin_rate_scaling() {
        let earth_radius = Meters(6.371e6);

        // Test that larger tethers always have higher spin rates
        let lengths = [
            Meters(50e3),  // 50 km
            Meters(100e3), // 100 km
            Meters(200e3), // 200 km
            Meters(500e3), // 500 km
        ];

        let mut prev_spin = 0.0;
        for length in &lengths {
            let spin = momentum_exchange_spin_rate(*length, earth_radius).unwrap();
            assert!(
                spin > prev_spin,
                "Spin rate should increase with tether length"
            );
            assert!(spin >= 1.0, "Spin rate should be at least 1x orbital rate");
            prev_spin = spin;
        }
    }
}
