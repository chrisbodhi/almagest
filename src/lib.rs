#![no_std]

pub mod kepler;
pub mod tethers;
pub mod utils;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
