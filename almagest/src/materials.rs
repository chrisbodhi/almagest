//! Material properties for aerospace and engineering applications.
//!
//! This module provides well-characterized material properties for use in
//! space tether, structural, and other engineering calculations.

use crate::utils::{KilogramsPerMetersCubed, Pascals, Real};

/// Represents the key material properties needed for tether and structural calculations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    /// Material name
    pub name: &'static str,
    /// Ultimate tensile strength in Pascals
    pub tensile_strength: Pascals,
    /// Material density in kg/m³
    pub density: KilogramsPerMetersCubed,
    /// Young's modulus in Pascals (optional)
    pub youngs_modulus: Option<Pascals>,
    /// Material description or notes
    pub description: &'static str,
}

impl Material {
    /// Creates a new material with the specified properties.
    pub const fn new(
        name: &'static str,
        tensile_strength: Pascals,
        density: KilogramsPerMetersCubed,
        youngs_modulus: Option<Pascals>,
        description: &'static str,
    ) -> Self {
        Self {
            name,
            tensile_strength,
            density,
            youngs_modulus,
            description,
        }
    }

    /// Calculates the specific strength (strength-to-weight ratio) in N⋅m/kg.
    pub fn specific_strength(&self) -> Real {
        self.tensile_strength.value() / self.density.0
    }
}

/// High-performance fiber materials commonly used in space tethers.
pub mod fibers {
    use super::Material;
    use crate::utils::{KilogramsPerMetersCubed, Pascals};

    /// PBO (Poly-p-phenylene benzobisoxazole) - Zylon fiber
    /// One of the strongest synthetic fibers available.
    pub const PBO: Material = Material::new(
        "PBO (Zylon)",
        Pascals(5.9e9),                  // 5.9 GPa
        KilogramsPerMetersCubed(1340.0), // 1340 kg/m³
        Some(Pascals(270e9)),            // 270 GPa
        "Ultra-high strength synthetic fiber, excellent for space tethers",
    );

    /// Carbon Nanotube (theoretical single-wall)
    pub const CARBON_NANOTUBE: Material = Material::new(
        "Carbon Nanotube (SWNT)",
        Pascals(63e9),                   // 63 GPa (theoretical)
        KilogramsPerMetersCubed(1300.0), // 1300 kg/m³
        Some(Pascals(1000e9)),           // 1 TPa
        "Theoretical single-wall carbon nanotube properties",
    );

    /// Kevlar 49 - widely used aramid fiber
    pub const KEVLAR_49: Material = Material::new(
        "Kevlar 49",
        Pascals(3.6e9),                  // 3.6 GPa
        KilogramsPerMetersCubed(1440.0), // 1440 kg/m³
        Some(Pascals(112e9)),            // 112 GPa
        "High-strength aramid fiber, commonly used in aerospace",
    );

    /// Ultra-High Molecular Weight Polyethylene (Spectra/Dyneema)
    pub const UHMWPE: Material = Material::new(
        "UHMWPE (Spectra/Dyneema)",
        Pascals(3.5e9),                 // 3.5 GPa
        KilogramsPerMetersCubed(970.0), // 970 kg/m³
        Some(Pascals(172e9)),           // 172 GPa
        "Ultra-high molecular weight polyethylene fiber",
    );
}

/// Metallic materials for structural and cable applications.
pub mod metals {
    use super::Material;
    use crate::utils::{KilogramsPerMetersCubed, Pascals};

    /// Steel wire (high-carbon piano wire)
    pub const PIANO_WIRE: Material = Material::new(
        "Piano Wire Steel",
        Pascals(2.2e9),                  // 2.2 GPa
        KilogramsPerMetersCubed(7850.0), // 7850 kg/m³
        Some(Pascals(200e9)),            // 200 GPa
        "High-carbon steel wire, very high strength",
    );

    /// Aluminum 6061-T6
    pub const ALUMINUM_6061_T6: Material = Material::new(
        "Aluminum 6061-T6",
        Pascals(310e6),                  // 310 MPa
        KilogramsPerMetersCubed(2700.0), // 2700 kg/m³
        Some(Pascals(69e9)),             // 69 GPa
        "Common aerospace aluminum alloy",
    );

    /// Titanium Ti-6Al-4V
    pub const TITANIUM_6AL_4V: Material = Material::new(
        "Titanium Ti-6Al-4V",
        Pascals(1170e6),                 // 1.17 GPa
        KilogramsPerMetersCubed(4430.0), // 4430 kg/m³
        Some(Pascals(114e9)),            // 114 GPa
        "Aerospace grade titanium alloy",
    );
}

