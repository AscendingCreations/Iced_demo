use crate::item::*;
use araiseal_styles::TEXT_WHITE;
use araiseal_types::*;
use araiseal_ui::*;
use arr_macro::arr;
use std::convert::TryFrom;

use iced::{
    alignment::{Alignment, Horizontal, Vertical},
    Column, Element, Length, Row, Rule, Text,
};

#[derive(Derivative)]
#[derivative(Default())]
pub struct ItemUiSkillBufs {
    #[derivative(Default(value = "arr![NumInput::new(0); 12]"))]
    pub input: [NumInput<i16, Message>; SKILL_MAX],
}

impl ItemUiSkillBufs {
    pub fn layout(&mut self) -> Element<Message> {
        let mut i = 0;
        let mut column = Column::new()
            .spacing(6)
            .align_items(Alignment::Center)
            .push(
                Text::new("Skill Buffers")
                    .width(Length::Fill)
                    .vertical_alignment(Vertical::Bottom)
                    .horizontal_alignment(Horizontal::Center)
                    .color(TEXT_WHITE),
            )
            .push(Rule::horizontal(0));
        let mut row = Row::new().spacing(12).align_items(Alignment::Center);

        for (id, control) in self.input.iter_mut().enumerate() {
            if i == 6 {
                i = 0;
                column = column.push(row);
                row = Row::new().spacing(12).align_items(Alignment::Center);
            }
            let txt = SkillStat::try_from(id as u8).unwrap_or(SkillStat::Count);

            row = row.push(
                Column::new()
                    .spacing(5)
                    .push(Text::new(txt.to_string()).color(TEXT_WHITE))
                    .push(control.view(id, 0, 100, 1, Message::SkillInput)),
            );
            i += 1;
        }

        column.push(row).into()
    }
}
