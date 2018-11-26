//! This module defines Pokemon species. There are as of now 807 of them, including everything up to
//! Blacephalon and not including Meltan and Melmetal, who do not appear in the core series
//! games. The ordering is defined by National Dex number, with
//! [Bulbapedia](https://bulbapedia.bulbagarden.net/wiki/List_of_Pokémon_by_National_Pokédex_number)
//! as a reference source. The actual values were copied from [this
//! CSV](https://raw.githubusercontent.com/veekun/pokedex/master/pokedex/data/csv/pokemon.csv), with
//! heavy modifications.
//!
//! What a Pokemon "is" as we normally think of it usually just corresponds with the species, except
//! when it doesn't. In this library the exceptions, when multiple Pokemon correspond with a single
//! dex number and species, are called *formes*. The species that have formes have an associated
//! enum as an associated value. This means that any specification of a Pokemon species that the
//! compiler allows refers to a Pokemon with completely determined base stats, cry, appearance,
//! etc., and vice versa.
//!
//! This library takes the liberty of not including some formes that don't matter in battle: the
//! Pikachu cosplay and hat variants, spiky-eared Pichu, Unown letters, the Basculin stripe formes,
//! the Shellos/Gastrodon sea formes, the Sawsbuck formes, the Keldeo formes, the Vivillon formes,
//! the Florges formes, the Furfrou formes, the Xerneas cosmetic changes, the Minior core color
//! changes (but not the difference between Minior-Meteor and Minior), the Magearna color changes,
//! and the Cherrim formes. (The last one seems important, but Flower Gift is implemented as giving
//! these stats directly, so there's no need for a purely cosmetic change.) These are equivalent in
//! the context of battling Pokemon (although perhaps not for determining legitimacy of sets, but
//! that's beyond the scope of Salazzle) and so it's just not worth the hassle.
//!
//! The Genesect formes (Shock, Burn, Chill, Douse) are a special case: they do have an effect, but
//! it's coded as a property of the move Techno Blast, which is the only effect besides
//! appearance. Thus, Genesect does not have specific formes in this library.

use std::fmt;

/// The Castform formes: Normal, Sunny, Rainy, and Snowy. These don't change stats, but they do
/// change typing to the one resembling the weather: Normal, Fire, Water, and Ice. This would be a
/// super cool game mechanic if Castform's stats were at all usable in competitive play: as it stands
/// this is not going to ever be used, and it mainly tests how comfortable you are with camel case
/// (bet you're glad I have the extra e in forme!)
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum CastformForme {
    Normal,
    Sunny,
    Rainy,
    Snowy,    
}

/// The Deoxys formes. These change stats and move compatibility, the first Pokemon to have such a
/// mechanic. These *are* competitively relevant, unlike Castform: Deoxys-Speed has the highest Speed
/// stat in the game, Deoxys-Attack has a bonkers 180 Atk and SpA, and Deoxys-Defense has 160 Def and
/// SpD.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum DeoxysForme {
    Normal,
    Attack,
    Defense,
    Speed,    
}

/// The Wormadam formes. The Burmy formes that determine a Wormadam's forme upon evolution are purely
/// cosmetic: these, however, impact typing and move compatiblity. Compared to Deoxys, Trash is
/// probably a pretty good descriptor; nonetheless, here they are.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum WormadamForme {
    Plant,
    Sandy,
    Trash
}

/// The Rotom formes. These change typing, and stats between the "ghost" normal forme and the ones
/// after Rotom possesses an appliance, but the choice of appliance doesn't affect base stats. It
/// does, however, affect typing and move compatibility.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum RotomForme {
    /// The unevolved form of Rotom. It's just called "Rotom", so Ghost is used as a name.
    Ghost,
    Heat,
    Wash,
    Frost,
    Fan,
    Mow
}

/// The Giratina formes. There are two: the Altered forme has 100 attacking stats and 120 defensive
/// stats, and the Origin forme switches them. Their abilities also differ (Pressure/Telepathy and
/// Levitate respectively).
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum GiratinaForme {
    Altered,
    Origin
}

/// The fabled Arceus formes. These are interesting in that they differ depending on held item, unlike
/// most other formes. These each change the typing of Judgment and the base typing of Arceus
/// itself. As such, there's one forme for each typing. An interesting small but important detail is
/// that Arceus-Dragon can learn Draco Meteor, which no other Arceus forme can. This doesn't matter
/// competitively, as you could just switch the plates after, but it's good to know.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum ArceusForme {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy
}

/// The Darminitan formes. In a mechanic that will become very common, Darmanitan switches to Zen Mode
/// if its HP is below half at the end of a turn. It changes typing and stats.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum DarmanitanForme {
    Standard,
    /// Represented as "Zen Mode" in-game.
    ZenMode
}

/// The Kyurem formes. These are important: they change some learned moves and base
/// stats. Kyurem-White is a specially-speced upgrade from normal Kyurem, and Kyurem-Black is a
/// physically-speced version of normal Kyurem.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum KyuremForme {
    /// In the game this is just "Kyurem", so Normal is arbitrary.
    Normal,
    Black,
    White
}

