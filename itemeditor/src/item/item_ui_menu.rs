use crate::item::*;
use araiseal_types::*;
use araiseal_ui::*;

use iced::{
    button,
    pick_list::{self, PickList},
    Element, Length, Row,
};

#[derive(Derivative)]
#[derivative(Default())]
pub struct ItemUiMenu {
    #[derivative(Default(value = "Vec::with_capacity(MAX_ITEMS)"))]
    pub list: Vec<ListData>,
    pub list_selected: Option<ListData>,
    pub data_list: pick_list::State<ListData>,
    pub save_all_button: button::State,
    pub save_button: button::State,
    pub revert_button: button::State,
}

impl ItemUiMenu {
    pub fn layout(&mut self) -> Element<Message> {
        Row::new()
            .width(Length::Fill)
            .spacing(5)
            .push(
                PickList::new(
                    &mut self.data_list,
                    &self.list[..],
                    self.list_selected.clone(),
                    Message::ListSelect,
                )
                .style(araiseal_styles::CustomPickList)
                .width(Length::Fill),
            )
            .push(
                button(&mut self.revert_button, "Revert")
                    .on_press(Message::RevertButtonPress)
                    .style(araiseal_styles::Button::Primary),
            )
            .push(
                button(&mut self.save_button, "Save")
                    .on_press(Message::SaveButtonPress)
                    .style(araiseal_styles::Button::Primary),
            )
            .push(
                button(&mut self.save_all_button, "Save All")
                    .on_press(Message::SaveAllButtonPress)
                    .style(araiseal_styles::Button::Primary),
            )
            .into()
    }
}
