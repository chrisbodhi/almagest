use almagest::{
    materials::fibers::KEVLAR_49,
    utils::{Kilograms, Kilometers, MetersPerSecond},
};
use modeling::Tether;

pub fn main() {
    let t = Tether::new(
        Kilometers(100.0),
        Kilometers(95.0),
        Kilograms(100.0),
        KEVLAR_49,
        MetersPerSecond(7_000.0),
    );
    println!("{:#?}", t);
}