/// The Meloetta formes. These are unique in that Meloetta switches between them by using Relic Song
/// in battle: out of battle and at the start of battles, it is in Meloetta-Aria forme. These change
/// typing and stats.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum MeloettaForme {
    Aria,
    Pirouette
}


/// The Greninja formes, new as of Gen VII. This is a unique forme change mechanic: the way to turn
/// from Greninja into Greninja-Ash is to have the ability Battle Bond and KO an
/// opponent. Greninja-Ash has different properties on the move Water Shuriken (base power 20, hits 3
/// times without randomness, as opposed to 15 BP and 2-5 hits) and different stats. In the game,
/// there are three formes of Greninja: Protean/Torrent, pre-bond, and Greninja-Ash. The nomenclature
/// for this isn't well-defined in the main games, so in this library Greninja-Normal is Torrent or
/// Protean, BattleBond is pre-bond Greninja with Battle Bond, and Ash is Greninja-Ash.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum GreninjaForme {
    /// A Greninja without Battle Bond as an ability.
    Normal,
    /// A Greninja with the ability Battle Bond, but not actually transformed into Greninja-Ash. These
    /// are always male and cannot breed, but otherwise have the same typing, movepool, and stats as
    /// normal Greninja.
    BattleBond,
    Ash
}


/// The Gourgeist (and Pumpkaboo) formes. The main difference here is just base stats: it's
/// technically true that Gourgeist-Super and Gourgeist-Small can't learn Insomnia natively,
/// Gourgeist-Small can by breeding and an event Gourgeist-Super has Insomnia, so in competitive play
/// that difference doesn't matter.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum GourgeistForme {
    Small,
    Average,
    Large,
    Super
}

/// The Zygarde formes. Zygarde-10% and Zygarde-50% are both different Pokemon with different base
/// stats. A Zygarde with Power Construct, once it reaches less than 50% HP at the end of a turn,
/// converts into Zygarde-Complete with the same *lost HP* (which, factoring in the much higher base
/// HP of Zygarde-Complete, means that the HP will increase considerably). The different formes have
/// different base stats but not different typing or movepool. Because 10% and 50% are not valid
/// identifiers, `TenPercent` and `FiftyPercent` are used instead.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum ZygardeForme {
    TenPercent,
    FiftyPercent,
    Complete
}


/// The Hoopa formes. These are completely different Pokemon, with different stats, movepool, typing,
/// and appearance.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum HoopaForme {
    Confined,
    Unbound
}


/// The Oricorio formes. These change Oricorio's typing, which is also important because Revelation
/// Dance uses the user's primary type. Because `Pom-Pom` and `Pa'u` are not valid identifiers,
/// `PomPom` and `Pau` are used instead.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum OricorioForme {
    /// The Fire-type Oricorio.
    Baile,
    /// The Electric-type Oricorio, which is displayed, "Pom-Pom" in-game.
    PomPom,
    /// The Psychic-type Oricorio, which is displayed "Pa'u" in-game.
    Pau,
    /// The Ghost-type Oricorio.
    Sensu
}

/// The Lycanroc formes. These change movepool, ability, and base stats.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum LycanrocForme {
    Midday,
    Midnight,
    Dusk
}


/// The Wishiwashi formes. Wishiwashi starts out in School forme, changing to Solo forme when its HP
/// reaches 25% of its maximum HP at the end of a turn. Base stats are the only difference.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum WishiwashiForme {
    School,
    Solo
}

/// The Shaymin formes. These have different typings, stats, movepools, and abilities: it's easier to
/// think of them as different Pokemon.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum ShayminForme {    
    Land,
    Sky
}


/// The Silvally (and Type: Null) formes. These mirror Arceus formes in depending on held item and
/// affecting a single attack, Multi-Attack instead of Judgment. As such, it has the exact same values
/// as `ArceusForme`.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum SilvallyForme {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy
}


/// The Minior formes. These behave like Wishiwashi's formes, only with the cutoff at 50% of max
/// HP. There are also different Core colors, but those aren't competitively relevant. These formes
/// have different base stats, and Meteor Form Minior has status immunity due to Shields Down.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum MiniorForme {
    Meteor,
    Core
}


/// The Mimikyu formes. Mimikyu has its disguise at the start of battle, and upon being hit by any
/// attack loses it and turns into Mimikyu-Busted, not taking any damage. There are no other changes:
/// it is just a flag for whether Disguise is still active.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum MimikyuForme {
    Disguised,
    Busted
}


/// The Necrozma formes. The Solgaleo and Lunaala formes are both essentially different Pokemon
/// completely, while Ultra Necrozma is an evolution in-battle using Ultranecrozmium Z. These each
/// differ in stats, typing, and ability. Due to hyphens not being allowed in identifiers, they have
/// been elided.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum NecrozmaForme {
    /// The standard Necrozma found in Sun and Moon.
    Normal,
    /// The Solgaleo form found in Ultra Sun, written "Dusk Mane" in game.
    DuskMane,
    /// The Lunaala form found in Ultra Moon, written "Dawn Wings" in game.
    DuskWings,
    /// The Ultra form evolved into while in battle.
    Ultra
}

/// The Alola formes. These change typing, ability, base stats, and movepool, and are basically
/// completely different Pokemon.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum AlolaForme {
    Normal,
    Alola
}


