# CLAUDE.md - Almagest Workspace Guide

## Project Overview

Almagest is a Rust workspace for astrodynamics, providing tools for orbit determination, propagation, and analysis. The workspace is split into two crates:

- **`almagest`**: Core orbital mechanics library (`no_std`, pure Rust)
- **`almagest-wasm`**: WebAssembly bindings for web applications

## Key Features

- **Orbital mechanics calculations** (Kepler elements, propagation)
- **Material property database** for aerospace applications
- **Space tether analysis** (characteristic velocity, momentum exchange)
- **WebAssembly bindings** for web integration
- **No-std compatibility** for embedded/constrained environments

## Workspace Structure

```
almagest/                      # Workspace root
├── Cargo.toml                # Workspace manifest
├── almagest/                 # Core library crate
│   ├── Cargo.toml           # Pure Rust library (no_std, rlib)
│   ├── src/
│   │   ├── lib.rs           # Main library entry point
│   │   ├── kepler.rs        # Orbital mechanics
│   │   ├── materials.rs     # Material properties database
│   │   ├── tethers.rs       # Space tether calculations
│   │   └── utils.rs         # Type-safe units and utilities
│   └── examples/
│       └── tether_materials.rs # CLI material comparison
├── almagest-wasm/            # WASM bindings crate
│   ├── Cargo.toml           # WASM-specific (cdylib, depends on almagest)
│   ├── src/
│   │   └── lib.rs           # WebAssembly interface
│   ├── examples/
│   │   └── *.html           # Interactive web demos
│   └── pkg/                 # Generated WASM package (after build)
└── README.md
```

## Development Workflow

### Building and Testing

```bash
# Build and test the entire workspace
cargo build
cargo test

# Work with specific crates
cargo build -p almagest          # Core library only
cargo test -p almagest           # Test core library
cargo build -p almagest-wasm     # WASM bindings only

# Generate documentation
cargo doc --open
```

#### Comprehensive Testing Strategy

The project has three levels of testing for maximum reliability:

**1. Core Library Tests** (52 tests):
```bash
cargo test -p almagest
```
- Type-safe unit arithmetic and dimensional analysis
- Material property validation and comparisons
- Orbital mechanics calculations (circular, elliptical, parabolic)
- Tether characteristic velocity calculations with validation
- Edge cases, precision, and mathematical relationships

**2. WASM Interface Tests** (4 native tests):
```bash
cd almagest-wasm
cargo test
```
- JS-Rust data conversion accuracy
- Interface logic validation without WASM environment
- Cross-platform calculation consistency

**3. WASM Integration Tests** (12 WASM-specific tests):
```bash
cd almagest-wasm
wasm-pack test --node
```
- Actual WebAssembly environment testing
- Error handling propagation from Rust to JavaScript
- Memory management and cleanup verification
- Real-world material calculations in WASM
- Floating-point precision across language boundaries

**Run All Tests:**
```bash
# From workspace root - covers core + WASM interface
cargo test

# WASM-specific tests (requires wasm-pack)
cd almagest-wasm && wasm-pack test --node

# Alternative WASM test methods:
cd almagest-wasm && wasm-pack test --headless --chrome  # Browser testing
cd almagest-wasm && cargo test --target wasm32-unknown-unknown  # Direct cargo
```

### WebAssembly Development

```bash
# Install WASM target (first time only)
rustup target add wasm32-unknown-unknown
cargo install wasm-pack  # if not already installed

# Build WASM package from the almagest-wasm directory
cd almagest-wasm
wasm-pack build --target web

# Serve HTML examples (WASM requires HTTP server)
python3 -m http.server 8000
# Then visit http://localhost:8000/examples/
```

## Code Patterns and Conventions

### Type-Safe Units

The project uses newtype wrappers for physical units:

```rust
use crate::utils::{Pascals, KilogramsPerMetersCubed, MetersPerSecond};

let strength = Pascals(5.9e9);        // 5.9 GPa
let density = KilogramsPerMetersCubed(1340.0);  // kg/m³
let velocity = characteristic_velocity(strength, density)?;
```

### Material Database Usage

```rust
use almagest::materials::fibers;
use almagest::tethers::characteristic_velocity_for_material;

let pbo_velocity = characteristic_velocity_for_material(&fibers::PBO)?;
```

### WebAssembly Integration

For web applications, use the WASM bindings from the `almagest-wasm` crate:

```javascript
import init, { JsMaterial, characteristic_velocity_js } from './almagest-wasm/pkg/almagest_wasm.js';

await init();
const material = new JsMaterial(5.9e9, 1340.0);  // Pascals, kg/m³
const velocity = characteristic_velocity_js(material);
material.free();  // Important: clean up WASM memory
```

