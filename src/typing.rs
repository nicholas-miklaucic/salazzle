//! This crate defines the Pokemon types: their weaknesses and resistances, coverage data, and useful
//! helper functions that get weaknesses and coverage for combined types. The data for types is, as
//! of now, current to Gen VII, and uses [Bulbapedia](https://bulbapedia.bulbagarden.net/wiki/Type) as
//! a source.

use std::convert::Into;
use std::error;
use std::fmt;
use std::f32::EPSILON;


/// Describes the relationship between two types. This makes comparison easier, because otherwise one
/// would have to compare the floating-point multipliers. Converts to a floating point that gives the
/// numerical relationship: Weakness is 2x, Resistance is 0.5x, and Immunity is 0x.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Multiplier {
    Regular,
    Weakness,
    Resistance,
    Immunity,
}

/// A generic error for converting from an invalid numeric multipliers.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InvalidNumericMultiplierError {
}

impl fmt::Display for InvalidNumericMultiplierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid numeric multiplier")
    }
}

impl error::Error for InvalidNumericMultiplierError {
    fn description(&self) -> &str {
        "given multiplier was not 0, 0.5, 1, or 2 and so is invalid"
    }
    
    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}


impl Multiplier {
    /// Attempts to convert from a floating-point multiplier to an enum value, returning a `Result`.
    pub fn from_num_multiplier(multiplier: &f32) -> Result<Multiplier, InvalidNumericMultiplierError> {
        if (multiplier - 0.0).abs() <= EPSILON {
            Ok(Multiplier::Immunity)
        } else if (multiplier - 0.5).abs() <= EPSILON {
            Ok(Multiplier::Resistance)
        } else if (multiplier - 1.0).abs() <= EPSILON {
            Ok(Multiplier::Regular)
        } else if (multiplier - 2.0).abs() <= EPSILON {
            Ok(Multiplier::Weakness)
        } else {
            Err(InvalidNumericMultiplierError{})
        }
    }
}

impl Into<f32> for Multiplier {
    fn into(self) -> f32 {
        match self {
            Multiplier::Regular => 1.,
            Multiplier::Weakness => 2.,
            Multiplier::Resistance => 0.5,
            Multiplier::Immunity => 0.,
        }
    }
}



    
/// The type weakness chart, copied from Bulbapedia and current to Gen VII.
/// The format is a flattened version of the type matrix, given in the order it appears in Bulbapedia,
/// also the order that it appears in the `Typing` enum. For example, the sixth element is 0.5,
/// because Normal deals half damage against Rock.
const TYPE_MULTIPLIERS: [f32; 324] = [
    1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,  // Normal
    2.0, 1.0, 0.5, 0.5, 1.0, 2.0, 0.5, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 2.0, 1.0, 2.0, 0.5,  // Fighting
    1.0, 2.0, 1.0, 1.0, 1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0,  // Flying
    1.0, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.5, 0.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0,  // Poison
    1.0, 1.0, 0.0, 2.0, 1.0, 2.0, 0.5, 1.0, 2.0, 2.0, 1.0, 0.5, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0,  // Ground
    1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 2.0, 1.0, 0.5, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0,  // Rock
    1.0, 0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 2.0, 1.0, 2.0, 1.0, 1.0, 2.0, 0.5,  // Bug
    0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5, 1.0,  // Ghost
    1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.5, 1.0, 2.0, 1.0, 1.0, 2.0,  // Steel
    1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 2.0, 1.0, 2.0, 0.5, 0.5, 2.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0,  // Fire
    1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0,  // Water
    1.0, 1.0, 0.5, 0.5, 2.0, 2.0, 0.5, 1.0, 0.5, 0.5, 2.0, 0.5, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0,  // Grass
    1.0, 1.0, 2.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 0.5, 1.0, 1.0, 0.5, 1.0, 1.0,  // Electric
    1.0, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 0.0, 1.0,  // Psychic
    1.0, 1.0, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 0.5, 0.5, 0.5, 2.0, 1.0, 1.0, 0.5, 2.0, 1.0, 1.0,  // Ice
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.0,  // Dragon
    1.0, 1.0, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 0.5, 0.5, 0.5, 2.0, 1.0, 1.0, 0.5, 1.0, 0.5, 0.5,  // Dark
    1.0, 2.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0,  // Fairy
];

/// A Pokemon type, although `Typing` is used to prevent any confusion with types in Rust.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Typing {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}

impl Typing {
    /// Returns an integer 0-17 in the ordering Bulbapedia and the games themselves use, the same
    /// order as they are defined in the Pokemon games.
    pub fn num_code(&self) -> u8 {
        match *self {
            Typing::Normal => 0,
            Typing::Fighting => 1,
            Typing::Flying => 2,
            Typing::Poison => 3,
            Typing::Ground => 4,
            Typing::Rock => 5,
            Typing::Bug => 6,
            Typing::Ghost => 7,
            Typing::Steel => 8,
            Typing::Fire => 9,
            Typing::Water => 10,
            Typing::Grass => 11,
            Typing::Electric => 12,
            Typing::Psychic => 13,
            Typing::Ice => 14,
            Typing::Dragon => 15,
            Typing::Dark => 16,
            Typing::Fairy => 17,
        }
    }
    /// Returns the multiplier a move this `Typing` has when attacking a Pokemon with the given other `Typing`.
    pub fn offense_multiplier(&self, other: &Typing) -> Multiplier {
        // get index in flattened matrix
        let index: usize = (self.num_code() as usize * 18) + (other.num_code() as usize);
        Multiplier::from_num_multiplier(&TYPE_MULTIPLIERS[index]).unwrap()
    }
    /// Returns the multiplier a Pokemon with this `Typing` has when being attacked with a move of the given other `Typing`.
    pub fn defense_multiplier(&self, other: &Typing) -> Multiplier {
        other.offense_multiplier(self)
    }
    /// Given a Vector of typings, 
}
