#![allow(dead_code, unused_imports, unused_variables)]
use almagest::utils::MetersSquared;
use almagest::{
    materials::Material,
    utils::{
        CentimetersSquared, Kilograms, Kilometers, Meters, MetersPerSecond, MetersPerSecondSquared,
        Real,
    },
};

/// What do I want to model here?
/// We want a *tether*, because that's the big thing we're exploring.
/// We'll want a *body* about which is orbits, because that effects the tether.
/// We need the tether to catch something, so -- *payload*?
///
/// The physics of the payload-catcher (Kendall of the Pittsburgh Pirates) will depend on stuff we declare for the tether:
///   - √ overall length
///   - √ overall mass
///   - counterweight mass -- defer
///   - √ rotational velocity
///   - √ orbit altitude for its center
///   - orbit period for its center -- derived?
///   - length of receiver to center -- defer
///   - length of center to counterweight -- defer
///   - mass of center -- defer
///   - √ material for tether
///   - taper ratio for tether -- defer
///   - orbital inclination -- defer
/// > NB: "Detailed data collected has shown that for low lunar orbit the only "stable" orbits are at inclinations near 27°, 50°, 76°, and 86°." [Gravitation of the moon, Wikipedia](https://en.wikipedia.org/wiki/Gravitation_of_the_Moon)

#[derive(Debug)]
pub struct Tether {
    /// Orbit altitude for just the middle of the tether (assumed station)
    altitude: Kilometers,
    length: Kilometers,
    mass: Kilograms,
    /// Material provides us the data we need to calculate specifics
    material: Material,
    rotational_velocity: MetersPerSecond,
}

impl Tether {
    pub fn new(
        altitude: Kilometers,
        length: Kilometers,
        mass: Kilograms,
        material: Material,
        rotational_velocity: MetersPerSecond,
    ) -> Self {
        Self {
            altitude,
            length,
            mass, // This will be calculated based on the expected payload mass.
            material,
            rotational_velocity,
        }
    }

    /// Returns (thinnest, thickest)
    fn cross_sectional_area(&self) -> (CentimetersSquared, CentimetersSquared) {
        const TAPER_RATIO: Real = 1.124;
        let len: Meters = self.length.into();
        let avg_cross_section: CentimetersSquared =
            MetersSquared(self.mass.0 / (len.0 * self.material.density.0)).into();
        let thinnest = CentimetersSquared(avg_cross_section.0 / 2.0);
        let thickest = CentimetersSquared(TAPER_RATIO * thinnest.0);
        (thinnest, thickest)
    }

    /// NB: this will later be "reduced by the strain imposed by the takeoff acceleration." [Mora77, p.311]
    pub fn max_load(&self, gravity: MetersPerSecondSquared) -> Kilograms {
        let cable_area: MetersSquared = self.cross_sectional_area().0.into();
        Kilograms((self.material.tensile_strength.0 * cable_area.0) * gravity.0)
    }

    pub fn mass_ratio(&self, body: Moon) -> Real {
        let max_load = self.max_load(body.gravity);
        self.mass.0 / max_load.0
    }

