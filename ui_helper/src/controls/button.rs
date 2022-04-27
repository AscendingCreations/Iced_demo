use iced::{alignment::Horizontal, button, Button, Text};

pub fn button<'a, Message: Clone>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(Horizontal::Center),
    )
    .padding(12)
    .min_width(100)
}