/// The most common type of forme: Mega Evolution. This is for Pokemon with only one Mega Evolution:
/// some special Pokemon have both an X and Y Mega Evolution, and those have a separate enumerated
/// type. These don't change HP, but they add 100 total points in base stats and can change typing
/// and ability.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum MegaEvolution {
    Normal,
    Mega
}


/// The rarer kind of Mega Evolution, one where there is both an X and Y evolution. The only Pokemon
/// with this are Charizard and Mewtwo.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum XYMegaEvolution {
    Normal,
    /// This is represented as "Mega-X" in-game.
    MegaX,
    /// This is represented as "Mega-Y" in-game.
    MegaY
}


/// The Primal Reversions, Groudon and Kyogre formes that have different abilities, typings, and
/// stats. A held item causes the switch on switch-in, and it can happen multiple times unlike Mega
/// Evolutions.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum PrimalReversion {
    Normal,
    Primal
}

/// The Genie formes: Incarnate and Therian. These are closer to different Pokemon than different
/// formes, with differing typing, ability, stats, and movepool. These affect Thundurus, Tornadus, and
/// Landorus.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum GenieForme {
    Incarnate,
    Therian
}


/// The Aegislash formes. Aegislash uses the Sword form to attack, and the Shield forme to defend:
/// using different types of moves toggles the two.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumString)]
pub enum AegislashForme {
    Sword,
    Shield
}




