mod date;
mod duration;
mod effecttype;
mod enums;
mod instant;
mod position;
mod rgb;
mod sharedstructs;

#[macro_use]
extern crate derivative;

pub use date::MyDate;
pub use duration::MyDuration;
pub use effecttype::*;
pub use enums::*;
pub use instant::MyInstant;
pub use position::*;
pub use rgb::*;
pub use sharedstructs::*;

pub const SKILL_MAX: usize = SkillStat::Count as usize;
pub const COMBAT_MAX: usize = CombatStat::Count as usize;
pub const VITALS_MAX: usize = VitalTypes::Count as usize;

pub type Combatstats = [i16; COMBAT_MAX];
pub type BuffCombatstats = [i16; COMBAT_MAX];
pub type Skillstats = [i16; SKILL_MAX];

///Map Data Maxs
pub const MAX_MAPS: usize = 3000;
pub const MAP_MAX_X: usize = 32;
pub const MAP_MAX_Y: usize = 32;
pub const MAX_SKILL_INV: usize = 11;
///Array Data Maxs
pub const MAX_NPCS: usize = 1000;
pub const MAX_ITEMS: usize = 2000;
pub const MAX_QUESTS: usize = 500;
pub const MAX_SKILLS: usize = 500;
pub const MAX_RESOURCES: usize = 1000;
pub const MAX_CRAFTS: usize = 1000;
