use crate::item::*;
use araiseal_styles::{self, CheckBoxStyle, TEXT_WHITE};
use araiseal_types::*;
use araiseal_ui::*;

use iced::{
    alignment::{Alignment, Horizontal, Vertical},
    button,
    pick_list::{self, PickList},
    text_input, Button, Checkbox, Color, Column, Container, Element, Image, Length, Row, Rule,
    Text, TextInput,
};

use iced_aw::color_picker::{self, ColorPicker};

#[allow(dead_code)]
#[derive(Derivative)]
#[derivative(Default())]
pub struct ItemUiGeneric {
    #[derivative(Default(value = "Vec::with_capacity(ItemTypes::Count as usize)"))]
    pub type_list: Vec<ItemTypes>,
    pub type_selected: Option<ItemTypes>,
    pub type_state: pick_list::State<ItemTypes>,
    pub txt_name: text_input::State,
    pub txt_value: String,
    #[derivative(Default(value = "NumInput::new(0)"))]
    pub skill_input: NumInput<u16, Message>,
    #[derivative(Default(value = "NumInput::new(0)"))]
    pub level_input: NumInput<u16, Message>,
    #[derivative(Default(value = "NumInput::new(0)"))]
    pub sound_input: NumInput<u16, Message>,
    #[derivative(Default(value = "NumInput::new(0)"))]
    pub sprite_input: NumInput<u16, Message>,
    #[derivative(Default(value = "NumInput::new(0)"))]
    pub type2: NumInput<u16, Message>,
    #[derivative(Default(value = "NumInput::new(0)"))]
    pub animation_input: NumInput<i64, Message>,
    pub breakable: bool,
    pub stackable: bool,
    pub repairable: bool,
    #[derivative(Default(value = "NumInput::new(0)"))]
    pub stack_limit_input: NumInput<u16, Message>,
    #[derivative(Default(value = "NumInput::new(0)"))]
    pub base_price_input: NumInput<u64, Message>,
    #[derivative(Default(value = "NumInput::new(0)"))]
    pub req_input: NumInput<u16, Message>,
    pub reqtype_list: Vec<StatType>,
    pub reqtype_selected: Option<StatType>,
    pub reqtype_state: pick_list::State<StatType>,
    pub req_list: Vec<ListData>,
    pub req_selected: Option<ListData>,
    pub req_state: pick_list::State<ListData>,
    pub color: Color,
    pub color_state: color_picker::State,
    pub color_btn_state: button::State,
}