/// A Pokemon species. Note that, because Rust doesn't allow it, what would be `Nidoran♀` is
/// `NidoranF` and what would be `Nidoran♂` is `NidoranM`. All other punctuation has also been
/// removed, and the result is camel-cased, with hyphens denoting word boundaries. For example,
/// Kommo-o is represented as `KommoO` but Farfetch'd is `Farfetchd`. Despite these necessary
/// modifications, these species are all serialized as they would appear in-game, so converting
/// `Species::NidoranF` to a string will give you `Nidoran♀`.
///
/// `Species` implements `FromStr`: both the exact name of the enum and the official in-game version
/// can be used: both "Tapu Bulu" and "TapuBulu" convert to `Species::TapuBulu`. One can also iterate
/// through each species, with `Species::iter()`.
///
/// One final caution: Pokemon species doesn't actually tell you much: Pokemon *formes* are far more
/// useful. Pokemon species doesn't tell you typing (Persian vs. Persian-Alola, Oricorio formes,
/// etc.), base stats (Meloetta-Aria vs. Meloetta-Pirouette), or much else. Think of this enum more
/// as just a way of making the validity of Pokemon species checkable in the type system, rather than
/// as a useful piece of information in its own right.
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumDiscriminants)]
#[strum_discriminants(name(SpeciesDiscriminant), derive(Display, Hash, EnumString, EnumIter))]
pub enum Species {
    Bulbasaur,
    Ivysaur,
    Venusaur(MegaEvolution),
    Charmander,
    Charmeleon,
    Charizard(XYMegaEvolution),
    Squirtle,
    Wartortle,
    Blastoise(MegaEvolution),
    Caterpie,
    Metapod,
    Butterfree,
    Weedle,
    Kakuna,
    Beedrill(MegaEvolution),
    Pidgey,
    Pidgeotto,
    Pidgeot(MegaEvolution),
    Rattata(AlolaForme),
    Raticate(AlolaForme),
    Spearow,
    Fearow,
    Ekans,
    Arbok,
    Pikachu,
    Raichu(AlolaForme),
    Sandshrew(AlolaForme),
    Sandslash(AlolaForme),
    #[strum(serialize="Nidoran♀", serialize="NidoranF")]
    NidoranF,
    Nidorina,
    Nidoqueen,
    #[strum(serialize="Nidoran♂", serialize="NidoranM")]
    NidoranM,
    Nidorino,
    Nidoking,
    Clefairy,
    Clefable,
    Vulpix(AlolaForme),
    Ninetales(AlolaForme),
    Jigglypuff,
    Wigglytuff,
    Zubat,
    Golbat,
    Oddish,
    Gloom,
    Vileplume,
    Paras,
    Parasect,
    Venonat,
    Venomoth,
    Diglett(AlolaForme),
    Dugtrio(AlolaForme),
    Meowth,
    Persian(AlolaForme),
    Psyduck,
    Golduck,
    Mankey,
    Primeape,
    Growlithe,
    Arcanine,
    Poliwag,
    Poliwhirl,
    Poliwrath,
    Abra,
    Kadabra,
    Alakazam(MegaEvolution),
    Machop,
    Machoke,
    Machamp,
    Bellsprout,
    Weepinbell,
    Victreebel,
    Tentacool,
    Tentacruel,
    Geodude(AlolaForme),
    Graveler(AlolaForme),
    Golem(AlolaForme),
    Ponyta,
    Rapidash,
    Slowpoke,
    Slowbro,
    Magnemite,
    Magneton,
    #[strum(serialize="Farfetch'd", serialize="Farfetchd")]
    Farfetchd,
    Doduo,
    Dodrio,
    Seel,
    Dewgong,
    Grimer(AlolaForme),
    Muk(AlolaForme),
    Shellder,
    Cloyster,
    Gastly,
    Haunter,
    Gengar(MegaEvolution),
    Onix,
    Drowzee,
    Hypno,
    Krabby,
    Kingler,
    Voltorb,
    Electrode,
    Exeggcute,
    Exeggutor(AlolaForme),
    Cubone,
    Marowak(AlolaForme),
    Hitmonlee,
    Hitmonchan,
    Lickitung,
    Koffing,
    Weezing,
    Rhyhorn,
    Rhydon,
    Chansey,
    Tangela,
    Kangaskhan(MegaEvolution),
    Horsea,
    Seadra,
    Goldeen,
    Seaking,
    Staryu,
    Starmie,
    #[strum(serialize="Mr. Mime", serialize="MrMime")]
    MrMime,
    Scyther,
    Jynx,
    Electabuzz,
    Magmar,
    Pinsir(MegaEvolution),
    Tauros,
    Magikarp,
    Gyarados(MegaEvolution),
    Lapras,
    Ditto,
    Eevee,
    Vaporeon,
    Jolteon,
    Flareon,
    Porygon,
    Omanyte,
    Omastar,
    Kabuto,
    Kabutops,
    Aerodactyl(MegaEvolution),
    Snorlax,
    Articuno,
    Zapdos,
    Moltres,
    Dratini,
    Dragonair,
    Dragonite,
    Mewtwo(XYMegaEvolution),
    Mew,
    // gen 2 starts here
    Chikorita,
    Bayleef,
    Meganium,
    Cyndaquil,
    Quilava,
    Typhlosion,
    Totodile,
    Croconaw,
    Feraligatr,
    Sentret,
    Furret,
    Hoothoot,
    Noctowl,
    Ledyba,
    Ledian,
    Spinarak,
    Ariados,
    Crobat,
    Chinchou,
    Lanturn,
    Pichu,
    Cleffa,
    Igglybuff,
    Togepi,
    Togetic,
    Natu,
    Xatu,
    Mareep,
    Flaaffy,
    Ampharos,
    Bellossom,
    Marill,
    Azumarill,
    Sudowoodo,
    Politoed,
    Hoppip,
    Skiploom,
    Jumpluff,
    Aipom,
    Sunkern,
    Sunflora,
    Yanma,
    Wooper,
    Quagsire,
    Espeon,
    Umbreon,
    Murkrow,
    Slowking,
    Misdreavus,
    Unown,
    Wobbuffet,
    Girafarig,
    Pineco,
    Forretress,
    Dunsparce,
    Gligar,
    Steelix(MegaEvolution),
    Snubbull,
    Granbull,
    Qwilfish,
    Scizor(MegaEvolution),
    Shuckle,
    Heracross(MegaEvolution),
    Sneasel,
    Teddiursa,
    Ursaring,
    Slugma,
    Magcargo,
    Swinub,
    Piloswine,
    Corsola,
    Remoraid,
    Octillery,
    Delibird,
    Mantine,
    Skarmory,
    Houndour,
    Houndoom(MegaEvolution),
    Kingdra,
    Phanpy,
    Donphan,
    Porygon2,
    Stantler,
    Smeargle,
    Tyrogue,
    Hitmontop,
    Smoochum,
    Elekid,
    Magby,
    Miltank,
    Blissey,
    Raikou,
    Entei,
    Suicune,
    Larvitar,
    Pupitar,
    Tyranitar(MegaEvolution),
    Lugia,
    #[strum(serialize="Ho-Oh", serialize="HoOh")]
    HoOh,
    Celebi,
    // gen 3 starts here
    Treecko,
    Grovyle,
    Sceptile(MegaEvolution),
    Torchic,
    Combusken,
    Blaziken(MegaEvolution),
    Mudkip,
    Marshtomp,
    Swampert(MegaEvolution),
    Poochyena,
    Mightyena,
    Zigzagoon,
    Linoone,
    Wurmple,
    Silcoon,
    Beautifly,
    Cascoon,
    Dustox,
    Lotad,
    Lombre,
    Ludicolo,
    Seedot,
    Nuzleaf,
    Shiftry,
    Taillow,
    Swellow,
    Wingull,
    Pelipper,
    Ralts,
    Kirlia,
    Gardevoir(MegaEvolution),
    Surskit,
    Masquerain,
    Shroomish,
    Breloom,
    Slakoth,
    Vigoroth,
    Slaking,
    Nincada,
    Ninjask,
    Shedinja,
    Whismur,
    Loudred,
    Exploud,
    Makuhita,
    Hariyama,
    Azurill,
    Nosepass,
    Skitty,
    Delcatty,
    Sableye(MegaEvolution),
    Mawile(MegaEvolution),
    Aron,
    Lairon,
    Aggron(MegaEvolution),
    Meditite,
    Medicham(MegaEvolution),
    Electrike,
    Manectric(MegaEvolution),
    Plusle,
    Minun,
    Volbeat,
    Illumise,
    Roselia,
    Gulpin,
    Swalot,
    Carvanha,
    Sharpedo(MegaEvolution),
    Wailmer,
    Wailord,
    Numel,
    Camerupt(MegaEvolution),
    Torkoal,
    Spoink,
    Grumpig,
    Spinda,
    Trapinch,
    Vibrava,
    Flygon,
    Cacnea,
    Cacturne,
    Swablu,
    Altaria(MegaEvolution),
    Zangoose,
    Seviper,
    Lunatone,
    Solrock,
    Barboach,
    Whiscash,
    Corphish,
    Crawdaunt,
    Baltoy,
    Claydol,
    Lileep,
    Cradily,
    Anorith,
    Armaldo,
    Feebas,
    Milotic,
    Castform(CastformForme),
    Kecleon,
    Shuppet,
    Banette,
    Duskull,
    Dusclops,
    Tropius,
    Chimecho,
    Absol,
    Wynaut,
    Snorunt,
    Glalie,
    Spheal,
    Sealeo,
    Walrein,
    Clamperl,
    Huntail,
    Gorebyss,
    Relicanth,
    Luvdisc,
    Bagon,
    Shelgon,
    Salamence(MegaEvolution),
    Beldum,
    Metang,
    Metagross(MegaEvolution),
    Regirock,
    Regice,
    Registeel,
    Latias,
    Latios,
    Kyogre(PrimalReversion),
    Groudon(PrimalReversion),
    Rayquaza(MegaEvolution),
    Jirachi,
    Deoxys(DeoxysForme),
    // gen 4 starts here
    Turtwig,
    Grotle,
    Torterra,
    Chimchar,
    Monferno,
    Infernape,
    Piplup,
    Prinplup,
    Empoleon,
    Starly,
    Staravia,
    Staraptor,
    Bidoof,
    Bibarel,
    Kricketot,
    Kricketune,
    Shinx,
    Luxio,
    Luxray,
    Budew,
    Roserade,
    Cranidos,
    Rampardos,
    Shieldon,
    Bastiodon,
    Burmy,
    Wormadam(WormadamForme),
    Mothim,
    Combee,
    Vespiquen,
    Pachirisu,
    Buizel,
    Floatzel,
    Cherubi,
    Cherrim,
    Shellos,
    Gastrodon,
    Ambipom,
    Drifloon,
    Drifblim,
    Buneary,
    Lopunny(MegaEvolution),
    Mismagius,
    Honchkrow,
    Glameow,
    Purugly,
    Chingling,
    Stunky,
    Skuntank,
    Bronzor,
    Bronzong,
    Bonsly,
    #[strum(serialize="Mime Jr.", serialize="MimeJr")]
    MimeJr,
    Happiny,
    Chatot,
    Spiritomb,
    Gible,
    Gabite,
    Garchomp(MegaEvolution),
    Munchlax,
    Riolu,
    Lucario(MegaEvolution),
    Hippopotas,
    Hippowdon,
    Skorupi,
    Drapion,
    Croagunk,
    Toxicroak,
    Carnivine,
    Finneon,
    Lumineon,
    Mantyke,
    Snover,
    Abomasnow(MegaEvolution),
    Weavile,
    Magnezone,
    Lickilicky,
    Rhyperior,
    Tangrowth,
    Electivire,
    Magmortar,
    Togekiss,
    Yanmega,
    Leafeon,
    Glaceon,
    Gliscor,
    Mamoswine,
    #[strum(serialize="Porygon-Z", serialize="PorygonZ")]
    PorygonZ,
    Gallade(MegaEvolution),
    Probopass,
    Dusknoir,
    Froslass,
    Rotom(RotomForme),
    Uxie,
    Mesprit,
    Azelf,
    Dialga,
    Palkia,
    Heatran,
    Regigigas,
    Giratina(GiratinaForme),
    Cresselia,
    Phione,
    Manaphy,
    Darkrai,
    Shaymin(ShayminForme),
    Arceus(ArceusForme),
    // gen 5 starts here
    Victini,
    Snivy,
    Servine,
    Serperior,
    Tepig,
    Pignite,
    Emboar,
    Oshawott,
    Dewott,
    Samurott,
    Patrat,
    Watchog,
    Lillipup,
    Herdier,
    Stoutland,
    Purrloin,
    Liepard,
    Pansage,
    Simisage,
    Pansear,
    Simisear,
    Panpour,
    Simipour,
    Munna,
    Musharna,
    Pidove,
    Tranquill,
    Unfezant,
    Blitzle,
    Zebstrika,
    Roggenrola,
    Boldore,
    Gigalith,
    Woobat,
    Swoobat,
    Drilbur,
    Excadrill,
    Audino,
    Timburr,
    Gurdurr,
    Conkeldurr,
    Tympole,
    Palpitoad,
    Seismitoad,
    Throh,
    Sawk,
    Sewaddle,
    Swadloon,
    Leavanny,
    Venipede,
    Whirlipede,
    Scolipede,
    Cottonee,
    Whimsicott,
    Petilil,
    Lilligant,
    Basculin,
    Sandile,
    Krokorok,
    Krookodile,
    Darumaka,
    Darmanitan(DarmanitanForme),
    Maractus,
    Dwebble,
    Crustle,
    Scraggy,
    Scrafty,
    Sigilyph,
    Yamask,
    Cofagrigus,
    Tirtouga,
    Carracosta,
    Archen,
    Archeops,
    Trubbish,
    Garbodor,
    Zorua,
    Zoroark,
    Minccino,
    Cinccino,
    Gothita,
    Gothorita,
    Gothitelle,
    Solosis,
    Duosion,
    Reuniclus,
    Ducklett,
    Swanna,
    Vanillite,
    Vanillish,
    Vanilluxe,
    Deerling,
    Sawsbuck,
    Emolga,
    Karrablast,
    Escavalier,
    Foongus,
    Amoonguss,
    Frillish,
    Jellicent,
    Alomomola,
    Joltik,
    Galvantula,
    Ferroseed,
    Ferrothorn,
    Klink,
    Klang,
    Klinklang,
    Tynamo,
    Eelektrik,
    Eelektross,
    Elgyem,
    Beheeyem,
    Litwick,
    Lampent,
    Chandelure,
    Axew,
    Fraxure,
    Haxorus,
    Cubchoo,
    Beartic,
    Cryogonal,
    Shelmet,
    Accelgor,
    Stunfisk,
    Mienfoo,
    Mienshao,
    Druddigon,
    Golett,
    Golurk,
    Pawniard,
    Bisharp,
    Bouffalant,
    Rufflet,
    Braviary,
    Vullaby,
    Mandibuzz,
    Heatmor,
    Durant,
    Deino,
    Zweilous,
    Hydreigon,
    Larvesta,
    Volcarona,
    Cobalion,
    Terrakion,
    Virizion,
    Tornadus(GenieForme),
    Thundurus(GenieForme),
    Reshiram,
    Zekrom,
    Landorus(GenieForme),
    Kyurem(KyuremForme),
    Keldeo,
    Meloetta(KyuremForme),
    Genesect,
    // gen 6 starts here
    Chespin,
    Quilladin,
    Chesnaught,
    Fennekin,
    Braixen,
    Delphox,
    Froakie,
    Frogadier,
    Greninja(GreninjaForme),
    Bunnelby,
    Diggersby,
    Fletchling,
    Fletchinder,
    Talonflame,
    Scatterbug,
    Spewpa,
    Vivillon,
    Litleo,
    Pyroar,
    Flabebe,
    Floette,
    Florges,
    Skiddo,
    Gogoat,
    Pancham,
    Pangoro,
    Furfrou,
    Espurr,
    Meowstic,
    Honedge,
    Doublade,
    Aegislash(AegislashForme),
    Spritzee,
    Aromatisse,
    Swirlix,
    Slurpuff,
    Inkay,
    Malamar,
    Binacle,
    Barbaracle,
    Skrelp,
    Dragalge,
    Clauncher,
    Clawitzer,
    Helioptile,
    Heliolisk,
    Tyrunt,
    Tyrantrum,
    Amaura,
    Aurorus,
    Sylveon,
    Hawlucha,
    Dedenne,
    Carbink,
    Goomy,
    Sliggoo,
    Goodra,
    Klefki,
    Phantump,
    Trevenant,
    Pumpkaboo(GourgeistForme),
    Gourgeist(GourgeistForme),
    Bergmite,
    Avalugg,
    Noibat,
    Noivern,
    Xerneas,
    Yveltal,
    Zygarde(ZygardeForme),
    Diancie(MegaEvolution),
    Hoopa(HoopaForme),
    Volcanion,
    // gen 7 starts here
    Rowlet,
    Dartrix,
    Decidueye,
    Litten,
    Torracat,
    Incineroar,
    Popplio,
    Brionne,
    Primarina,
    Pikipek,
    Trumbeak,
    Toucannon,
    Yungoos,
    Gumshoos,
    Grubbin,
    Charjabug,
    Vikavolt,
    Crabrawler,
    Crabominable,
    Oricorio(OricorioForme),
    Cutiefly,
    Ribombee,
    Rockruff,
    Lycanroc(LycanrocForme),
    Wishiwashi(WishiwashiForme),
    Mareanie,
    Toxapex,
    Mudbray,
    Mudsdale,
    Dewpider,
    Araquanid,
    Fomantis,
    Lurantis,
    Morelull,
    Shiinotic,
    Salandit,
    Salazzle,
    Stufful,
    Bewear,
    Bounsweet,
    Steenee,
    Tsareena,
    Comfey,
    Oranguru,
    Passimian,
    Wimpod,
    Golisopod,
    Sandygast,
    Palossand,
    Pyukumuku,
    #[strum(serialize="Type: Null", serialize="TypeNull")]
    TypeNull(SilvallyForme),
    Silvally(SilvallyForme),
    Minior(MiniorForme),
    Komala,
    Turtonator,
    Togedemaru,
    Mimikyu(MimikyuForme),
    Bruxish,
    Drampa,
    Dhelmise,
    #[strum(serialize="Jangmo-O", serialize="JangmoO")]
    JangmoO,
    #[strum(serialize="Hakamo-O", serialize="HakamoO")]
    HakamoO,
    #[strum(serialize="Kommo-O", serialize="KommoO")]
    KommoO,
    #[strum(serialize="Tapu Koko", serialize="TapuKoko")]
    TapuKoko,
    #[strum(serialize="Tapu Lele", serialize="TapuLele")]
    TapuLele,
    #[strum(serialize="Tapu Bulu", serialize="TapuBulu")]
    TapuBulu,
    #[strum(serialize="Tapu Fini", serialize="TapuFini")]
    TapuFini,
    Cosmog,
    Cosmoem,
    Solgaleo,
    Lunala,
    Nihilego,
    Buzzwole,
    Pheromosa,
    Xurkitree,
    Celesteela,
    Kartana,
    Guzzlord,
    Necrozma(NecrozmaForme),
    Magearna,
    Marshadow,
    Poipole,
    Naganadel,
    Stakataka,
    Blacephalon,
    Zeraora,
}

