//! This file provides a simple way of dealing with Pokemon stats, of which there are 6: HP, Attack,
//! Defense, Special Attack, Special Defense, and Speed.

use std::fmt;

/// One of the six Pokemon stats. The abbreviated names are used to reduce ambiguity in
/// nomenclature. The long forms are used for string conversion.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Stat {
    HP,
    Atk,
    Def,
    SpA,
    SpD,
    Spe
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Stat::HP => "HP",
            Stat::Atk => "Attack",
            Stat::Def => "Defense",
            Stat::SpA => "Special Attack",
            Stat::SpD => "Special Defense",
            Stat::Spe => "Speed",
        })
    }
}

