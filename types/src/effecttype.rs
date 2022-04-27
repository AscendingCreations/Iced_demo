use num_enum::TryFromPrimitive;
use serde_repr::*;
use strum_macros::Display;

#[derive(
    Copy,
    Clone,
    Eq,
    Debug,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Derivative,
    Display,
    TryFromPrimitive,
)]
#[derivative(Default)]
#[repr(u8)]
pub enum EffectType {
    #[derivative(Default)]
    None,
    Freeze,
    Poison,
    Burn,
    Confuse,
    Stun,
    StatBoost,
    StatNegate,
    SkillBoost,
    SkillNegate,
    Regen,
    Decay,
    Revive,
    Transform,
    Invisible,
    Barrier,
    Counter,
    Absorb,
    Weaken,
    Affectionate,
    ExpBoost,
    ExpVacuum,
    Dispelled,
    Curse,
    StatusPrevent,
    MultiCast,
    Detect,
    Sluggish,
    Quicken,
}

#[allow(dead_code)]
impl EffectType {
    pub fn is_selfcast(&self) -> bool {
        matches!(
            self,
            EffectType::StatBoost
                | EffectType::SkillBoost
                | EffectType::Regen
                | EffectType::Transform
                | EffectType::Invisible
                | EffectType::Barrier
                | EffectType::Counter
                | EffectType::Absorb
                | EffectType::ExpBoost
                | EffectType::Dispelled
                | EffectType::StatusPrevent
                | EffectType::MultiCast
                | EffectType::Quicken
        )
    }

    pub fn is_enemycast(&self) -> bool {
        matches!(
            self,
            EffectType::Freeze
                | EffectType::Poison
                | EffectType::Burn
                | EffectType::Confuse
                | EffectType::Stun
                | EffectType::StatNegate
                | EffectType::SkillNegate
                | EffectType::Decay
                | EffectType::Weaken
                | EffectType::Affectionate
                | EffectType::ExpVacuum
                | EffectType::Dispelled
                | EffectType::Curse
                | EffectType::Detect
                | EffectType::Sluggish
        )
    }

    pub fn is_allycast(&self) -> bool {
        matches!(
            self,
            EffectType::StatBoost
                | EffectType::SkillBoost
                | EffectType::Regen
                | EffectType::Revive
                | EffectType::Transform
                | EffectType::Invisible
                | EffectType::Barrier
                | EffectType::Counter
                | EffectType::Absorb
                | EffectType::Dispelled
                | EffectType::StatusPrevent
                | EffectType::Quicken
        )
    }
}
