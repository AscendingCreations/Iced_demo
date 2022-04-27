use num_enum::TryFromPrimitive;
use serde_repr::*;
use strum_macros::Display;

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Display,
    Derivative,
)]
#[repr(u8)]
#[derivative(Default)]
pub enum SkillStat {
    #[derivative(Default)]
    Gathering,
    Mining,
    Fishing,
    Woodcutting,
    Forging,
    Cooking,
    Crafting,
    Alchemy,
    Melee,
    Archery,
    Summoning,
    Taming,
    #[strum(serialize = "None")]
    Count,
} //12

#[derive(
    Copy, Clone, Debug, Serialize_repr, Deserialize_repr, Display, TryFromPrimitive, Eq, PartialEq,
)]
#[repr(u8)]
pub enum CombatStat {
    // WEAPON SKILLS
    Slash,
    Blunt,
    Pierce,
    // Defense Skills
    Slashdef,
    Bluntdef,
    Piercedef,
    // Magic Skills
    Fire,
    Water,
    Wind,
    Electric,
    Earth,
    Light,
    Dark,
    // Resistance
    Fireres,
    Waterres,
    Windres,
    Electricres,
    Earthres,
    Lightres,
    Darkres,
    Statusres,
    #[strum(serialize = "None")]
    Count,
} //21

#[allow(dead_code)]
impl CombatStat {
    pub fn get_defense(&self) -> usize {
        match self {
            CombatStat::Slash => CombatStat::Slashdef as usize,
            CombatStat::Blunt => CombatStat::Bluntdef as usize,
            CombatStat::Pierce => CombatStat::Piercedef as usize,
            CombatStat::Fire => CombatStat::Fireres as usize,
            CombatStat::Water => CombatStat::Waterres as usize,
            CombatStat::Wind => CombatStat::Windres as usize,
            CombatStat::Electric => CombatStat::Electricres as usize,
            CombatStat::Dark => CombatStat::Darkres as usize,
            _ => CombatStat::Bluntdef as usize,
        }
    }

