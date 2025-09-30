#[allow(dead_code, unused_imports, unused_variables)]
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

    // NEXT!!!!
    pub fn calc_impulse(&self, payload: Payload, body: Moon) -> MetersPerSecond {
        todo!("ChatGPT said this is the top priority.")
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

pub struct Moon {
    gravity: MetersPerSecondSquared,
    mass: Kilograms,
    radius: Kilometers,
}

impl Moon {
    pub fn new() -> Self {
        Self {
            mass: Kilograms(7.348e22),
            radius: Kilometers(1_737.5),
            gravity: MetersPerSecondSquared(1.625),
        }
    }
}

/// The payload consists of the thing being carried and the thing doing the carrying.
///   - mass
///   - velocity
#[derive(Debug)]
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
        assert_eq!(t.length, Kilometers(100.0));
        assert_eq!(t.rotational_velocity, MetersPerSecond(7_000.0));
    }
}
