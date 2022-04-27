use araiseal_types::*;
use araiseal_ui::*;
use iced::Color;

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    SaveButtonPress,
    SaveAllButtonPress,
    RevertButtonPress,
    ListSelect(ListData),
    ReqTypeSelect(StatType),
    ReqSelect(ListData),
    TypeSelect(ItemTypes),
    CombatInput((usize, NumInputMessage<i16>)),
    SkillInput((usize, NumInputMessage<i16>)),
    DataInput((usize, NumInputMessage<i16>)),
    GenericInput((usize, NumInputMessage<u16>)),
    AnimationInput((usize, NumInputMessage<i64>)),
    BasePriceInput((usize, NumInputMessage<u64>)),
    Repairable(bool),
    Stackable(bool),
    Breakable(bool),
    NameInput(String),
    ChooseColor,
    SubmitColor(Color),
    CancelColor,
}