### Separation of Concerns

**CRITICAL**: All physics calculations and business logic must remain in the Rust crates (`almagest` core library). JavaScript code in web demos should **only** handle:
- UI interactions (sliders, buttons, form inputs)
- DOM manipulation and rendering
- Animation and visualization
- User input validation and formatting
- Calling WASM functions with properly formatted parameters

JavaScript should **never** implement physics calculations, even as fallbacks. If WASM fails, display an error message rather than duplicating complex calculations in JavaScript. This ensures:
- Single source of truth for all physics
- Consistent precision across platforms
- Easier maintenance and testing
- Type safety through Rust's ownership system

## Key Calculations

### Characteristic Velocity (Tethers)

**Formula**: `v = √(2σ/ρ)`
- Located in: `src/tethers.rs:42`
- Validation: Checks for reasonable physical limits
- Materials: Pre-defined in `src/materials.rs`

### Material Properties

The materials database includes:
- **Fibers**: PBO, Carbon Nanotube, Kevlar 49, UHMWPE
- **Metals**: Piano Wire, Titanium, Aluminum
- **Composites**: Carbon Fiber, Glass Fiber

Each material has:
- Tensile strength (Pascals)
- Density (kg/m³)
- Young's modulus (optional)
- Description

## Examples and Demos

### Command Line Examples

```bash
# Run material comparison example
cargo run --example tether_materials
```

### Interactive Web Demos

Located in `almagest-wasm/examples/*.html`:
- `characteristic_velocity.html` - Interactive material property explorer
- Uses `almagest-wasm` WASM bindings with JavaScript fallback
- Real-time calculation updates
- Material preset tiles

## Adding New Features

### New Material

Add to `almagest/src/materials.rs`:

```rust
pub const NEW_MATERIAL: Material = Material::new(
    "Material Name",
    Pascals(strength_in_pa),
    KilogramsPerMetersCubed(density),
    Some(Pascals(youngs_modulus)), // or None
    "Description"
);
```

### New WASM Function

1. Add Rust function in `almagest-wasm/src/lib.rs`
2. Rebuild WASM: `cd almagest-wasm && wasm-pack build --target web`
3. Import in JavaScript: `import { new_function } from './almagest-wasm/pkg/almagest_wasm.js'`

### New Web Demo

1. Create HTML file in `almagest-wasm/examples/`
2. Import WASM modules: `import('./pkg/almagest_wasm.js')`
3. Serve via HTTP server (required for WASM)

## Performance Notes

- **Calculations**: Core math uses `libm` for `no_std` compatibility
- **WASM**: ~16KB binary size for web deployment
- **Memory**: WASM objects need explicit `.free()` calls
- **Fallback**: JavaScript implementations available for WASM failures

## Testing Strategy (Legacy Section)

This section is now superseded by the comprehensive testing strategy above. The project has evolved to have:
- **68 total tests** across all levels
- **Multi-layered approach**: Native → Interface → WASM integration
- **Full WASM validation**: Actual WebAssembly environment testing
- **Memory safety verification**: Cross-language boundary testing

## Publishing and Distribution

- **Core crate**: `cargo publish` from `almagest/` directory
- **WASM crate**: `cargo publish` from `almagest-wasm/` directory
- **WASM package**: Generated in `almagest-wasm/pkg/` directory
- **Examples**: Self-contained HTML files in `almagest-wasm/examples/`

## Common Issues and Solutions

### Compilation Issues

**Workspace Architecture**: The project uses a clean workspace structure:
- **Core library** (`almagest/`): Pure `#![no_std]` with `crate-type = ["rlib"]`
- **WASM bindings** (`almagest-wasm/`): Uses `std` with `crate-type = ["cdylib"]`
- No conditional compilation needed - each crate has a single purpose
- Core library works in embedded environments, WASM crate handles web interface
- Tests work seamlessly since each crate has appropriate configuration

### WASM Loading Failures
- Ensure serving via HTTP (not file://)
- Check browser developer console for detailed errors
- Verify `almagest-wasm/pkg/` directory exists after wasm-pack build
- Import path should be `./almagest-wasm/pkg/almagest_wasm.js`

### Memory Issues
- Always call `.free()` on WASM objects
- Consider object pooling for high-frequency calculations

### Unit Conversion
- WASM functions expect SI units (Pascals, kg/m³)
- JavaScript helpers handle GPa ↔ Pa conversions
- Validate inputs before passing to WASM

## Future Development

- Consider adding more orbital mechanics functions
- Expand material database with more categories
- Add 3D visualization capabilities
- Performance optimization for large-scale calculations

---

*This file should be updated as the project evolves. It serves as a living document for both development and usage guidance.*