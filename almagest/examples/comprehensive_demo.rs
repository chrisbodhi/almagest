//! # Comprehensive Almagest Library Demo
//!
//! This example demonstrates the key features of the Almagest astrodynamics library,
//! showcasing orbital mechanics, material properties, and tether calculations.

use almagest::{
    celestials::celestial_bodies::{EARTH, MARS, MOON},
    kepler::{Ellipse, Point},
    materials::{fibers, metals},
    tethers::{characteristic_velocity, characteristic_velocity_for_material},
    utils::{Eccentricity, KilogramsPerMetersCubed, Kilometers, Meters, Pascals},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Almagest Astrodynamics Library Demo\n");

    // === Celestial Bodies ===
    println!("=== Celestial Bodies ===");
    demo_celestial_bodies();
    println!();

    // === Type-Safe Units ===
    println!("=== Type-Safe Unit System ===");
    demo_units();
    println!();

    // === Orbital Mechanics ===
    println!("=== Orbital Mechanics ===");
    demo_orbital_mechanics()?;
    println!();

    // === Materials Database ===
    println!("=== Materials Database ===");
    demo_materials();
    println!();

    // === Tether Analysis ===
    println!("=== Space Tether Analysis ===");
    demo_tether_analysis()?;
    println!();

    println!("✅ Demo completed successfully!");
    Ok(())
}

/// Demonstrates celestial body properties and comparisons
fn demo_celestial_bodies() {
    println!("Earth properties:");
    println!("  Mass: {:.3e} kg", EARTH.mass.0);
    println!("  Radius: {} km", EARTH.radius.0);

    println!("\nComparative analysis:");
    let earth_mars_mass_ratio = EARTH.mass.0 / MARS.mass.0;
    let earth_moon_radius_ratio = EARTH.radius.0 / MOON.radius.0;

    println!(
        "  Earth is {:.1}x more massive than Mars",
        earth_mars_mass_ratio
    );
    println!(
        "  Earth is {:.1}x larger in radius than Moon",
        earth_moon_radius_ratio
    );
}

/// Demonstrates type-safe unit operations
fn demo_units() {
    // Length calculations with automatic unit propagation
    let earth_radius = Meters(6_371_000.0);
    let iss_altitude = Meters(408_000.0);
    let iss_orbit_radius = earth_radius + iss_altitude;

    println!("ISS orbital radius: {} m", iss_orbit_radius.value());

    // Area calculation from length multiplication
    let length = Meters(10.0);
    let width = Meters(5.0);
    let area = length * width;
    println!(
        "Area calculation: {} × {} = {} m²",
        length.value(),
        width.value(),
        area.value()
    );

    // Unit conversion
    let distance_km: Kilometers = earth_radius.into();
    println!("Earth radius: {} km", distance_km.0);
}

/// Demonstrates orbital mechanics calculations
fn demo_orbital_mechanics() -> Result<(), &'static str> {
    println!("Creating orbital ellipses...");

    // Circular Low Earth Orbit
    let leo = Ellipse::new(
        Eccentricity::new(0.0)?, // Perfect circle
        Point {
            x: Meters(0.0),
            y: Meters(0.0),
        },
        Meters(6_671_000.0), // ~300 km altitude
    );

    let semi_major: Kilometers = leo.semi_major_axis().into();
    let peri: Kilometers = leo.periapsis().into();
    let apo: Kilometers = leo.apoapsis().into();

    println!("LEO (circular orbit):");
    println!("  Semi-major axis: {:.0} km", semi_major.value());
    println!("  Eccentricity: {:.3}", leo.eccentricity().value());
    println!("  Periapsis: {:.0} km", peri.value());
    println!("  Apoapsis: {:.0} km", apo.value());

    // Geostationary Transfer Orbit
    let gto = Ellipse::from_periapsis_apoapsis(
        Meters(6_571_000.0),  // 200 km altitude
        Meters(42_164_000.0), // GEO altitude
        Point {
            x: Meters(0.0),
            y: Meters(0.0),
        },
    );

    let gto_semi: Kilometers = gto.semi_major_axis().into();
    let gto_semi_minor: Kilometers = gto.semi_minor_axis().into();

    println!("\nGTO (elliptical transfer orbit):");
    println!("  Semi-major axis: {:.0} km", gto_semi.value());
    println!("  Eccentricity: {:.4}", gto.eccentricity().value());
    println!("  Semi-minor axis: {:.0} km", gto_semi_minor.value());

    Ok(())
}

/// Demonstrates material properties and comparisons
fn demo_materials() {
    println!("High-performance materials comparison:");

    let materials = [
        ("PBO Fiber", &fibers::PBO),
        ("Kevlar 49", &fibers::KEVLAR_49),
        ("UHMWPE", &fibers::UHMWPE),
        ("Steel Wire", &metals::PIANO_WIRE),
        ("Aluminum", &metals::ALUMINUM_6061_T6),
    ];

    println!(
        "{:<12} {:>12} {:>12} {:>15}",
        "Material", "Strength(GPa)", "Density(kg/m³)", "Specific(MJ/kg)"
    );
    println!("{}", "-".repeat(55));

    for (name, material) in &materials {
        let strength_gpa = material.tensile_strength.value() / 1e9;
        let specific_strength = material.specific_strength() / 1e6; // Convert to MJ/kg

        println!(
            "{:<12} {:>12.1} {:>12.0} {:>15.2}",
            name, strength_gpa, material.density.0, specific_strength
        );
    }
}

/// Demonstrates space tether characteristic velocity calculations
fn demo_tether_analysis() -> Result<(), &'static str> {
    println!("Tether characteristic velocity analysis:");

    // Calculate using individual parameters
    let custom_velocity = characteristic_velocity(
        Pascals(4.0e9),                  // 4 GPa tensile strength
        KilogramsPerMetersCubed(1500.0), // 1500 kg/m³ density
    )?;

    println!(
        "Custom material (4 GPa, 1500 kg/m³): {:.0} m/s",
        custom_velocity.value()
    );

    // Compare different materials
    println!("\nMaterial performance ranking:");
    let test_materials = [
        ("PBO", &fibers::PBO),
        ("Carbon Nanotube", &fibers::CARBON_NANOTUBE),
        ("UHMWPE", &fibers::UHMWPE),
        ("Kevlar 49", &fibers::KEVLAR_49),
        ("Steel Piano Wire", &metals::PIANO_WIRE),
    ];

    let mut velocities: Vec<(f64, &str)> = test_materials
        .iter()
        .map(|(name, material)| {
            let velocity = characteristic_velocity_for_material(material).expect("Valid material");
            (velocity.value(), *name)
        })
        .collect();

    // Sort by characteristic velocity (descending)
    velocities.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    for (i, (velocity, name)) in velocities.iter().enumerate() {
        println!("  {}. {:<18} {:>6.0} m/s", i + 1, name, velocity);
    }

    // Physical interpretation
    println!("\n💡 Physical meaning:");
    println!("   Higher characteristic velocity = better tether performance");
    println!("   Represents maximum theoretical deployment speed");
    println!("   Formula: v = √(2σ/ρ) where σ=strength, ρ=density");

    Ok(())
}