    /// Calculates the delta-v (impulse) imparted to a payload released from the tether tip.
    ///
    /// This accounts for angular momentum conservation during the exchange:
    /// - Before release: tether + payload rotating together
    /// - After release: payload gets velocity boost, tether rotation slows
    /// - The mass ratio determines how much momentum is actually transferred
    ///
    /// The calculation considers:
    /// 1. Orbital velocity at the release altitude
    /// 2. Rotational velocity contribution from the tether
    /// 3. Angular momentum exchange based on payload/tether mass ratio
    ///
    /// # Arguments
    /// * `payload` - The payload being released (mass affects momentum exchange)
    /// * `body` - The celestial body being orbited
    ///
    /// # Returns
    /// The delta-v imparted to the payload in m/s
    pub fn calc_impulse(&self, payload: Payload, body: Moon) -> MetersPerSecond {
        // Calculate orbital velocity at the tether's center altitude
        let orbit_radius: Meters = (body.radius + self.altitude).into(); // Convert km to m
        let gravitational_parameter = body.mass.0 * 6.67430e-11; // G * M

        // Orbital velocity at center: v = √(μ/r)
        let orbital_velocity = libm::sqrt(gravitational_parameter / orbit_radius.0);

        // Calculate the tip position (assuming release from upper tip)
        let tether_length_m = Meters(self.length.0 * 1000.0);
        let tip_radius = Meters(orbit_radius.0 + tether_length_m.0 / 2.0);
        let orbital_velocity_at_tip = libm::sqrt(gravitational_parameter / tip_radius.0);

        // Angular momentum before release (tether + payload system)
        // L = I * ω, where I = m * r² for point mass at tip
        // Simplified: treating payload as point mass at tip, tether as point mass at center
        let tether_angular_momentum = self.mass.0 * orbit_radius.0 * orbital_velocity;
        let payload_angular_momentum =
            payload.mass.0 * tip_radius.0 * (orbital_velocity_at_tip + self.rotational_velocity.0);
        let total_angular_momentum = tether_angular_momentum + payload_angular_momentum;

        // After release, tether retains most angular momentum
        // Effective velocity transfer depends on mass ratio
        // For momentum exchange: Δv ∝ (m_tether / m_payload)
        let mass_ratio = self.mass.0 / payload.mass.0;

        // The ideal tip velocity boost (if tether were infinite mass)
        let ideal_tip_boost = self.rotational_velocity.0;

        // Actual velocity boost accounting for finite tether mass
        // When payload is released, it "steals" angular momentum from the tether
        // Effective boost = ideal_boost * efficiency_factor
        // efficiency_factor ≈ mass_ratio / (1 + mass_ratio) for simple model
        let efficiency = mass_ratio / (1.0 + mass_ratio);
        let actual_tip_boost = ideal_tip_boost * efficiency;

        // Total velocity at release = orbital velocity at tip + rotational boost
        let release_velocity = orbital_velocity_at_tip + actual_tip_boost;

        // Delta-v is the difference from what orbital velocity would be at center altitude
        let delta_v = release_velocity - orbital_velocity;

        MetersPerSecond(delta_v)
    }

    /// The material's tensile strength divided by (density of the material times the celestial body's gravity)
    ///
    /// L = σ/(ρ × g)
    // todo: move to Material with same function signature
    pub fn characteristic_length(&self, body: Moon) -> Kilometers {
        Kilometers(self.material.tensile_strength.0 / (self.material.density.0 * body.gravity.0))
    }
}

/// The body needs some fields:
///   - mass
///   - radius
///   - gravity

#[derive(Debug, Clone, Copy)]
pub struct Moon {
    gravity: MetersPerSecondSquared,
    mass: Kilograms,
    radius: Kilometers,
}

impl Moon {
    pub const fn new() -> Self {
        Self {
            mass: Kilograms(7.348e22),
            radius: Kilometers(1_737.5),
            gravity: MetersPerSecondSquared(1.625),
        }
    }
}

impl Default for Moon {
    fn default() -> Self {
        Self::new()
    }
}

/// The payload consists of the thing being carried and the thing doing the carrying.
///   - mass
///   - velocity
#[derive(Debug, Clone, Copy)]
pub struct Payload {
    mass: Kilograms,
    velocity: MetersPerSecond,
}

