# almagest
_an astronomical library workspace_

⚠️ This library is in active development. As such, there are no guarantees of API stability or even correctness. ⚠️

Rust edition 2024

>Keep in mind that I lay no claim to having discovered these things<br/>
>through my own skill. I am but an ignorant compiler of the works of<br/>
>ancient astronomers, and have but put their material into my own<br/>
>words for your instruction; and with this sword shall I slay envy.<br/>

&mdash; Geoffrey Chaucer, _Prologue of his Treatise on the Astrolabe_, by way of Albert Waugh in _Sundials: Their Theory and Construction_

## Overview

Almagest is a Rust workspace for astrodynamics, providing tools for orbit determination, propagation, and analysis. The workspace contains two crates:

- **`almagest`** - Core orbital mechanics library (`no_std`, pure Rust)
- **`almagest-wasm`** - WebAssembly bindings for web applications

The core library is written without the usage of the `std` library, relying on the `no_std` feature for minimal dependencies and maximum deployment flexibility.

## Using almagest with WebAssembly

The `almagest-wasm` crate provides WebAssembly bindings for the core `almagest` library. This allows you to use orbital mechanics calculations in web applications.

**Example JavaScript usage:**

```js
import init, { JsMaterial, characteristic_velocity_js } from './almagest-wasm/pkg/almagest_wasm.js';

async function run() {
    await init();

    // Example: Calculate characteristic velocity for PBO fiber
    // 5.9e9 Pascals tensile strength, 1340 kg/m³ density
    const pbo = new JsMaterial(5.9e9, 1340.0);
    const velocity = characteristic_velocity_js(pbo);

    console.log('PBO characteristic velocity:', velocity, 'm/s');
    // Output: ~2967 m/s

    pbo.free(); // Clean up WASM memory
}

run();
```

See the [interactive demo](almagest-wasm/examples/characteristic_velocity.html) for a complete web application example.

## Development

Install Rust edition 2024 and, with it, `cargo`, et al.

### Build and Test

```sh
# Build and test the entire workspace
cargo build
cargo test

# Work with specific crates
cargo build -p almagest          # Core library only
cargo test -p almagest           # Test core library
cargo run -p almagest --example tether_materials  # Run CLI example
```

#### Testing Strategy

The project has comprehensive testing at multiple levels:

**Core Library Tests**
```sh
cargo test -p almagest
# Tests: units, materials, orbital mechanics, tether calculations
```

**WASM Interface Tests**
```sh
cd almagest-wasm
cargo test
# Tests: JS-Rust data conversion, interface logic validation
```

**WASM Integration Tests**
```sh
cd almagest-wasm
wasm-pack test --node
# Tests: actual WASM environment, error handling, memory management
```

**Run All Tests:**
```sh
# From workspace root - runs core + WASM interface tests
cargo test

# Also run WASM-specific tests
cd almagest-wasm && wasm-pack test --node
```

### Browse the Documentation

```sh
cargo doc --open
```

### Build for WebAssembly

To build the WASM bindings for web projects:

```sh
# Install WASM toolchain (first time only)
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

# Build WASM package
cd almagest-wasm
wasm-pack build --target web

# Serve examples (WASM requires HTTP server)
python3 -m http.server 8000
# Then visit http://localhost:8000/examples/
```
