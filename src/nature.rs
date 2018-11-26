//! This file enumerates the 25 natures that Pokemon can have, along with the stats they
//! affect. [Bulbapedia](https://bulbapedia.bulbagarden.net/wiki/Nature) is used as a source.

use crate::stat::Stat;

/// One of the 25 natures a Pokemon can have. There are 5 × 5 = 25 possible natures (natures cannot
/// affect HP), 5 of which are the same because they have no effect. They are ordered left-right
/// top-down from the Bulbapedia table: Hardy is Attack+ and Attack- (so no effect), Lonely is Attack+
/// and Defense-, etc.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Nature {
    Hardy,
    Lonely,
    Adamant,
    Naughty,
    Brave,
    Bold,
    Docile,
    Impish,
    Lax,
    Relaxed,
    Modest,
    Mild,
    Bashful,
    Rash,
    Quiet,
    Calm,
    Gentle,
    Careful,
    Quirky,
    Sassy,
    Timid,
    Hasty,
    Jolly,
    Naive,
    Serious
}

impl Nature {
    /// Returns a Vector of every Nature, ordered as in the declaration. Doesn't ever change its output.
    pub fn all_natures() -> Vec<Nature> {        
        return vec![Nature::Hardy, Nature::Lonely, Nature::Adamant, Nature::Naughty, Nature::Brave,
                     Nature::Bold, Nature::Docile, Nature::Impish, Nature::Lax, Nature::Relaxed,
                     Nature::Modest, Nature::Mild, Nature::Bashful, Nature::Rash, Nature::Quiet,
                     Nature::Calm, Nature::Gentle, Nature::Careful, Nature::Quirky, Nature::Sassy,
                     Nature::Timid, Nature::Hasty, Nature::Jolly, Nature::Naive, Nature::Serious];        
    }
    /// Returns True if the Nature does affect stats, and False otherwise.
    pub fn has_stat_effect(self) -> bool {
        match self {
            Nature::Hardy | Nature::Docile | Nature::Bashful | Nature::Quirky | Nature::Serious => false,
            _ => true
        }
    }
    /// Returns the Stat boosted by this Nature. Note that you should check whether a Nature actually
    /// boosts anything before necessarily calculating anything with this in mind. For example, this
    /// will return `Atk` for `Hardy`, but `Hardy` doesn't boost `Atk`: it does nothing.
    pub fn increased_stat(self) -> Stat {
        match self {
            Nature::Hardy | Nature::Lonely | Nature::Adamant | Nature::Naughty | Nature::Brave => Stat::Atk,
            Nature::Bold | Nature::Docile | Nature::Impish | Nature::Lax | Nature::Relaxed => Stat::Def,
            Nature::Modest | Nature::Mild | Nature::Bashful | Nature::Rash | Nature::Quiet => Stat::SpA,
            Nature::Calm | Nature::Gentle | Nature::Careful | Nature::Quirky | Nature::Sassy => Stat::SpD,
            Nature::Timid | Nature::Hasty | Nature::Jolly | Nature::Naive | Nature::Serious => Stat::Spe,
        }
    }
    /// Returns the Stat decreased by this Nature. Note that you should check whether a Nature actually
    /// boosts anything before necessarily calculating anything with this in mind. For example, this
    /// will return `Atk` for `Hardy`, but `Hardy` doesn't decrease `Atk`: it does nothing.
    pub fn decreased_stat(self) -> Stat {
        match self {
            Nature::Hardy | Nature::Bold | Nature::Modest | Nature::Calm | Nature::Timid => Stat::Atk,
            Nature::Lonely | Nature::Docile | Nature::Mild | Nature::Gentle | Nature::Hasty => Stat::Def,
            Nature::Adamant | Nature::Impish | Nature::Bashful | Nature::Careful | Nature::Jolly => Stat::SpA,
            Nature::Naughty | Nature::Lax | Nature::Rash | Nature::Quirky | Nature::Naive => Stat::SpD,
            Nature::Brave | Nature::Relaxed | Nature::Quiet | Nature::Sassy | Nature::Serious => Stat::Spe,
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_neutral_stats() {
        for nat in Nature::all_natures() {
            if !nat.has_stat_effect() {
                assert_eq!(nat.increased_stat(), nat.decreased_stat());
            }
        }
    }
}