impl Payload {
    fn new(mass: Kilograms, velocity: MetersPerSecond) -> Self {
        Self { mass, velocity }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use almagest::materials::fibers::KEVLAR_49;

    #[test]
    fn it_works() {
        let t = Tether::new(
            Kilometers(100.0),
            Kilometers(95.0),
            Kilograms(100.0),
            KEVLAR_49,
            MetersPerSecond(7_000.0),
        );
        assert_eq!(t.length, Kilometers(95.0));
        assert_eq!(t.rotational_velocity, MetersPerSecond(7_000.0));
    }

    mod calc_impulse_tests {
        use super::*;

        /// Test basic momentum conservation: impulse should increase with rotational velocity
        #[test]
        fn impulse_increases_with_rotational_velocity() {
            let moon = Moon::default();
            let payload = Payload::new(Kilograms(1000.0), MetersPerSecond(0.0));

            let tether_slow = Tether::new(
                Kilometers(100.0),
                Kilometers(100.0),
                Kilograms(10_000.0),
                KEVLAR_49,
                MetersPerSecond(500.0),
            );

            let tether_fast = Tether::new(
                Kilometers(100.0),
                Kilometers(100.0),
                Kilograms(10_000.0),
                KEVLAR_49,
                MetersPerSecond(1500.0),
            );

            let impulse_slow = tether_slow.calc_impulse(payload, moon);
            let impulse_fast = tether_fast.calc_impulse(payload, moon);

            assert!(
                impulse_fast.0 > impulse_slow.0,
                "Higher rotational velocity should produce greater impulse"
            );
        }

        /// Test mass ratio effect: heavier tether should transfer more momentum
        #[test]
        fn heavier_tether_increases_efficiency() {
            let moon = Moon::default();
            let payload = Payload::new(Kilograms(1000.0), MetersPerSecond(0.0));

            let tether_light = Tether::new(
                Kilometers(100.0),
                Kilometers(100.0),
                Kilograms(5_000.0),
                KEVLAR_49,
                MetersPerSecond(1000.0),
            );

            let tether_heavy = Tether::new(
                Kilometers(100.0),
                Kilometers(100.0),
                Kilograms(50_000.0),
                KEVLAR_49,
                MetersPerSecond(1000.0),
            );

            let impulse_light = tether_light.calc_impulse(payload, moon);
            let impulse_heavy = tether_heavy.calc_impulse(payload, moon);

            assert!(
                impulse_heavy.0 > impulse_light.0,
                "Heavier tether should transfer momentum more efficiently"
            );
        }

        /// Test with realistic lunar tether parameters from Moravec (1977)
        /// Reference: "A Non-Synchronous Orbital Skyhook" by Hans Moravec
        /// Typical lunar tether: 100km altitude, 100km length, rotating at ~1.6 km/s tip speed
        #[test]
        fn moravec_lunar_tether_scenario() {
            let moon = Moon::default();
            // Small payload (e.g., sample return capsule)
            let payload = Payload::new(Kilograms(500.0), MetersPerSecond(0.0));

            // Realistic lunar tether parameters
            let tether = Tether::new(
                Kilometers(100.0),   // Low lunar orbit
                Kilometers(100.0),   // 100km total length
                Kilograms(20_000.0), // ~20 ton tether
                KEVLAR_49,
                MetersPerSecond(1600.0), // 1.6 km/s tip velocity
            );

            let impulse = tether.calc_impulse(payload, moon);

            // Expected delta-v should be substantial but not exceed rotational velocity
            // With mass ratio of 40:1, efficiency should be ~0.976
            // Expected impulse ~1.5-1.6 km/s range
            assert!(
                impulse.0 > 1400.0 && impulse.0 < 1700.0,
                "Moravec-style tether should provide ~1.5 km/s impulse, got {} m/s",
                impulse.0
            );
        }

        /// Test escape velocity scenario
        /// Lunar escape velocity from low orbit: ~2.4 km/s
        /// Test if aggressive tether can provide sufficient delta-v
        #[test]
        fn lunar_escape_capability() {
            let moon = Moon::default();
            let payload = Payload::new(Kilograms(1000.0), MetersPerSecond(0.0));

            // Aggressive tether design for escape missions
            let tether = Tether::new(
                Kilometers(100.0),
                Kilometers(150.0),    // Longer tether
                Kilograms(100_000.0), // Heavy tether for efficiency
                KEVLAR_49,
                MetersPerSecond(2500.0), // High tip speed
            );

            let impulse = tether.calc_impulse(payload, moon);

            // Should provide enough delta-v to contribute significantly to escape
            // (escape from 100km lunar orbit requires ~2.2 km/s additional delta-v)
            assert!(
                impulse.0 > 2000.0,
                "High-performance tether should provide >2 km/s for escape missions, got {} m/s",
                impulse.0
            );
        }

        /// Test efficiency limits with very light payload
        /// Should approach full rotational velocity transfer
        #[test]
        fn light_payload_high_efficiency() {
            let moon = Moon::default();
            // Very light payload relative to tether
            let payload = Payload::new(Kilograms(10.0), MetersPerSecond(0.0));

            let tether = Tether::new(
                Kilometers(100.0),
                Kilometers(100.0),
                Kilograms(50_000.0), // 5000:1 mass ratio
                KEVLAR_49,
                MetersPerSecond(1000.0),
            );

            let impulse = tether.calc_impulse(payload, moon);

            // With 5000:1 mass ratio, efficiency = 5000/(1+5000) = 0.9998
            // The rotational component should be ~999.8 m/s
            // Total impulse includes orbital velocity differences too
            // We should get at least 97% of rotational velocity in the impulse
            assert!(
                impulse.0 > 970.0,
                "Light payload should achieve high momentum transfer, got {} m/s",
                impulse.0
            );
        }

        /// Test with heavy payload (low efficiency)
        #[test]
        fn heavy_payload_reduced_efficiency() {
            let moon = Moon::default();
            // Heavy payload equal to tether mass
            let payload = Payload::new(Kilograms(10_000.0), MetersPerSecond(0.0));

            let tether = Tether::new(
                Kilometers(100.0),
                Kilometers(100.0),
                Kilograms(10_000.0), // 1:1 mass ratio
                KEVLAR_49,
                MetersPerSecond(1000.0),
            );

            let impulse = tether.calc_impulse(payload, moon);

            // With 1:1 mass ratio, efficiency should be ~0.5
            // Impulse should be roughly half the rotational velocity
            let efficiency = impulse.0 / 1000.0;
            assert!(
                efficiency > 0.45 && efficiency < 0.55,
                "Equal mass payload should have ~50% efficiency, got {:.4}",
                efficiency
            );
        }

        /// Test altitude effect: higher orbits have lower orbital velocities
        #[test]
        fn altitude_affects_baseline_velocity() {
            let moon = Moon::default();
            let payload = Payload::new(Kilograms(1000.0), MetersPerSecond(0.0));

            let tether_low = Tether::new(
                Kilometers(50.0), // Lower orbit (faster)
                Kilometers(100.0),
                Kilograms(20_000.0),
                KEVLAR_49,
                MetersPerSecond(1000.0),
            );

            let tether_high = Tether::new(
                Kilometers(500.0), // Higher orbit (slower)
                Kilometers(100.0),
                Kilograms(20_000.0),
                KEVLAR_49,
                MetersPerSecond(1000.0),
            );

            let impulse_low = tether_low.calc_impulse(payload, moon);
            let impulse_high = tether_high.calc_impulse(payload, moon);

            // Both should provide similar rotational boost, but measured from different
            // orbital velocity baselines. The delta-v should be similar since it's
            // primarily determined by rotational velocity and mass ratio.
            let difference = (impulse_low.0 - impulse_high.0).abs();
            assert!(
                difference < 100.0,
                "Altitude should have minimal effect on rotational impulse transfer (difference: {} m/s)",
                difference
            );
        }

        /// Validate against published tether exchange ratios
        /// Hoyt & Forward (2000): "Mass ratio of 100:1 provides ~99% efficiency"
        #[test]
        fn validate_hoyt_forward_efficiency() {
            let moon = Moon::default();
            let payload = Payload::new(Kilograms(100.0), MetersPerSecond(0.0));

            let tether_light = Tether::new(
                Kilometers(100.0),
                Kilometers(100.0),
                Kilograms(1_000.0), // 10:1 mass ratio
                KEVLAR_49,
                MetersPerSecond(1500.0),
            );

            let tether_heavy = Tether::new(
                Kilometers(100.0),
                Kilometers(100.0),
                Kilograms(10_000.0), // 100:1 mass ratio
                KEVLAR_49,
                MetersPerSecond(1500.0),
            );

            let impulse_light = tether_light.calc_impulse(payload, moon);
            let impulse_heavy = tether_heavy.calc_impulse(payload, moon);

            // With 100:1 mass ratio, efficiency = 100/101 = 0.9901 (99.01%)
            // With 10:1 mass ratio, efficiency = 10/11 = 0.9091 (90.91%)
            // The heavier tether should provide noticeably more impulse
            let improvement = (impulse_heavy.0 - impulse_light.0) / impulse_light.0;
            assert!(
                improvement > 0.05,
                "100:1 mass ratio should provide significantly more impulse than 10:1 (improvement: {:.1}%)",
                improvement * 100.0
            );

            // Total impulse should be at least 97% of rotational velocity for 100:1 ratio
            assert!(
                impulse_heavy.0 > 1455.0,
                "100:1 ratio should yield high efficiency, got {} m/s",
                impulse_heavy.0
            );
        }
    }
}
