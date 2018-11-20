//! This crate defines the Pokemon types: their weaknesses and resistances, coverage data, and useful
//! helper functions that get weaknesses and coverage for combined types. The data for types is, as
//! of now, current to Gen VII, and uses [Bulbapedia](https://bulbapedia.bulbagarden.net/wiki/Type) as
//! a source.

use std::convert::{Into, TryFrom};
use std::error;
use std::fmt;
use std::ops::{Mul};
use std::f32::EPSILON;


/// Describes the relationship between two types. This makes comparison easier, because otherwise one
/// would have to compare the floating-point multipliers. Converts to a floating point that gives the
/// numerical relationship: Weakness is 2x, Resistance is 0.5x, and Immunity is 0x. Double weakness is
/// 4x, and double resistance is 0.25x.
/// 
/// Multiplication works the same way as multiplying the numeric factors, but values outside of 4x
/// weakness or resistance are clamped within. Ordering is also the same as ordering on the numeric
/// multipliers.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Multiplier {
    Immunity,
    DoubleResistance,
    Resistance,
    Regular,
    Weakness,
    DoubleWeakness,
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
        "given multiplier was not 0, 0.25, 0.5, 1, 2, or 4, and so is invalid"
    }
    
    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}


impl Multiplier {
    /// Attempts to convert from a floating-point multiplier to an enum value, returning a `Result`.
    pub fn from_num_multiplier(multiplier: f32) -> Result<Multiplier, InvalidNumericMultiplierError> {
        if (multiplier - 0.0).abs() <= EPSILON {
            Ok(Multiplier::Immunity)
        } else if (multiplier - 0.25).abs() <= EPSILON {
            Ok(Multiplier::DoubleResistance)
        } else if (multiplier - 0.5).abs() <= EPSILON {
            Ok(Multiplier::Resistance)
        } else if (multiplier - 1.0).abs() <= EPSILON {
            Ok(Multiplier::Regular)
        } else if (multiplier - 2.0).abs() <= EPSILON {
            Ok(Multiplier::Weakness)
        } else if (multiplier - 4.0).abs() <= EPSILON {
            Ok(Multiplier::DoubleWeakness)
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
            Multiplier::DoubleWeakness => 4.,
            Multiplier::Resistance => 0.5,
            Multiplier::DoubleResistance => 0.25,
            Multiplier::Immunity => 0.,
        }
    }
}


impl Mul<Multiplier> for Multiplier {
    type Output = Multiplier;

    /// Combines multipliers as expected. Note that multipliers don't go past double, so
    /// DoubleWeakness times Weakness is DoubleWeakness.
    fn mul(self, _rhs: Multiplier) -> Multiplier {
        let mul1: f32 = self.into();
        let mul2: f32 = _rhs.into();
        let num = mul1 * mul2;
        if (num - 0.125).abs() <= EPSILON {
            Multiplier::DoubleResistance
        } else if (num - 8.0).abs() <= EPSILON {
            Multiplier::DoubleWeakness
        } else {
            Multiplier::from_num_multiplier(num).unwrap()
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
    1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5, 0.5,  // Dark
    1.0, 2.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0,  // Fairy
];

/// A generic error for converting from an invalid numeric multipliers.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InvalidTypingCodeError {
}

impl fmt::Display for InvalidTypingCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid numeric code for typing")
    }
}

impl error::Error for InvalidTypingCodeError {
    fn description(&self) -> &str {
        "given code was not in range 0-17 and so is invalid"
    }
    
    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

/// A Pokemon type, although `Typing` is used to prevent any confusion with types in Rust.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
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

impl TryFrom<u8> for Typing {
    type Error = InvalidTypingCodeError;