impl Species {
    /// Returns true if the given species has a forme, and false otherwise. Formes are Pokemon with
    /// different characterisics but the same species, like Deoxys-Attack and Deoxys-Defense.
    pub fn has_forme(self) -> bool {
        // This is bad design: it duplicates data already listed above. If there's a better way to do
        // this, I'd really like to know, but Rust doesn't allow generic unwrapping of enums or
        // generic match statements for what I want to do.
        match self {
            // in National Pokedex order
            Species::Venusaur(_) => true,
            Species::Charizard(_) => true,
            Species::Blastoise(_) => true,
            Species::Beedrill(_) => true,
            Species::Pidgeot(_) => true,
            Species::Raichu(_) => true,
            Species::Sandshrew(_) => true,
            Species::Sandslash(_) => true,
            Species::Vulpix(_) => true,
            Species::Ninetales(_) => true,
            Species::Diglett(_) => true,
            Species::Dugtrio(_) => true,
            Species::Persian(_) => true,
            Species::Rattata(_) => true,
            Species::Raticate(_) => true,
            Species::Alakazam(_) => true,
            Species::Geodude(_) => true,
            Species::Graveler(_) => true,
            Species::Golem(_) => true,
            Species::Grimer(_) => true,
            Species::Muk(_) => true,
            Species::Gengar(_) => true,
            Species::Exeggutor(_) => true,
            Species::Marowak(_) => true,
            Species::Kangaskhan(_) => true,
            Species::Pinsir(_) => true,
            Species::Gyarados(_) => true,
            Species::Aerodactyl(_) => true,
            Species::Mewtwo(_) => true,
            Species::Steelix(_) => true,
            Species::Scizor(_) => true,
            Species::Heracross(_) => true,
            Species::Houndoom(_) => true,
            Species::Tyranitar(_) => true,
            Species::Sceptile(_) => true,
            Species::Blaziken(_) => true,
            Species::Swampert(_) => true,
            Species::Gardevoir(_) => true,
            Species::Sableye(_) => true,
            Species::Mawile(_) => true,
            Species::Aggron(_) => true,
            Species::Medicham(_) => true,
            Species::Manectric(_) => true,
            Species::Sharpedo(_) => true,
            Species::Camerupt(_) => true,
            Species::Altaria(_) => true,
            Species::Castform(_) => true,
            Species::Salamence(_) => true,
            Species::Metagross(_) => true,
            Species::Kyogre(_) => true,
            Species::Groudon(_) => true,
            Species::Rayquaza(_) => true,
            Species::Deoxys(_) => true,
            Species::Wormadam(_) => true,
            Species::Lopunny(_) => true,
            Species::Garchomp(_) => true,
            Species::Lucario(_) => true,
            Species::Abomasnow(_) => true,
            Species::Gallade(_) => true,
            Species::Rotom(_) => true,
            Species::Giratina(_) => true,
            Species::Shaymin(_) => true,
            Species::Arceus(_) => true,
            Species::Darmanitan(_) => true,
            Species::Tornadus(_) => true,
            Species::Thundurus(_) => true,
            Species::Landorus(_) => true,
            Species::Kyurem(_) => true,
            Species::Meloetta(_) => true,
            Species::Greninja(_) => true,
            Species::Aegislash(_) => true,
            Species::Pumpkaboo(_) => true,
            Species::Gourgeist(_) => true,
            Species::Zygarde(_) => true,
            Species::Diancie(_) => true,
            Species::Hoopa(_) => true,
            Species::Oricorio(_) => true,
            Species::Lycanroc(_) => true,
            Species::Wishiwashi(_) => true,
            Species::TypeNull(_) => true,
            Species::Silvally(_) => true,
            Species::Minior(_) => true,
            Species::Mimikyu(_) => true,
            Species::Necrozma(_) => true,
            _ => false            
        }
    }


