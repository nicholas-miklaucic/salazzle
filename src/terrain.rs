//! This file defines the various types of terrains, field conditions that have become very relevant
//! in the USUM OU meta due to the Tapus setting them on switch in.

#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum Terrain {
    /// Electric terrain prevents sleep, including Yawn, on grounded targets, and powers up
    /// Electric-type moves used by grounded Pokemon.
    Electric,
    /// Grassy terrain restores 1/16 max HP to each grounded Pokemon each turn, halves the damage
    /// taken from Bulldoze, Earthquake, and Magnitude, and powers up Grass-type moves used by
    /// grounded Pokemon.
    Grassy,
    /// Misty terrain makes grounded Pokemon immune to status, including confusion, and halves the
    /// power of Dragon-type moves on grounded targets.
    Misty,
    /// Psychic terrain makes grounded Pokemon immune to being hit by moves with increased priority,
    /// as well as increasing the power of Psychic-type moves used by grounded Pokemon.
    Psychic,
}
