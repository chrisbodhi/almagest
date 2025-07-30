# almagest
_an astronomical library_

⚠️ This library is in active development. As such, there are no guarantees of API stability or even correctness. ⚠️

Rust edition 2024

>Keep in mind that I lay no claim to having discovered these things<br/>
>through my own skill. I am but an ignorant compiler of the works of<br/>
>ancient astronomers, and have but put their material into my own<br/>
>words for your instruction; and with this sword shall I slay envy.<br/>

&mdash; Geoffrey Chaucer, _Prologue of his Treatise on the Astrolabe_, by way of Albert Waugh in _Sundials: Their Theory and Construction_

## Overview

Almagest is a Rust library for astrodynamics, providing tools for orbit determination, propagation, and analysis. It's written without the usage of the `std` library, relying on the `no_std` feature for minimal dependencies and maximum deployment flexibility.

## Development

Install Rust edition 2024 and, with it, `cargo`, et al.

### Browse the Documentation

To browse the documentation, go to the `almagest` directory and run:

```sh
cargo doc --open
```

### Build and Test

To build and test the library, go to the `almagest` directory and run:

```sh
cargo build
cargo test
```
