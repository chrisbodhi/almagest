//! Example: Comparing materials for space tether applications
//!
//! This example demonstrates how to use the materials database to compare
//! different materials for space tether applications by calculating their
//! characteristic velocities and specific strengths.
//!
//! Run with: cargo run --example tether_materials

use almagest::materials::{Material, composites, fibers, metals};
use almagest::tethers::characteristic_velocity_for_material;
use almagest::utils::MetersPerSecond;

fn main() {
    println!("Space Tether Material Comparison");
    println!("================================\n");

    // Define materials to compare
    let materials = [
        (
            "High-Performance Fibers",
            vec![
                &fibers::PBO,
                &fibers::CARBON_NANOTUBE,
                &fibers::KEVLAR_49,
                &fibers::UHMWPE,
            ],
        ),
        (
            "Metals",
            vec![
                &metals::PIANO_WIRE,
                &metals::TITANIUM_6AL_4V,
                &metals::ALUMINUM_6061_T6,
            ],
        ),
        (
            "Composites",
            vec![&composites::CARBON_FIBER_UD, &composites::GLASS_FIBER],
        ),
    ];

    for (category, material_list) in materials {
        println!("{}", category);
        println!("{}", "=".repeat(category.len()));
        println!();

        for material in material_list {
            analyze_material(material);
            println!();
        }
        println!();
    }

    // Find the best materials
    println!("Summary: Top Materials for Space Tethers");
    println!("========================================\n");

    let all_materials = vec![
        &fibers::PBO,
        &fibers::CARBON_NANOTUBE,
        &fibers::KEVLAR_49,
        &fibers::UHMWPE,
        &metals::PIANO_WIRE,
        &metals::TITANIUM_6AL_4V,
        &metals::ALUMINUM_6061_T6,
        &composites::CARBON_FIBER_UD,
        &composites::GLASS_FIBER,
    ];

    // Sort by characteristic velocity
    let mut materials_by_velocity: Vec<_> = all_materials
        .iter()
        .map(|mat| {
            let velocity =
                characteristic_velocity_for_material(mat).unwrap_or(MetersPerSecond(0.0));
            (*mat, velocity.0)
        })
        .collect();
    materials_by_velocity.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("Ranked by Characteristic Velocity (higher is better for tethers):");
    for (i, (material, velocity)) in materials_by_velocity.iter().enumerate() {
        println!("{:2}. {:25} {:6.0} m/s", i + 1, material.name, velocity);
    }

    println!("\n📊 Key Insights:");
    println!("• Theoretical carbon nanotubes offer the highest performance");
    println!("• PBO (Zylon) is the best currently available material");
    println!("• High-performance synthetic fibers far outperform metals");
    println!("• Specific strength correlates well with tether performance");
}

fn analyze_material(material: &Material) {
    let char_velocity = characteristic_velocity_for_material(material)
        .map(|v| v.0)
        .unwrap_or(0.0);

    let specific_strength = material.specific_strength();

    println!("📋 {}", material.name);
    println!("   Description: {}", material.description);
    println!(
        "   Tensile Strength: {:.1} GPa",
        material.tensile_strength.value() / 1e9
    );
    println!("   Density: {:.0} kg/m³", material.density.0);

    if let Some(youngs) = material.youngs_modulus {
        println!("   Young's Modulus: {:.0} GPa", youngs.value() / 1e9);
    }

    println!(
        "   Specific Strength: {:.1} kN⋅m/kg",
        specific_strength / 1000.0
    );
    println!("   Characteristic Velocity: {:.0} m/s", char_velocity);

    // Performance rating
    let rating = match char_velocity {
        v if v > 4000.0 => "🌟 Exceptional (theoretical limit)",
        v if v > 2500.0 => "⭐ Excellent",
        v if v > 1500.0 => "✨ Good",
        v if v > 800.0 => "💫 Fair",
        _ => "⚡ Poor",
    };
    println!("   Tether Rating: {}", rating);
}
