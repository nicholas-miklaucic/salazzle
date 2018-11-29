//! This file deals provides a way of dealing with stat stages, multipliers to a stat in
//! Pokemon. There are 13 total stages, described from -6 to 6. For normal stats, stage 0 is 2/2, or
//! simply 100% of the base stat. Decreasing the stat stage by one increases the denominator by one:
//! stage -1 is 2/3 of the original stat. Increasing the stat stage by one increases the numerator
//! by one: stage 1 is 3/2 of the original stat, or a 50% boost.
//!
//! For accuracy and evasion, the rule is the same, but stage 0 is 3/3 instead of 2/2 to reduce the
//! brokenness of moves like Minimize. That means a stat stage of -3 is still just a 50% reduction.

use std::convert::From;
use std::ops::Add;

/// A stat stage, from -6 to 6 inclusive. Nomenclature follows the `bounded_integer` crate's rules: N
/// is replacing a minus sign, and P is replacing a plus sign. Z0 is 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i8)]
pub enum StatStage {
    N6 = -6,
    N5,
    N4,
    N3,
    N2,
    N1,
    Z0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6
}

impl StatStage {
    /// Gets the actual multiplier of a stat stage, applied to a normal stat (HP, Atk, Def, SpA, SpD,
    /// or Spe). For example, a stat stage of -5 is 2/7 of the original stat.
    pub fn normal_multiplier(self) -> f64 {
        if self < StatStage::Z0 {
            let numer: f64 = 2.0;
            let denom: f64 = 2.0 + (self as i8) as f64;
            numer / denom
        } else {
            // Z0 works in this case as expected, so include it
            let numer: f64 = 2.0 + (self as i8) as f64;
            let denom: f64 = 2.0;
            numer / denom
        }
    }

    /// Gets the actual multiplier of a stat stage, when applied to accuracy or evasion. For example,
    /// a stat stage of -4 is 3/7 of the original stat.
    pub fn accuracy_multiplier(self) -> f64 {
        if self < StatStage::Z0 {
            let numer: f64 = 3.0;
            let denom: f64 = 3.0 + (self as i8) as f64;
            numer / denom
        } else {
            // Z0 works in this case as expected, so include it
            let numer: f64 = 3.0 + (self as i8) as f64;
            let denom: f64 = 3.0;
            numer / denom
        }
    }
}

impl Add for StatStage {
    type Output = StatStage;

    /// A bounded addition, where the output cannot exceed 6 in either direction.
    fn add(self, other: StatStage) -> StatStage {
        let num = self as i8 + other as i8;
        if num <= -6 {
            StatStage::N6
        } else if num >= 6 {
            StatStage::P6
        } else {
            // within bounds
            match num {
                -5 => StatStage::N5,
                -4 => StatStage::N4,
                -3 => StatStage::N3,
                -2 => StatStage::N2,
                -1 => StatStage::N1,
                0 => StatStage::Z0,
                1 => StatStage::P1,
                2 => StatStage::P2,
                3 => StatStage::P3,
                4 => StatStage::P4,
                5 => StatStage::P5,
                _ => panic!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_integer_conversion() {
        assert_eq!(StatStage::N6 as i8, -6i8);
        assert_eq!(StatStage::N5 as i8, -5i8);
        assert_eq!(StatStage::N4 as i8, -4i8);
        assert_eq!(StatStage::N3 as i8, -3i8);
        assert_eq!(StatStage::N2 as i8, -2i8);
        assert_eq!(StatStage::N1 as i8, -1i8);
        assert_eq!(StatStage::Z0 as i8, 0i8);
        assert_eq!(StatStage::P1 as i8, 1i8);
        assert_eq!(StatStage::P2 as i8, 2i8);
        assert_eq!(StatStage::P3 as i8, 3i8);
        assert_eq!(StatStage::P4 as i8, 4i8);
        assert_eq!(StatStage::P5 as i8, 5i8);
        assert_eq!(StatStage::P6 as i8, 6i8);
    }
}