impl ItemUiGeneric {
    pub fn layout(&mut self, _item_type: ItemTypes) -> Element<Message> {
        let column = Column::new()
            .spacing(6)
            .align_items(Alignment::Center)
            .push(
                Text::new("Generic")
                    .width(Length::Fill)
                    .vertical_alignment(Vertical::Bottom)
                    .horizontal_alignment(Horizontal::Center)
                    .color(TEXT_WHITE),
            )
            .push(Rule::horizontal(0));

        let row = Row::new()
            .spacing(12)
            .align_items(Alignment::Center)
            .push(
                Column::new().push(
                    TextInput::new(
                        &mut self.txt_name,
                        "Name",
                        &self.txt_value[..],
                        Message::NameInput,
                    )
                    .width(Length::Units(256))
                    .style(araiseal_styles::CustomTextInput)
                    .padding(3),
                ),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(
                        Text::new("Item Type")
                            .horizontal_alignment(Horizontal::Center)
                            .color(TEXT_WHITE),
                    )
                    .push(
                        PickList::new(
                            &mut self.type_state,
                            &self.type_list[..],
                            self.type_selected,
                            Message::TypeSelect,
                        )
                        .style(araiseal_styles::CustomPickList),
                    ),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(
                        Text::new("Item Type 2")
                            .horizontal_alignment(Horizontal::Center)
                            .color(TEXT_WHITE),
                    )
                    .push(self.type2.view(5, 0, 100, 1, Message::GenericInput)),
            );

        let sprite_value = self.sprite_input.value;

        let row1 = Row::new().spacing(6).push(
            Column::new()
                .spacing(5)
                .push(Text::new("Item Sprite".to_string()).color(TEXT_WHITE))
                .push(
                    Row::new()
                        .align_items(Alignment::Center)
                        .spacing(6)
                        .push(self.sprite_input.view(4, 0, 1000, 1, Message::GenericInput))
                        .push(
                            Container::new(
                                Image::new(format!("./resources/items/{}.png", sprite_value))
                                    .width(Length::Units(32))
                                    .height(Length::Units(32)),
                            )
                            .center_x()
                            .center_y()
                            //.style(styles::ImageContainer)
                            .width(Length::Units(32))
                            .height(Length::Units(32)),
                        ),
                ),
        );

        let row2 = Row::new()
            .spacing(6)
            .push(
                Column::new()
                    .spacing(5)
                    .push(Text::new("Skill ID".to_string()).color(TEXT_WHITE))
                    .push(self.skill_input.view(0, 0, 100, 1, Message::GenericInput)),
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(Text::new("Level Req".to_string()).color(TEXT_WHITE))
                    .push(self.level_input.view(1, 0, 200, 1, Message::GenericInput)),
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(Text::new("Sound ID".to_string()).color(TEXT_WHITE))
                    .push(self.sound_input.view(2, 0, 100, 1, Message::GenericInput)),
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(Text::new("Stack Limit".to_string()).color(TEXT_WHITE))
                    .push(
                        self.stack_limit_input
                            .view(3, 1, 1000, 1, Message::GenericInput),
                    ),
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(Text::new("Base Store Price".to_string()).color(TEXT_WHITE))
                    .push(
                        self.base_price_input
                            .view(0, 0, 999999999, 1, Message::BasePriceInput),
                    ),
            );

        let row3 = Row::new()
            .spacing(6)
            .push(
                Column::new()
                    .spacing(5)
                    .push(Text::new("Req Stat Type".to_string()).color(TEXT_WHITE))
                    .push(
                        PickList::new(
                            &mut self.reqtype_state,
                            &self.reqtype_list[..],
                            self.reqtype_selected,
                            Message::ReqTypeSelect,
                        )
                        .style(araiseal_styles::CustomPickList),
                    ),
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(Text::new("Req Stat".to_string()).color(TEXT_WHITE))
                    .push(
                        PickList::new(
                            &mut self.req_state,
                            &self.req_list[..],
                            self.req_selected.clone(),
                            Message::ReqSelect,
                        )
                        .style(araiseal_styles::CustomPickList),
                    ),
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(Text::new("Req Stat Level".to_string()).color(TEXT_WHITE))
                    .push(self.req_input.view(6, 0, 101, 1, Message::GenericInput)),
            );

        let row4 = Row::new()
            .spacing(6)
            .push(
                Checkbox::new(
                    self.breakable,
                    String::from("Breakable"),
                    Message::Breakable,
                )
                .style(CheckBoxStyle),
            )
            .push(
                Checkbox::new(
                    self.repairable,
                    String::from("Repairable"),
                    Message::Repairable,
                )
                .style(CheckBoxStyle),
            )
            .push(
                Checkbox::new(
                    self.stackable,
                    String::from("Stackable"),
                    Message::Stackable,
                )
                .style(CheckBoxStyle),
            );

        let colorpicker = ColorPicker::new(
            &mut self.color_state,
            Button::new(
                &mut self.color_btn_state,
                Text::new("Set Color").color(TEXT_WHITE),
            )
            .style(araiseal_styles::Button::Secondary)
            .on_press(Message::ChooseColor),
            Message::CancelColor,
            Message::SubmitColor,
        )
        .style(araiseal_styles::CustomColorPicker);

        let style = araiseal_styles::ColorContainer { color: self.color };

        let row5 = Row::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .push(colorpicker)
            .push(
                Text::new(format!(
                    "R: {:.0} G: {:.0} B: {:.0} A: {:.0}",
                    self.color.r * 255.0,
                    self.color.g * 255.0,
                    self.color.b * 255.0,
                    self.color.a * 255.0
                ))
                .color(TEXT_WHITE),
            )
            .push(
                Container::new(Row::new())
                    .height(Length::Units(32))
                    .width(Length::Units(32))
                    .style(style),
            );

        column
            .push(row)
            .push(row1)
            .push(row2)
            .push(row3)
            .push(row4)
            .push(row5)
            .into()
    }
}
