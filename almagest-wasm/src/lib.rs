use almagest::tethers::{
    characteristic_velocity, momentum_exchange_angular_velocity, momentum_exchange_efficiency,
    momentum_exchange_orbital_period, momentum_exchange_orbital_velocity,
    momentum_exchange_spin_rate,
};
use almagest::utils::{
    KilogramsPerMetersCubed, Meters, MetersCubedByKilogramSecondsSquared, MetersPerSecond, Pascals,
    RadiansPerSecond, Seconds,
};
use wasm_bindgen::prelude::*;

/// JS-friendly struct for passing material properties from JavaScript.
#[wasm_bindgen]
pub struct JsMaterial {
    #[wasm_bindgen(readonly)]
    pub tensile_strength: f64,
    #[wasm_bindgen(readonly)]
    pub density: f64,
}

#[wasm_bindgen]
impl JsMaterial {
    #[wasm_bindgen(constructor)]
    pub fn new(tensile_strength: f64, density: f64) -> JsMaterial {
        JsMaterial {
            tensile_strength,
            density,
        }
    }
}

fn handle_error(error_msg: &str) -> JsValue {
    #[cfg(target_arch = "wasm32")]
    {
        JsValue::from_str(error_msg)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("Calculation failed: {}", error_msg);
        JsValue::UNDEFINED
    }
}

/// JS-friendly wrapper for characteristic_velocity that accepts a JsMaterial object.
#[wasm_bindgen]
pub fn characteristic_velocity_js(material: &JsMaterial) -> Result<f64, JsValue> {
    let ts = Pascals(material.tensile_strength);
    let d = KilogramsPerMetersCubed(material.density);
    match characteristic_velocity(ts, d) {
        Ok(MetersPerSecond(val)) => Ok(val),
        Err(e) => Err(handle_error(e)),
    }
}

/// JS-friendly struct for orbital parameters.
#[wasm_bindgen]
pub struct JsOrbitalParams {
    #[wasm_bindgen(readonly)]
    pub radius: f64,
    #[wasm_bindgen(readonly)]
    pub gravitational_parameter: f64,
}

#[wasm_bindgen]
impl JsOrbitalParams {
    #[wasm_bindgen(constructor)]
    pub fn new(radius: f64, gravitational_parameter: f64) -> JsOrbitalParams {
        JsOrbitalParams {
            radius,
            gravitational_parameter,
        }
    }
}

/// JS-friendly struct for momentum exchange tether parameters.
#[wasm_bindgen]
pub struct JsTetherParams {
    #[wasm_bindgen(readonly)]
    pub material_tensile_strength: f64,
    #[wasm_bindgen(readonly)]
    pub material_density: f64,
    #[wasm_bindgen(readonly)]
    pub orbital_radius: f64,
    #[wasm_bindgen(readonly)]
    pub gravitational_parameter: f64,
}

#[wasm_bindgen]
impl JsTetherParams {
    #[wasm_bindgen(constructor)]
    pub fn new(
        material_tensile_strength: f64,
        material_density: f64,
        orbital_radius: f64,
        gravitational_parameter: f64,
    ) -> JsTetherParams {
        JsTetherParams {
            material_tensile_strength,
            material_density,
            orbital_radius,
            gravitational_parameter,
        }
    }
}

/// Calculates orbital velocity for momentum exchange tether.
#[wasm_bindgen]
pub fn momentum_exchange_orbital_velocity_js(params: &JsOrbitalParams) -> Result<f64, JsValue> {
    let radius = Meters(params.radius);
    let mu = MetersCubedByKilogramSecondsSquared(params.gravitational_parameter);

    match momentum_exchange_orbital_velocity(radius, mu) {
        Ok(MetersPerSecond(val)) => Ok(val),
        Err(e) => Err(handle_error(e)),
    }
}

