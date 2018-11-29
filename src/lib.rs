#![feature(try_from)]

extern crate strum;
#[macro_use]
extern crate strum_macros;

pub mod stat;
pub mod typing;
pub mod nature;
pub mod species;
pub mod terrain;
pub mod weather;
pub mod stat_stage;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
