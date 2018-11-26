//! This file enumerates the possible weather conditions in Pokemon. Harsh sunlight, heavy rain, and
//! mysterious air currents are all illegal in OU (Groudon and Kyogre's Primal Reversions are both
//! banned, as is Mega Rayquaza), but for completeness's sake I include them nontheless.

/// Each type of weather that can appear in Pokemon. Normal is just the designation for a battle
/// without any other weather currently in effect.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Weather {
    Normal,
    Rain,
    HeavyRain,
    Sun,
    HarshSun,
    Sand,
    Hail,
    StrongWinds
}

impl Weather {
    /// Returns True if the weather is set by a forme of one of the RSE cover legendaries. This means
    /// that it does not persist on switching out, and it suppresses non-special weathers.
    pub fn is_special(self) -> bool {
        match self {
            HeavyRain | HarshSun | StrongWinds => true,
            _ => false
        }
    }
}