/// Composite materials for advanced applications.
pub mod composites {
    use super::Material;
    use crate::utils::{KilogramsPerMetersCubed, Pascals};

    /// Carbon fiber reinforced polymer (unidirectional)
    pub const CARBON_FIBER_UD: Material = Material::new(
        "Carbon Fiber (UD)",
        Pascals(3.5e9),                  // 3.5 GPa (fiber direction)
        KilogramsPerMetersCubed(1600.0), // 1600 kg/m³
        Some(Pascals(230e9)),            // 230 GPa
        "Unidirectional carbon fiber reinforced polymer",
    );

    /// Glass fiber reinforced polymer
    pub const GLASS_FIBER: Material = Material::new(
        "Glass Fiber (E-glass)",
        Pascals(3.4e9),                  // 3.4 GPa
        KilogramsPerMetersCubed(2540.0), // 2540 kg/m³
        Some(Pascals(72e9)),             // 72 GPa
        "E-glass fiber reinforced polymer",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_material_creation() {
        let material = Material::new(
            "Test Material",
            Pascals(1e9),
            KilogramsPerMetersCubed(1000.0),
            Some(Pascals(50e9)),
            "A test material",
        );

        assert_eq!(material.name, "Test Material");
        assert_eq!(material.tensile_strength.value(), 1e9);
        assert_eq!(material.density.0, 1000.0);
    }

    #[test]
    fn test_specific_strength() {
        let pbo = fibers::PBO;
        let specific_strength = pbo.specific_strength();

        // PBO: 5.9e9 Pa / 1340 kg/m³ ≈ 4.4e6 N⋅m/kg
        assert_relative_eq!(specific_strength, 4.4e6, epsilon = 0.1e6);
    }

    #[test]
    fn test_material_comparison() {
        let pbo = fibers::PBO;
        let kevlar = fibers::KEVLAR_49;
        let aluminum = metals::ALUMINUM_6061_T6;

        // PBO should have higher specific strength than Kevlar
        assert!(pbo.specific_strength() > kevlar.specific_strength());

        // Both fibers should have much higher specific strength than aluminum
        assert!(pbo.specific_strength() > aluminum.specific_strength());
        assert!(kevlar.specific_strength() > aluminum.specific_strength());
    }

    #[test]
    fn test_all_materials_have_valid_properties() {
        let materials = [
            fibers::PBO,
            fibers::CARBON_NANOTUBE,
            fibers::KEVLAR_49,
            fibers::UHMWPE,
            metals::PIANO_WIRE,
            metals::ALUMINUM_6061_T6,
            metals::TITANIUM_6AL_4V,
            composites::CARBON_FIBER_UD,
            composites::GLASS_FIBER,
        ];

        for material in &materials {
            // All materials should have positive tensile strength and density
            assert!(material.tensile_strength.value() > 0.0);
            assert!(material.density.0 > 0.0);

            // Name and description should not be empty
            assert!(!material.name.is_empty());
            assert!(!material.description.is_empty());

            // Young's modulus, if present, should be positive
            if let Some(youngs) = material.youngs_modulus {
                assert!(youngs.value() > 0.0);
            }
        }
    }

    #[test]
    fn test_realistic_material_properties() {
        // Test that material properties are within realistic ranges

        // PBO should have very high strength
        assert!(fibers::PBO.tensile_strength.value() > 5e9); // > 5 GPa

        // Aluminum should be much weaker than high-performance fibers
        assert!(metals::ALUMINUM_6061_T6.tensile_strength.value() < 1e9); // < 1 GPa

        // Carbon nanotubes should have the highest theoretical strength
        assert!(
            fibers::CARBON_NANOTUBE.tensile_strength.value() > fibers::PBO.tensile_strength.value()
        );

        // Density ranges should be realistic
        // Fibers typically 1000-2000 kg/m³
        assert!(fibers::PBO.density.0 > 1000.0 && fibers::PBO.density.0 < 2000.0);

        // Metals typically 2000-8000 kg/m³
        assert!(
            metals::ALUMINUM_6061_T6.density.0 > 2000.0
                && metals::ALUMINUM_6061_T6.density.0 < 4000.0
        );
        assert!(
            metals::TITANIUM_6AL_4V.density.0 > 4000.0
                && metals::TITANIUM_6AL_4V.density.0 < 5000.0
        );
    }
}