/// Calculates orbital period for momentum exchange tether.
#[wasm_bindgen]
pub fn momentum_exchange_orbital_period_js(params: &JsOrbitalParams) -> Result<f64, JsValue> {
    let radius = Meters(params.radius);
    let mu = MetersCubedByKilogramSecondsSquared(params.gravitational_parameter);

    match momentum_exchange_orbital_period(radius, mu) {
        Ok(Seconds(val)) => Ok(val),
        Err(e) => Err(handle_error(e)),
    }
}

/// Calculates angular velocity for momentum exchange tether.
#[wasm_bindgen]
pub fn momentum_exchange_angular_velocity_js(params: &JsOrbitalParams) -> Result<f64, JsValue> {
    let radius = Meters(params.radius);
    let mu = MetersCubedByKilogramSecondsSquared(params.gravitational_parameter);

    match momentum_exchange_angular_velocity(radius, mu) {
        Ok(RadiansPerSecond(val)) => Ok(val),
        Err(e) => Err(handle_error(e)),
    }
}

/// Calculates momentum exchange efficiency for a tether system.
#[wasm_bindgen]
pub fn momentum_exchange_efficiency_js(params: &JsTetherParams) -> Result<f64, JsValue> {
    // First create the material from the parameters
    let material = almagest::materials::Material::new(
        "Custom Material",
        Pascals(params.material_tensile_strength),
        KilogramsPerMetersCubed(params.material_density),
        None,
        "User-defined material for momentum exchange calculation",
    );

    let radius = Meters(params.orbital_radius);
    let mu = MetersCubedByKilogramSecondsSquared(params.gravitational_parameter);

    match momentum_exchange_efficiency(&material, radius, mu) {
        Ok(efficiency) => Ok(efficiency),
        Err(e) => Err(handle_error(e)),
    }
}