    fn try_from(code: u8) -> Result<Typing, InvalidTypingCodeError> {
        match code {
            0 => Ok(Typing::Normal),
            1 => Ok(Typing::Fighting),
            2 => Ok(Typing::Flying),
            3 => Ok(Typing::Poison),
            4 => Ok(Typing::Ground),
            5 => Ok(Typing::Rock),
            6 => Ok(Typing::Bug),
            7 => Ok(Typing::Ghost),
            8 => Ok(Typing::Steel),
            9 => Ok(Typing::Fire),
            10 => Ok(Typing::Water),
            11 => Ok(Typing::Grass),
            12 => Ok(Typing::Electric),
            13 => Ok(Typing::Psychic),
            14 => Ok(Typing::Ice),
            15 => Ok(Typing::Dragon),
            16 => Ok(Typing::Dark),
            17 => Ok(Typing::Fairy),
            _ => Err(InvalidTypingCodeError{})
        }
    }
}

impl Typing {
    /// Returns a `Vector` of all of the `Typing`s, in numerical order. Always returns the same value.
    pub fn all_typings() -> Vec<Typing> {
        (0..18).map(|x| Typing::try_from(x).unwrap()).collect()
    }
    /// Returns an integer 0-17 in the ordering Bulbapedia and the games themselves use, the same
    /// order as they are defined in the Pokemon games.  NOTE: Because enums now implement
    /// discriminants, `t as u8` should return the same thing as `t.num_code()`.
    pub fn num_code(self) -> u8 {
        match self {
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
    pub fn offense_multiplier(self, other: Typing) -> Multiplier {
        // get index in flattened matrix
        let index: usize = (self.num_code() as usize * 18) + (other.num_code() as usize);
        Multiplier::from_num_multiplier(TYPE_MULTIPLIERS[index]).unwrap()
    }
    /// Returns a `Vec` of 18 `Multiplier`s, indicating the offensive multiplier this `Typing`
    /// receives on each other typing, in numerical order.
    pub fn offense_multipliers(self) -> Vec<Multiplier> {
        Typing::all_typings().into_iter().map(|t| self.offense_multiplier(t)).collect()
    }
    /// Returns the multiplier a Pokemon with this `Typing` has when being attacked with a move of the given other `Typing`.
    pub fn defense_multiplier(self, other: Typing) -> Multiplier {
        other.offense_multiplier(self)
    }
    /// Returns a `Vec` of 18 `Multiplier`s, indicating the defensive multiplier this `Typing`
    /// receives on each other typing, in numerical order.
    pub fn defense_multipliers(self) -> Vec<Multiplier> {
        Typing::all_typings().into_iter().map(|t| self.defense_multiplier(t)).collect()
    }  
    /// Given two typings, determines the effectiveness of an attack of this `Typing` has when
    /// attacking a Pokemon with the given combination of `Typing`s.
    pub fn combined_effectiveness(self, other: (Typing, Typing)) -> Multiplier {
        let (typing1, typing2) = other;
        self.offense_multiplier(typing1) * self.offense_multiplier(typing2)
    }
    /// Returns a `Vec` of `Typing`s that hit this `Typing` super effectively, sorted by numeric ID.
    pub fn weak_to(self) -> Vec<Typing> {
        Typing::all_typings().into_iter().filter(|&t| t.offense_multiplier(self) == Multiplier::Weakness).collect()
    }
    /// Returns a `Vec` of `Typing`s that hit this `Typing` not very effectively, sorted by numeric ID.
    pub fn resistant_to(self) -> Vec<Typing> {
        Typing::all_typings().into_iter().filter(|&t| t.offense_multiplier(self) == Multiplier::Resistance).collect()
    }
    /// Returns a `Vec` of `Typing`s that hit this `Typing` for neutral damage, sorted by numeric ID.
    pub fn neutral_to(self) -> Vec<Typing> {
        Typing::all_typings().into_iter().filter(|&t| t.offense_multiplier(self) == Multiplier::Regular).collect()
    }
    /// Returns a `Vec` of `Typing`s that this `Typing` is immune to damage from, sorted by numeric ID.
    pub fn immune_to(self) -> Vec<Typing> {
        Typing::all_typings().into_iter().filter(|&t| t.offense_multiplier(self) == Multiplier::Immunity).collect()
    }
    /// Returns a `Vec` of `Typing`s that this `Typing` hits super effectively, sorted by numeric ID.
    pub fn weak_against(self) -> Vec<Typing> {
        Typing::all_typings().into_iter().filter(|&t| self.offense_multiplier(t) == Multiplier::Weakness).collect()
    }
    /// Returns a `Vec` of `Typing`s that this `Typing` hits not very effectively, sorted by numeric ID.
    pub fn resistant_against(self) -> Vec<Typing> {
        Typing::all_typings().into_iter().filter(|&t| self.offense_multiplier(t) == Multiplier::Resistance).collect()
    }
    /// Returns a `Vec` of `Typing`s that this `Typing` hits for neutral damage, sorted by numeric ID.
    pub fn neutral_against(self) -> Vec<Typing> {
        Typing::all_typings().into_iter().filter(|&t| self.offense_multiplier(t) == Multiplier::Regular).collect()
    }
    /// Returns a `Vec` of `Typing`s that this `Typing` is immune to damage from, sorted by numeric ID.
    pub fn immune_against(self) -> Vec<Typing> {
        Typing::all_typings().into_iter().filter(|&t| self.offense_multiplier(t) == Multiplier::Immunity).collect()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;    
    #[test]
    fn test_offense_multipliers() {
        assert_eq!(Typing::Ground.offense_multiplier(Typing::Flying), Multiplier::Immunity);
        assert_eq!(Typing::Water.offense_multiplier(Typing::Fire), Multiplier::Weakness);
        assert_eq!(Typing::Normal.offense_multiplier(Typing::Grass), Multiplier::Regular);
        assert_eq!(Typing::Fighting.offense_multiplier(Typing::Psychic), Multiplier::Resistance);
    }
    #[test]
    fn test_defense_multipliers() {
        assert_eq!(Typing::Ghost.defense_multiplier(Typing::Normal), Multiplier::Immunity);
        assert_eq!(Typing::Flying.defense_multiplier(Typing::Electric), Multiplier::Weakness);
        assert_eq!(Typing::Steel.defense_multiplier(Typing::Dark), Multiplier::Regular);
        assert_eq!(Typing::Fairy.defense_multiplier(Typing::Bug), Multiplier::Resistance);
    }
    #[test]
    fn test_weak_against() {
        assert_eq!(Typing::Ice.weak_against(),
                   vec![Typing::Flying, Typing::Ground, Typing::Grass, Typing::Dragon]);
        assert_eq!(Typing::Dragon.weak_against(), vec![Typing::Dragon]);
    }
    #[test]
    fn test_neutral_against() {
        assert_eq!(Typing::Bug.neutral_against(),
                   vec![Typing::Normal, Typing::Ground, Typing::Rock, Typing::Bug, Typing::Water,
                        Typing::Electric, Typing::Ice, Typing::Dragon]);
        assert_eq!(Typing::Poison.neutral_against(),
                   vec![Typing::Normal, Typing::Fighting, Typing::Flying, Typing::Bug, Typing::Fire,
                        Typing::Water, Typing::Electric, Typing::Psychic, Typing::Ice, Typing::Dragon,
                        Typing::Dark]);
    }
    #[test]
    fn test_resistant_against() {
        assert_eq!(Typing::Fighting.resistant_against(),
                   vec![Typing::Flying, Typing::Poison, Typing::Bug, Typing::Psychic, Typing::Fairy]);
        assert_eq!(Typing::Grass.resistant_against(),
                   vec![Typing::Flying, Typing::Poison, Typing::Bug, Typing::Steel, Typing::Fire,
                        Typing::Grass, Typing::Dragon]);
    }
}
