# A history of motion

## Example code that cannot be run

```rust,ignore
use almagest::materials::fibers;
use almagest::tethers::characteristic_velocity_for_material;

fn main() {
    let velocity = characteristic_velocity_for_material(&fibers::PBO).unwrap();
    println!("PBO characteristic velocity: {} m/s", velocity.0);
}
```

## Example code that can be run, but is tedious

This is tedious because so much needs to be imported, line by line.
We can hide that from the user by using the `hidden` attribute on
the code block.

```rust,runnable
{{#include ../../../almagest/src/utils.rs:62}}
{{#include ../../../almagest/src/utils.rs:212}}
{{#include ../../../almagest/src/utils.rs:233:237}}
println!("{} Seconds", Seconds(44.0).value());
```

We reach the limits of this approach when we try to use external
crates or other modules in our crate.

## Example code that includes the entire file, but hides it

This can be good for testing self-contained code using
`mdbook test`, but something like this

```rust,ignore
{{#rustdoc_include ../../../almagest/src/tethers.rs:263:274}}
```

that uses the external crate `libm` and local modules will fail
the `test` command.
