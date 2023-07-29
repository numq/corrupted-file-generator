use iced::Element;
use iced::widget::{Button, button};

pub fn state_button<'a, Message, Content>(
    enabled: bool,
    content: Content,
    message: Message,
) -> Button<'a, Message>
    where
        Content: Into<Element<'a, Message>>,
{
    let mut button = button(content);
    if enabled {
        button = button.on_press(message);
    }
    button
}