    // TODO: implement generic "get string of underlying forme" using Box?
}

// impl fmt::Display for Species {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let species: Species = self.into();        
//     }
// }

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use std::str::FromStr;
    use strum::IntoEnumIterator;

    #[test]
    fn test_ids() {
        // each is 1 less than the National Pokedex number, because this starts at 0
        // I can't change it because it's autogenerated code
        assert_eq!(SpeciesDiscriminant::Poipole as u32, 802);
        assert_eq!(SpeciesDiscriminant::Zeraora as u32, 806);
        assert_eq!(SpeciesDiscriminant::Castform as u32, 350);
    }

    #[test]
    fn test_from_names() {
        assert_eq!(SpeciesDiscriminant::from_str("Bulbasaur").unwrap(), SpeciesDiscriminant::Bulbasaur);
        
        assert_eq!(SpeciesDiscriminant::from_str("Ho-Oh").unwrap(), SpeciesDiscriminant::HoOh);
        assert_eq!(SpeciesDiscriminant::from_str("HoOh").unwrap(), SpeciesDiscriminant::HoOh);
        
        assert_eq!(SpeciesDiscriminant::from_str("Farfetch'd").unwrap(), SpeciesDiscriminant::Farfetchd);
        assert_eq!(SpeciesDiscriminant::from_str("Farfetchd").unwrap(), SpeciesDiscriminant::Farfetchd);

        assert_eq!(SpeciesDiscriminant::from_str("Porygon-Z").unwrap(), SpeciesDiscriminant::PorygonZ);
        assert_eq!(SpeciesDiscriminant::from_str("PorygonZ").unwrap(), SpeciesDiscriminant::PorygonZ);
        
        assert_eq!(SpeciesDiscriminant::from_str("Nidoran♀").unwrap(), SpeciesDiscriminant::NidoranF);
        assert_eq!(SpeciesDiscriminant::from_str("NidoranF").unwrap(), SpeciesDiscriminant::NidoranF);

        assert_eq!(SpeciesDiscriminant::from_str("Nidoran♂").unwrap(), SpeciesDiscriminant::NidoranM);
        assert_eq!(SpeciesDiscriminant::from_str("NidoranM").unwrap(), SpeciesDiscriminant::NidoranM);

        assert_eq!(SpeciesDiscriminant::from_str("Jangmo-O").unwrap(), SpeciesDiscriminant::JangmoO);
        assert_eq!(SpeciesDiscriminant::from_str("JangmoO").unwrap(), SpeciesDiscriminant::JangmoO);

        assert_eq!(SpeciesDiscriminant::from_str("Hakamo-O").unwrap(), SpeciesDiscriminant::HakamoO);
        assert_eq!(SpeciesDiscriminant::from_str("HakamoO").unwrap(), SpeciesDiscriminant::HakamoO);

        assert_eq!(SpeciesDiscriminant::from_str("Kommo-O").unwrap(), SpeciesDiscriminant::KommoO);
        assert_eq!(SpeciesDiscriminant::from_str("KommoO").unwrap(), SpeciesDiscriminant::KommoO);

        assert_eq!(SpeciesDiscriminant::from_str("Tapu Koko").unwrap(), SpeciesDiscriminant::TapuKoko);
        assert_eq!(SpeciesDiscriminant::from_str("TapuKoko").unwrap(), SpeciesDiscriminant::TapuKoko);
        
        assert_eq!(SpeciesDiscriminant::from_str("Tapu Bulu").unwrap(), SpeciesDiscriminant::TapuBulu);
        assert_eq!(SpeciesDiscriminant::from_str("TapuBulu").unwrap(), SpeciesDiscriminant::TapuBulu);
        
        assert_eq!(SpeciesDiscriminant::from_str("Tapu Lele").unwrap(), SpeciesDiscriminant::TapuLele);
        assert_eq!(SpeciesDiscriminant::from_str("TapuLele").unwrap(), SpeciesDiscriminant::TapuLele);
        
        assert_eq!(SpeciesDiscriminant::from_str("Tapu Fini").unwrap(), SpeciesDiscriminant::TapuFini);
        assert_eq!(SpeciesDiscriminant::from_str("TapuFini").unwrap(), SpeciesDiscriminant::TapuFini);
    }

    #[test]
    fn test_to_names_and_enumiter() {
        for species in SpeciesDiscriminant::iter() {
            assert_eq!(SpeciesDiscriminant::from_str(&species.to_string()).unwrap(), species);
        }

        assert_eq!(&SpeciesDiscriminant::HoOh.to_string(), "Ho-Oh");
        assert_eq!(&SpeciesDiscriminant::Farfetchd.to_string(), "Farfetch'd");
        assert_eq!(&SpeciesDiscriminant::PorygonZ.to_string(), "Porygon-Z");
        assert_eq!(&SpeciesDiscriminant::NidoranF.to_string(), "Nidoran♀");
        assert_eq!(&SpeciesDiscriminant::NidoranM.to_string(), "Nidoran♂");
        assert_eq!(&SpeciesDiscriminant::JangmoO.to_string(), "Jangmo-O");
        assert_eq!(&SpeciesDiscriminant::HakamoO.to_string(), "Hakamo-O");
        assert_eq!(&SpeciesDiscriminant::KommoO.to_string(), "Kommo-O");
        assert_eq!(&SpeciesDiscriminant::TapuKoko.to_string(), "Tapu Koko");
        assert_eq!(&SpeciesDiscriminant::TapuBulu.to_string(), "Tapu Bulu");
        assert_eq!(&SpeciesDiscriminant::TapuLele.to_string(), "Tapu Lele");
        assert_eq!(&SpeciesDiscriminant::TapuFini.to_string(), "Tapu Fini");
        assert_eq!(&SpeciesDiscriminant::TypeNull.to_string(), "Type: Null");
    }
}