    pub fn is_defense(&self) -> bool {
        matches!(
            self,
            CombatStat::Slashdef
                | CombatStat::Bluntdef
                | CombatStat::Piercedef
                | CombatStat::Fireres
                | CombatStat::Waterres
                | CombatStat::Windres
                | CombatStat::Electricres
                | CombatStat::Earthres
                | CombatStat::Lightres
                | CombatStat::Darkres
                | CombatStat::Statusres
        )
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Derivative,
    Display,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum SkillRange {
    #[derivative(Default)]
    None,
    Target,
    Linear,
    Spray,
    Area,
    Targetarea,
    Position,
    DoubleLinear,
    Strike,
    Torrent,
    GrandTorrent,
    Wave,
    RandomChaos,
    Snake,
    Spread,
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Derivative,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum UserAccess {
    #[derivative(Default)]
    None,
    Monitor,
    Admin,
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Derivative,
    Display,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum CastActionBlock {
    #[derivative(Default)]
    None,
    NoCancelDuring,
    NoCancelAfter,
    NoCancelBoth,
}

#[allow(dead_code)]
impl CastActionBlock {
    pub fn is_blocked_after(&self) -> bool {
        matches!(
            self,
            CastActionBlock::NoCancelAfter | CastActionBlock::NoCancelBoth
        )
    }

    pub fn is_blocked_during(&self) -> bool {
        matches!(
            self,
            CastActionBlock::NoCancelDuring | CastActionBlock::NoCancelBoth
        )
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Derivative,
    Display,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum AIBehavior {
    #[derivative(Default)]
    Friendly, //Never Attack or be attacked
    Agressive,       //Will attack on sight
    Reactive,        //Will attack when attacked
    HelpReactive,    //for npcs that when one gets attacked all in the area target the attacker.
    Healer,          //Will never Attack only heal other npcs
    AgressiveHealer, //Will attack on sight and heal
    ReactiveHealer,  //Will attack when attacked and heal
}

#[allow(dead_code)]
impl AIBehavior {
    pub fn is_agressive(&self) -> bool {
        matches!(self, AIBehavior::Agressive | AIBehavior::AgressiveHealer)
    }

    pub fn is_reactive(&self) -> bool {
        matches!(
            self,
            AIBehavior::Reactive | AIBehavior::HelpReactive | AIBehavior::ReactiveHealer
        )
    }

    pub fn is_healer(&self) -> bool {
        matches!(
            self,
            AIBehavior::Healer | AIBehavior::AgressiveHealer | AIBehavior::ReactiveHealer
        )
    }

    pub fn is_friendly(&self) -> bool {
        matches!(self, AIBehavior::Friendly)
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Derivative,
    Display,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum NpcCastType {
    #[derivative(Default)]
    SelfOnly,
    Enemy,  // for Attack spells/bad effects
    Friend, // for healing/revival/good effects
    Ground, // no target just Attack at position
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Derivative,
    Display,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum ItemTypes {
    #[derivative(Default)]
    None,
    Weapon,
    Accessory,
    Cosmetic,
    Helmet,
    Armor,
    Trouser,
    Boots,
    Consume,
    Tool,
    Blueprint,
    Book,
    Questitem,
    Trap,
    Heavyobject,
    ScriptMain,
    ScriptRange,
    ScriptRepeater,
    ScriptEffect,
    ScriptPower,
    ScriptCast,
    ScriptData,
    Key,
    Count,
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Derivative,
    Display,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum Gods {
    #[derivative(Default)]
    None,
    Genusis,
    Ais,
    Eriz,
    Sonya,
    Purson,
    Count,
} //6

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Derivative,
    Display,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum VitalTypes {
    Hp,
    Mp,
    Sp,
    #[derivative(Default)]
    #[strum(serialize = "None")]
    Count,
}

#[derive(
    Copy, Clone, Serialize_repr, Deserialize_repr, Display, TryFromPrimitive, Eq, PartialEq,
)]
#[repr(u8)]
pub enum MapAttributes {
    None,
    Blocked,
    DirBlocked,
    NpcBlocked,
    PlayerBlocked,
    Bank,
    Shop,
    Door,
    Craft,
    Slide,
    Warp,
    Item,
    Portal,
    CheckPoint,
    Sign,
    Resource,
    Count,
}

#[derive(
    Copy,
    Clone,
    Serialize_repr,
    Deserialize_repr,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Derivative,
    Display,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum Weather {
    #[derivative(Default)]
    None,
    Rain,
    Snow,
    Sunny,
    Storm,
    Blizzard,
    Heat,
    Hail,
    SandStorm,
    Windy,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Display)]
pub enum MapLayers {
    Ground,
    Mask,
    Mask2,
    Anim1,
    Anim2,
    Anim3,
    Fringe,
    Fringe2,
    Count,
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Derivative,
    Display,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum ToolType {
    #[derivative(Default)]
    None,
    Axe,
    Pick,
    Rod,
    Hoe,
    Scythe,
    Shovel,
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Derivative,
    Display,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum StatType {
    #[derivative(Default)]
    None,
    Combat,
    Skill,
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Derivative,
    Display,
    Serialize_repr,
    Deserialize_repr,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum NpcMode {
    None,
    #[derivative(Default)]
    Normal,
    Pet,
    Summon,
    Boss,
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Derivative,
    Display,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum CastedType {
    #[derivative(Default)]
    Normal,
    MagicAttack,
    MagicHeal,
    MagicEffect,
    Trap,
    Summon,
    Warp,
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Derivative,
    Display,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum SkillFollowType {
    #[derivative(Default)]
    None,
    Caster,
    Target,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MapPos {
    None,
    UpLeft(u64),
    Up(u64),
    UpRight(u64),
    Left(u64),
    Center(u64),
    Right(u64),
    DownLeft(u64),
    Down(u64),
    DownRight(u64),
}

#[allow(dead_code)]
impl MapPos {
    pub fn contains(self, id: u64) -> bool {
        match self {
            MapPos::UpLeft(x)
            | MapPos::Up(x)
            | MapPos::UpRight(x)
            | MapPos::Left(x)
            | MapPos::Center(x)
            | MapPos::Right(x)
            | MapPos::DownLeft(x)
            | MapPos::Down(x)
            | MapPos::DownRight(x)
                if x == id =>
            {
                true
            }
            _ => false,
        }
    }

    pub fn get(self) -> Option<u64> {
        match self {
            MapPos::UpLeft(x)
            | MapPos::Up(x)
            | MapPos::UpRight(x)
            | MapPos::Left(x)
            | MapPos::Center(x)
            | MapPos::Right(x)
            | MapPos::DownLeft(x)
            | MapPos::Down(x)
            | MapPos::DownRight(x) => Some(x),
            MapPos::None => None,
        }
    }

    pub fn unwrap(self) -> u64 {
        match self {
            MapPos::UpLeft(x)
            | MapPos::Up(x)
            | MapPos::UpRight(x)
            | MapPos::Left(x)
            | MapPos::Center(x)
            | MapPos::Right(x)
            | MapPos::DownLeft(x)
            | MapPos::Down(x)
            | MapPos::DownRight(x) => x,
            MapPos::None => panic!("MapPos Can not be None for unwrap"),
        }
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Display,
    Derivative,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum CraftMethod {
    #[derivative(Default)]
    None, //allows you to craft anytime via a craft window
    Anvil, //must always be used here.
    Forge,
    AlchemyTable,
    Fire,
    Stove,
    SewingStation,
    Tanner,
    #[strum(serialize = "None")]
    Count,
}