/// Calculates tether spin rate based on Moravec 1977 physics.
#[wasm_bindgen]
pub fn momentum_exchange_spin_rate_js(
    tether_length: f64,
    central_body_radius: f64,
) -> Result<f64, JsValue> {
    let tether = Meters(tether_length);
    let radius = Meters(central_body_radius);

    match momentum_exchange_spin_rate(tether, radius) {
        Ok(spin_multiplier) => Ok(spin_multiplier),
        Err(e) => Err(handle_error(e)),
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use approx::assert_relative_eq;

    // Standard Rust tests for the WASM interface logic
    #[test]
    fn test_js_material_native() {
        let material = JsMaterial::new(5.9e9, 1340.0);
        assert_eq!(material.tensile_strength, 5.9e9);
        assert_eq!(material.density, 1340.0);
    }

    #[test]
    fn test_characteristic_velocity_js_native() {
        let material = JsMaterial::new(5.9e9, 1340.0);
        let result = characteristic_velocity_js(&material);

        assert!(result.is_ok());
        let velocity = result.unwrap();
        assert_relative_eq!(velocity, 2967.49, epsilon = 0.01);
    }

    #[test]
    fn test_error_propagation_native() {
        let material = JsMaterial::new(-1000.0, 1340.0);
        let result = characteristic_velocity_js(&material);

        // Just test that error is returned - we can't check the message in native tests
        assert!(result.is_err());
    }

    #[test]
    fn test_wasm_vs_native_accuracy() {
        // Test that WASM interface produces identical results to native
        let test_cases = [
            (5.9e9, 1340.0), // PBO
            (3.6e9, 1440.0), // Kevlar 49
            (3.5e9, 970.0),  // UHMWPE
            (2.2e9, 7850.0), // Piano wire
        ];

        for (strength, density) in test_cases {
            // Calculate using native Rust
            let native_result =
                characteristic_velocity(Pascals(strength), KilogramsPerMetersCubed(density))
                    .unwrap();

            // Calculate using WASM interface
            let js_material = JsMaterial::new(strength, density);
            let wasm_result = characteristic_velocity_js(&js_material).unwrap();

            // Results should be identical
            assert_relative_eq!(native_result.value(), wasm_result, epsilon = 1e-12);
        }
    }

    #[test]
    fn test_js_orbital_params_native() {
        let earth_mu = 3.986004418e14;
        let leo_radius = 6.771e6; // Earth radius + 400km

        let params = JsOrbitalParams::new(leo_radius, earth_mu);
        assert_eq!(params.radius, leo_radius);
        assert_eq!(params.gravitational_parameter, earth_mu);
    }

    #[test]
    fn test_momentum_exchange_orbital_velocity_js_native() {
        let earth_mu = 3.986004418e14;
        let leo_radius = 6.771e6;
        let params = JsOrbitalParams::new(leo_radius, earth_mu);

        let wasm_velocity = momentum_exchange_orbital_velocity_js(&params).unwrap();
        let native_velocity = momentum_exchange_orbital_velocity(
            Meters(leo_radius),
            MetersCubedByKilogramSecondsSquared(earth_mu),
        )
        .unwrap();

        assert_relative_eq!(wasm_velocity, native_velocity.value(), epsilon = 1e-12);
        assert_relative_eq!(wasm_velocity, 7669.0, epsilon = 10.0);
    }

    #[test]
    fn test_momentum_exchange_orbital_period_js_native() {
        let earth_mu = 3.986004418e14;
        let leo_radius = 6.771e6;
        let params = JsOrbitalParams::new(leo_radius, earth_mu);

        let wasm_period = momentum_exchange_orbital_period_js(&params).unwrap();
        let native_period = momentum_exchange_orbital_period(
            Meters(leo_radius),
            MetersCubedByKilogramSecondsSquared(earth_mu),
        )
        .unwrap();

        assert_relative_eq!(wasm_period, native_period.value(), epsilon = 1e-12);
        assert_relative_eq!(wasm_period, 5543.0, epsilon = 10.0);
    }

    #[test]
    fn test_momentum_exchange_angular_velocity_js_native() {
        let earth_mu = 3.986004418e14;
        let leo_radius = 6.771e6;
        let params = JsOrbitalParams::new(leo_radius, earth_mu);

        let wasm_angular_vel = momentum_exchange_angular_velocity_js(&params).unwrap();
        let native_angular_vel = momentum_exchange_angular_velocity(
            Meters(leo_radius),
            MetersCubedByKilogramSecondsSquared(earth_mu),
        )
        .unwrap();

        assert_relative_eq!(
            wasm_angular_vel,
            native_angular_vel.value(),
            epsilon = 1e-12
        );
        assert_relative_eq!(wasm_angular_vel, 0.00113, epsilon = 0.0001);
    }

    #[test]
    fn test_js_tether_params_native() {
        let tether_params = JsTetherParams::new(5.9e9, 1340.0, 6.771e6, 3.986004418e14);

        assert_eq!(tether_params.material_tensile_strength, 5.9e9);
        assert_eq!(tether_params.material_density, 1340.0);
        assert_eq!(tether_params.orbital_radius, 6.771e6);
        assert_eq!(tether_params.gravitational_parameter, 3.986004418e14);
    }

    #[test]
    fn test_momentum_exchange_efficiency_js_native() {
        let tether_params = JsTetherParams::new(5.9e9, 1340.0, 6.771e6, 3.986004418e14);

        let wasm_efficiency = momentum_exchange_efficiency_js(&tether_params).unwrap();

        // Create equivalent material for native calculation
        let material = almagest::materials::Material::new(
            "Test Material",
            Pascals(5.9e9),
            KilogramsPerMetersCubed(1340.0),
            None,
            "Test material",
        );

        let native_efficiency = momentum_exchange_efficiency(
            &material,
            Meters(6.771e6),
            MetersCubedByKilogramSecondsSquared(3.986004418e14),
        )
        .unwrap();

        assert_relative_eq!(wasm_efficiency, native_efficiency, epsilon = 1e-12);
        assert_relative_eq!(wasm_efficiency, 0.387, epsilon = 0.01);
        assert!(wasm_efficiency >= 0.0 && wasm_efficiency <= 1.0);
    }

    // WASM-specific tests (only run in WASM environment)
    #[cfg(target_arch = "wasm32")]
    mod wasm_tests {
        use super::*;
        use wasm_bindgen_test::*;

        // Tests can run in both browser and Node.js environments

        #[wasm_bindgen_test]
        fn test_js_material_construction() {
            let material = JsMaterial::new(5.9e9, 1340.0);
            assert_eq!(material.tensile_strength, 5.9e9);
            assert_eq!(material.density, 1340.0);
        }

        #[wasm_bindgen_test]
        fn test_js_material_property_access() {
            let material = JsMaterial::new(3.6e9, 1440.0);

            // Test that properties are accessible (readonly)
            assert_eq!(material.tensile_strength, 3.6e9);
            assert_eq!(material.density, 1440.0);
        }

        #[wasm_bindgen_test]
        fn test_characteristic_velocity_js_success() {
            let material = JsMaterial::new(5.9e9, 1340.0);
            let result = characteristic_velocity_js(&material);

            assert!(result.is_ok());
            let velocity = result.unwrap();

            // Should match the expected PBO characteristic velocity
            assert_relative_eq!(velocity, 2967.49, epsilon = 0.01);
        }

        #[wasm_bindgen_test]
        fn test_characteristic_velocity_js_vs_native() {
            // Test that WASM calculations exactly match native calculations
            let test_cases = [
                (5.9e9, 1340.0), // PBO
                (3.6e9, 1440.0), // Kevlar 49
                (3.5e9, 970.0),  // UHMWPE
                (2.2e9, 7850.0), // Piano wire
            ];

            for (strength, density) in test_cases {
                // Calculate using native Rust
                let native_result =
                    characteristic_velocity(Pascals(strength), KilogramsPerMetersCubed(density))
                        .unwrap();

                // Calculate using WASM interface
                let js_material = JsMaterial::new(strength, density);
                let wasm_result = characteristic_velocity_js(&js_material).unwrap();

                // Results should be identical
                assert_relative_eq!(native_result.value(), wasm_result, epsilon = 1e-12);
            }
        }

        #[wasm_bindgen_test]
        fn test_error_handling_negative_tensile_strength() {
            let material = JsMaterial::new(-1000.0, 1340.0);
            let result = characteristic_velocity_js(&material);

            assert!(result.is_err());

            // Convert JsValue error back to string to verify error message
            let error_str = result.unwrap_err().as_string().unwrap();
            assert_eq!(error_str, "Tensile strength must be positive");
        }

        #[wasm_bindgen_test]
        fn test_error_handling_negative_density() {
            let material = JsMaterial::new(5.9e9, -1340.0);
            let result = characteristic_velocity_js(&material);

            assert!(result.is_err());
            let error_str = result.unwrap_err().as_string().unwrap();
            assert_eq!(error_str, "Density must be positive");
        }

        #[wasm_bindgen_test]
        fn test_error_handling_zero_values() {
            // Zero tensile strength
            let material1 = JsMaterial::new(0.0, 1340.0);
            let result1 = characteristic_velocity_js(&material1);
            assert!(result1.is_err());

            // Zero density
            let material2 = JsMaterial::new(5.9e9, 0.0);
            let result2 = characteristic_velocity_js(&material2);
            assert!(result2.is_err());
        }

        #[wasm_bindgen_test]
        fn test_error_handling_excessive_values() {
            // Tensile strength beyond material limits
            let material1 = JsMaterial::new(300e9, 1340.0);
            let result1 = characteristic_velocity_js(&material1);
            assert!(result1.is_err());
            let error_str1 = result1.unwrap_err().as_string().unwrap();
            assert_eq!(error_str1, "Tensile strength exceeds known material limits");

            // Density beyond reasonable limits
            let material2 = JsMaterial::new(5.9e9, 60_000.0);
            let result2 = characteristic_velocity_js(&material2);
            assert!(result2.is_err());
            let error_str2 = result2.unwrap_err().as_string().unwrap();
            assert_eq!(error_str2, "Density exceeds reasonable material limits");
        }

        #[wasm_bindgen_test]
        fn test_floating_point_precision() {
            // Test with various floating-point values to ensure precision is maintained
            let precision_cases = [
                (1.23456789e9, 987.654321),
                (9.87654321e8, 1234.56789),
                (5.0e9, 1500.0),
                (1.0e6, 500.0),
            ];

            for (strength, density) in precision_cases {
                let material = JsMaterial::new(strength, density);
                let result = characteristic_velocity_js(&material);

                assert!(result.is_ok());
                let velocity = result.unwrap();

                // Verify the calculation is reasonable
                let expected = (2.0 * strength / density).sqrt();
                assert_relative_eq!(velocity, expected, epsilon = 1e-12);
            }
        }

        #[wasm_bindgen_test]
        fn test_real_world_materials() {
            // Test with realistic material properties
            struct TestMaterial {
                name: &'static str,
                strength: f64,
                density: f64,
                expected_velocity_range: (f64, f64), // (min, max)
            }

            let materials = [
                TestMaterial {
                    name: "PBO",
                    strength: 5.9e9,
                    density: 1340.0,
                    expected_velocity_range: (2900.0, 3000.0),
                },
                TestMaterial {
                    name: "Kevlar 49",
                    strength: 3.6e9,
                    density: 1440.0,
                    expected_velocity_range: (2200.0, 2300.0),
                },
                TestMaterial {
                    name: "UHMWPE",
                    strength: 3.5e9,
                    density: 970.0,
                    expected_velocity_range: (2600.0, 2700.0),
                },
                TestMaterial {
                    name: "Steel Piano Wire",
                    strength: 2.2e9,
                    density: 7850.0,
                    expected_velocity_range: (700.0, 800.0),
                },
            ];

            for mat in &materials {
                let js_material = JsMaterial::new(mat.strength, mat.density);
                let velocity = characteristic_velocity_js(&js_material).unwrap();

                assert!(
                    velocity >= mat.expected_velocity_range.0
                        && velocity <= mat.expected_velocity_range.1,
                    "Material {} velocity {} not in expected range {:?}",
                    mat.name,
                    velocity,
                    mat.expected_velocity_range
                );
            }
        }

        #[wasm_bindgen_test]
        fn test_data_conversion_edge_cases() {
            // Test very large numbers
            let large_material = JsMaterial::new(100e9, 10000.0);
            let result = characteristic_velocity_js(&large_material);
            assert!(result.is_ok());

            // Test very small but valid numbers
            let small_material = JsMaterial::new(1e6, 100.0);
            let result = characteristic_velocity_js(&small_material);
            assert!(result.is_ok());

            // Test that result makes physical sense
            let velocity = result.unwrap();
            assert!(velocity > 0.0);
            assert!(velocity < 1e6); // Should be less than 1 million m/s
        }

        #[wasm_bindgen_test]
        fn test_multiple_calculations() {
            // Test that multiple successive calculations work correctly
            // (tests that there are no memory issues or state corruption)
            let material = JsMaterial::new(5.9e9, 1340.0);

            let results: Vec<f64> = (0..10)
                .map(|_| characteristic_velocity_js(&material).unwrap())
                .collect();

            // All results should be identical
            for result in &results[1..] {
                assert_relative_eq!(results[0], *result, epsilon = 1e-15);
            }

            // Should match expected PBO velocity
            assert_relative_eq!(results[0], 2967.49, epsilon = 0.01);
        }
    } // end wasm_tests module
}
