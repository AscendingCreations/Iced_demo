use crate::item::*;
use araiseal_styles::TEXT_WHITE;
use araiseal_types::*;
use araiseal_ui::*;
use arr_macro::arr;

use iced::{
    alignment::{Alignment, Horizontal, Vertical},
    Column, Element, Length, Row, Rule, Text,
};

#[derive(Derivative)]
#[derivative(Default())]
pub struct ItemUiData {
    #[derivative(Default(value = "arr![NumInput::new(0); 20]"))]
    pub input: [NumInput<i16, Message>; 20],
}

impl ItemUiData {
    pub fn layout(&mut self, item_type: ItemTypes) -> Element<Message> {
        let mut i = 0;
        let mut column = Column::new()
            .spacing(6)
            .align_items(Alignment::Center)
            .push(
                Text::new("Data Inputs")
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

            row = row.push(
                Column::new()
                    .spacing(5)
                    .push(Text::new(data_labels(id, item_type)).color(TEXT_WHITE))
                    .push(control.view(id, i16::MIN, i16::MAX, 1, Message::DataInput)),
            );
            i += 1;
        }

        column.push(row).into()
    }
}
